use std::{fs::File, io::{BufWriter, Error, Write}, path::Path};

use crate::charts::{BackendColor, BackendCoord, BackendStyle, DrawingBackend, DrawingErrorKind};

fn make_svg_color(color: BackendColor) -> String {
    let (r, g, b) = color.rgb;
    return format!("#{:02X}{:02X}{:02X}", r, g, b);
}

fn make_svg_opacity(color: BackendColor) -> String {
    return format!("{}", color.alpha);
}

enum Target<'a> {
    File(String, &'a Path),
    Buffer(&'a mut String),
}

impl Target<'_> {
    fn get_mut(&mut self) -> &mut String {
        match self {
            Target::File(ref mut buf, _) => buf,
            Target::Buffer(buf) => buf,
        }
    }
}

enum SVGTag {
    Svg,
    Circle,
    Line,
    Polygon,
    Polyline,
    Rectangle,
    Text,
    #[allow(dead_code)]
    Image,
}

impl SVGTag {
    fn to_tag_name(&self) -> &'static str {
        match self {
            SVGTag::Svg => "svg",
            SVGTag::Circle => "circle",
            SVGTag::Line => "line",
            SVGTag::Polyline => "polyline",
            SVGTag::Rectangle => "rect",
            SVGTag::Text => "text",
            SVGTag::Image => "image",
            SVGTag::Polygon => "polygon",
        }
    }
}

pub struct SVGBackend<'a> {
    size: (u32, u32),
    tag_stack: Vec<SVGTag>,
    target: Target<'a>,
    saved: bool,
}

impl<'a> SVGBackend<'a> {
    pub fn with_file_path<T: AsRef<Path> + ?Sized>(path: &'a T, size: (u32, u32)) -> Self {
        let mut ret = Self {
            target: Target::File(String::default(), path.as_ref()),
            size,
            tag_stack: vec![],
            saved: false,
        };

        ret.init_svg_file(size);

        ret
    }

    /// Create a new SVG drawing backend and store the document into a String buffer
    pub fn with_buffer(buf: &'a mut String, size: (u32, u32)) -> Self {
        let mut ret = Self {
            target: Target::Buffer(buf),
            size,
            tag_stack: vec![],
            saved: false,
        };

        ret.init_svg_file(size);

        ret
    }

    fn escape_and_push(buf: &mut String, value: &str) {
        value.chars().for_each(|c| match c {
            '<' => buf.push_str("&lt;"),
            '>' => buf.push_str("&gt;"),
            '&' => buf.push_str("&amp;"),
            '"' => buf.push_str("&quot;"),
            '\'' => buf.push_str("&apos;"),
            other => buf.push(other),
        });
    }

    fn open_tag(&mut self, tag: SVGTag, attr: &[(&str, &str)], close: bool) {
        let buf = self.target.get_mut();
        buf.push('<');
        buf.push_str(tag.to_tag_name());
        for (key, value) in attr {
            buf.push(' ');
            buf.push_str(key);
            buf.push_str("=\"");
            Self::escape_and_push(buf, value);
            buf.push('\"');
        }

        if close {
            buf.push_str("/>\n");
        } else {
            self.tag_stack.push(tag);
            buf.push_str(">\n");
        }
    }

    fn close_tag(&mut self) -> bool {
        if let Some(tag) = self.tag_stack.pop() {
            let buf = self.target.get_mut();
            buf.push_str("</");
            buf.push_str(tag.to_tag_name());
            buf.push_str(">\n");
            return true;
        }
        false
    }

    fn init_svg_file(&mut self, size: (u32, u32)) {
        self.open_tag(
            SVGTag::Svg,
            &[
                ("width", &format!("{}", size.0)),
                ("height", &format!("{}", size.1)),
                ("viewBox", &format!("0 0 {} {}", size.0, size.1)),
                ("xmlns", "http://www.w3.org/2000/svg"),
            ],
            false,
        );
    }
}

impl<'a> DrawingBackend for SVGBackend<'a> {
    type ErrorType = Error;

    fn get_size(&self) -> (u32, u32) {
        self.size
    }

    fn ensure_prepared(&mut self) -> Result<(), crate::charts::DrawingErrorKind<Self::ErrorType>> {
        Ok(())
    }

    fn present(&mut self) -> Result<(), DrawingErrorKind<Error>> {
        if !self.saved {
            while self.close_tag() {}
            match self.target {
                Target::File(ref buf, path) => {
                    let outfile = File::create(path).map_err(DrawingErrorKind::DrawingError)?;
                    let mut outfile = BufWriter::new(outfile);
                    outfile
                        .write_all(buf.as_ref())
                        .map_err(DrawingErrorKind::DrawingError)?;
                }
                Target::Buffer(_) => {}
            }
            self.saved = true;
        }
        Ok(())
    }

    fn draw_line<S: BackendStyle>(
        &mut self,
        from: BackendCoord,
        to: BackendCoord,
        style: &S,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        if style.color().alpha == 0.0 {
            return Ok(());
        }
        self.open_tag(
            SVGTag::Line,
            &[
                ("opacity", &make_svg_opacity(style.color())),
                ("stroke", &make_svg_color(style.color())),
                ("stroke-width", &format!("{}", style.stroke_width())),
                ("x1", &format!("{}", from.0)),
                ("y1", &format!("{}", from.1)),
                ("x2", &format!("{}", to.0)),
                ("y2", &format!("{}", to.1)),
            ],
            true,
        );
        Ok(())
    }

    fn draw_rect<S: BackendStyle>(
        &mut self,
        upper_left: BackendCoord,
        bottom_right: BackendCoord,
        style: &S,
        fill: bool,
    ) -> Result<(), DrawingErrorKind<Self::ErrorType>> {
        if style.color().alpha == 0.0 {
            return Ok(());
        }

        let (fill, stroke) = if !fill {
            ("none".to_string(), make_svg_color(style.color()))
        } else {
            (make_svg_color(style.color()), "none".to_string())
        };

        self.open_tag(
            SVGTag::Rectangle,
            &[
                ("x", &format!("{}", upper_left.0)),
                ("y", &format!("{}", upper_left.1)),
                ("width", &format!("{}", bottom_right.0 - upper_left.0)),
                ("height", &format!("{}", bottom_right.1 - upper_left.1)),
                ("opacity", &make_svg_opacity(style.color())),
                ("fill", &fill),
                ("stroke", &stroke),
            ],
            true,
        );

        Ok(())
    }
}

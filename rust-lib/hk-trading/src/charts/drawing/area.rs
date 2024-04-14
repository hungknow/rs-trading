use crate::charts::{
    coord::{cartesian::Cartesian2d, ranged1d::Ranged, CoordTranslate, Shift},
    elements::{CoordMapper, Drawable, PointCollection},
    style::{Color, SizeDesc},
    DrawingBackend, DrawingErrorKind,
};
use std::{borrow::Borrow, ops::Range};
use std::{cell::RefCell, error::Error, rc::Rc};

/// The representation of the rectangle in backend canvas
#[derive(Clone, Debug)]
pub struct Rect {
    x0: i32,
    y0: i32,
    x1: i32,
    y1: i32,
}

impl Rect {
    /// Make the coordinate in the range of the rectangle
    pub fn truncate(&self, p: (i32, i32)) -> (i32, i32) {
        (p.0.min(self.x1).max(self.x0), p.1.min(self.y1).max(self.y0))
    }

    /// Evenly the rectangle into a grid with arbitrary breaks; return a rect iterator.
    fn split_grid(
        &self,
        x_breaks: impl Iterator<Item = i32>,
        y_breaks: impl Iterator<Item = i32>,
    ) -> impl Iterator<Item = Rect> {
        let mut xs = vec![self.x0, self.x1];
        let mut ys = vec![self.y0, self.y1];
        xs.extend(x_breaks.map(|v| v + self.x0));
        ys.extend(y_breaks.map(|v| v + self.y0));

        xs.sort_unstable();
        ys.sort_unstable();

        let xsegs: Vec<_> = xs
            .iter()
            .zip(xs.iter().skip(1))
            .map(|(a, b)| (*a, *b))
            .collect();

        let ysegs: Vec<_> = ys
            .iter()
            .zip(ys.iter().skip(1))
            .map(|(a, b)| (*a, *b))
            .collect();

        ysegs.into_iter().flat_map(move |(y0, y1)| {
            xsegs
                .clone()
                .into_iter()
                .map(move |(x0, x1)| Self { x0, y0, x1, y1 })
        })
    }
}

/// The abstraction of a drawing area. Plotters uses drawing area as the fundamental abstraction for the
/// high level drawing API. The major functionality provided by the drawing area is
/// 1. Layout specification - Split the parent drawing area into sub-drawing-areas
/// 2. Coordinate Translation - Allows guest coordinate system attached and used for drawing.
/// 3. Element based drawing - drawing area provides the environment the element can be drawn onto it.
pub struct DrawingArea<DB: DrawingBackend, CT: CoordTranslate> {
    pub(crate) backend: Rc<RefCell<DB>>,
    pub(crate) rect: Rect,
    pub(crate) coord: CT,
}

impl<DB: DrawingBackend, CT: CoordTranslate + Clone> Clone for DrawingArea<DB, CT> {
    fn clone(&self) -> Self {
        Self {
            backend: self.backend.clone(),
            rect: self.rect.clone(),
            coord: self.coord.clone(),
        }
    }
}

/// The error description of any drawing area API
#[derive(Debug)]
pub enum DrawingAreaErrorKind<E: Error + Send + Sync> {
    /// The error is due to drawing backend failure
    BackendError(DrawingErrorKind<E>),
    /// We are not able to get the mutable reference of the backend,
    /// which indicates the drawing backend is current used by other
    /// drawing operation
    SharingError,
    /// The error caused by invalid layout
    LayoutError,
}

type DrawingAreaError<T: DrawingBackend> = DrawingAreaErrorKind<T::ErrorType>;

// impl<DB: DrawingBackend, X: Ranged, Y: Ranged> From<DB> for DrawingArea<DB, Cartesian2d<X, Y>> {
//     fn from(backend: DB) -> Self {
//         Self::with_rc_cell(Rc::new(RefCell::new(backend)))
//     }
// }

impl<DB: DrawingBackend> From<DB> for DrawingArea<DB, Shift> {
    fn from(backend: DB) -> Self {
        Self::with_rc_cell(Rc::new(RefCell::new(backend)))
    }
}

impl<'a, DB: DrawingBackend> From<&'a Rc<RefCell<DB>>> for DrawingArea<DB, Shift> {
    fn from(backend: &'a Rc<RefCell<DB>>) -> Self {
        Self::with_rc_cell(backend.clone())
    }
}

/// A type which can be converted into a root drawing area
pub trait IntoDrawingArea<CT: CoordTranslate>: DrawingBackend + Sized {
    /// Convert the type into a root drawing area
    fn into_drawing_area(self) -> DrawingArea<Self, CT>;
}

impl<T: DrawingBackend, CT: CoordTranslate> IntoDrawingArea<CT> for T
where
    DrawingArea<T, CT>: From<T>,
{
    fn into_drawing_area(self) -> DrawingArea<T, CT> {
        self.into()
    }
}

impl<DB: DrawingBackend> DrawingArea<DB, Shift> {
    fn with_rc_cell(backend: Rc<RefCell<DB>>) -> Self {
        let (x1, y1) = RefCell::borrow(backend.borrow()).get_size();
        Self {
            rect: Rect {
                x0: 0,
                y0: 0,
                x1: x1 as i32,
                y1: y1 as i32,
            },
            backend,
            coord: Shift((0, 0)),
        }
    }

    /// Apply a new coord transformation object and returns a new drawing area
    pub fn apply_coord_spec<CT: CoordTranslate>(&self, coord_spec: CT) -> DrawingArea<DB, CT> {
        DrawingArea {
            rect: self.rect.clone(),
            backend: self.backend.clone(),
            coord: coord_spec,
        }
    }

    /// Split the drawing area into a grid with specified breakpoints on both X axis and Y axis
    pub fn split_by_breakpoints<
        XSize: SizeDesc,
        YSize: SizeDesc,
        XS: AsRef<[XSize]>,
        YS: AsRef<[YSize]>,
    >(
        &self,
        xs: XS,
        ys: YS,
    ) -> Vec<Self> {
        self.rect
            .split_grid(
                xs.as_ref().iter().map(|x| x.in_pixels(self)),
                ys.as_ref().iter().map(|x| x.in_pixels(self)),
            )
            .map(|rect| Self {
                rect: rect.clone(),
                backend: self.backend.clone(),
                coord: Shift((rect.x0, rect.y0)),
            })
            .collect()
    }
}

impl<DB: DrawingBackend, X: Ranged, Y: Ranged> DrawingArea<DB, Cartesian2d<X, Y>> {
    /// Draw the mesh on a area
    // pub fn draw_mesh<DrawFunc, YH: KeyPointHint, XH: KeyPointHint>(
    //     &self,
    //     mut draw_func: DrawFunc,
    //     y_count_max: YH,
    //     x_count_max: XH,
    // ) -> Result<(), DrawingAreaErrorKind<DB::ErrorType>>
    // where
    //     DrawFunc: FnMut(&mut DB, MeshLine<X, Y>) -> Result<(), DrawingErrorKind<DB::ErrorType>>,
    // {
    //     self.backend_ops(move |b| {
    //         self.coord
    //             .draw_mesh(y_count_max, x_count_max, |line| draw_func(b, line))
    //     })
    // }

    /// Get the range of X of the guest coordinate for current drawing area
    pub fn get_x_range(&self) -> Range<X::ValueType> {
        self.coord.get_x_range()
    }

    /// Get the range of Y of the guest coordinate for current drawing area
    pub fn get_y_range(&self) -> Range<Y::ValueType> {
        self.coord.get_y_range()
    }

    /// Get the range of X of the backend coordinate for current drawing area
    pub fn get_x_axis_pixel_range(&self) -> Range<i32> {
        self.coord.get_x_axis_pixel_range()
    }

    /// Get the range of Y of the backend coordinate for current drawing area
    pub fn get_y_axis_pixel_range(&self) -> Range<i32> {
        self.coord.get_y_axis_pixel_range()
    }
}

impl<DB: DrawingBackend, CT: CoordTranslate> DrawingArea<DB, CT> {
    pub fn draw_grid() {
        // Clear the canvas
        // Draw grid mesh
        // Sort the overlays by z-index
        // Draw each overlay
        // If overlay isn't displayed, skip
        // Draw the crosshair
    }

    fn backend_ops<R, O: FnOnce(&mut DB) -> Result<R, DrawingErrorKind<DB::ErrorType>>>(
        &self,
        ops: O,
    ) -> Result<R, DrawingAreaError<DB>> {
        if let Ok(mut db) = self.backend.try_borrow_mut() {
            db.ensure_prepared()
                .map_err(DrawingAreaErrorKind::BackendError)?;
            ops(&mut *db).map_err(DrawingAreaErrorKind::BackendError)
        } else {
            return Err(DrawingAreaErrorKind::SharingError);
        }
    }

    // Draw element
    pub fn draw<'a, E, B>(&self, element: &'a E) -> Result<(), DrawingAreaError<DB>>
    where
        B: CoordMapper,
        &'a E: PointCollection<'a, CT::From, B>,
        E: Drawable<DB, B>,
    {
        let backend_coords = element.point_iter().into_iter().map(|p| {
            let b = p.borrow();
            B::map(&self.coord, b, &self.rect)
        });
        self.backend_ops(move |b| element.draw(backend_coords, b, self.dim_in_pixel()))
    }

    /// Get the area dimension in pixel
    pub fn dim_in_pixel(&self) -> (u32, u32) {
        (
            (self.rect.x1 - self.rect.x0) as u32,
            (self.rect.y1 - self.rect.y0) as u32,
        )
    }

    /// Get the pixel range of this area
    pub fn get_pixel_range(&self) -> (Range<i32>, Range<i32>) {
        (self.rect.x0..self.rect.x1, self.rect.y0..self.rect.y1)
    }

    /// Fill the entire drawing area with a color
    pub fn fill<ColorType: Color>(&self, color: &ColorType) -> Result<(), DrawingAreaError<DB>> {
        self.backend_ops(|backend| {
            backend.draw_rect(
                (self.rect.x0, self.rect.y0),
                (self.rect.x1, self.rect.y1),
                &color.to_backend_color(),
                true,
            )
        })
    }

    /// Present all the pending changes to the backend
    pub fn present(&self) -> Result<(), DrawingAreaError<DB>> {
        self.backend_ops(|b| b.present())
    }
}

#[cfg(test)]
mod drawing_area_tests {
    use crate::charts::{
        drawing::backend_impl::create_mocked_drawing_area,
        elements::{Drawable, PointCollection},
        style::{
            colors::{BLACK, BLUE, CYAN, MAGENTA},
            Color, RED, WHITE, YELLOW,
        },
        BackendCoord, DrawingBackend,
    };

    struct MockedElement<X, Y> {
        points: [(X, Y); 4],
    }

    impl<X, Y, DB: DrawingBackend> Drawable<DB> for MockedElement<X, Y> {
        fn draw<I: Iterator<Item = BackendCoord>>(
            &self,
            pos: I,
            backend: &mut DB,
            parent_dim: (u32, u32),
        ) -> Result<(), crate::charts::DrawingErrorKind<<DB as DrawingBackend>::ErrorType>>
        {
            Ok(())
        }
    }

    impl<'a, X: 'a, Y: PartialOrd + 'a> PointCollection<'a, (X, Y)> for &'a MockedElement<X, Y> {
        type Point = &'a (X, Y);
        type IntoIter = &'a [(X, Y)];

        fn point_iter(self) -> Self::IntoIter {
            &self.points
        }
    }

    #[test]
    fn test_draw() {
        let drawing_area = create_mocked_drawing_area(100, 100, |m| {});
        let element = &MockedElement::<i32, i32> {
            points: [(0, 0), (1, 1), (2, 2), (3, 3)],
        };
        drawing_area.draw(element).expect("Drawing Error");
    }

    #[test]
    fn test_split_grid() {
        let colors = [
            &RED, &BLUE, &YELLOW, &WHITE, &BLACK, &MAGENTA, &CYAN, &BLUE, &RED,
        ];
        let breaks: [i32; 5] = [100, 200, 300, 400, 500];
        for nxb in 0..=5 {
            for nyb in 0..=5 {
                let drawing_area = create_mocked_drawing_area(1024, 768, |m| {
                    for row in 0..=nyb {
                        for col in 0..=nxb {
                            let get_bp = |full, limit, id| {
                                (if id == 0 {
                                    0
                                } else if id > limit {
                                    full
                                } else {
                                    breaks[id as usize - 1]
                                })
                            };

                            let expected_u = (get_bp(1024, nxb, col), get_bp(768, nyb, row));
                            let expected_d =
                                (get_bp(1024, nxb, col + 1), get_bp(768, nyb, row + 1));
                            let expected_color =
                                colors[(row * (nxb + 1) + col) as usize % colors.len()];

                            m.check_draw_rect(move |c, _, f, u, d| {
                                assert_eq!(c, expected_color.to_rgba());
                                assert!(f);
                                assert_eq!(u, expected_u);
                                assert_eq!(d, expected_d);
                            });
                        }
                    }
                });
                let result = drawing_area
                    .split_by_breakpoints(&breaks[0..nxb as usize], &breaks[0..nyb as usize]);
                for i in 0..result.len() {
                    result[i]
                        .fill(colors[i % colors.len()])
                        .expect("Drawing Error");
                }
            }
        }
    }
}

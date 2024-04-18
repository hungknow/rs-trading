use crate::charts::{
    coord::{cartesian::Cartesian2d, ranged1d::{Ranged, ValueFormatter}},
    ChartContext, DrawingBackend, MeshStyle,
};

mod draw_impl;

impl<'a, DB, XT, YT, X, Y> ChartContext<'a, DB, Cartesian2d<X, Y>>
where
    DB: DrawingBackend,
    X: Ranged<ValueType = XT> + ValueFormatter<XT>,
    Y: Ranged<ValueType = YT> + ValueFormatter<YT>,
{
    /// Initialize a mesh configuration object and mesh drawing can be finalized by calling
    /// the function `MeshStyle::draw`.
    pub fn configure_mesh(&mut self) -> MeshStyle<'a, '_, X, Y, DB> {
        MeshStyle::new(self)
    }
}

impl<'a, DB: DrawingBackend, X: Ranged, Y: Ranged> ChartContext<'a, DB, Cartesian2d<X, Y>> {}

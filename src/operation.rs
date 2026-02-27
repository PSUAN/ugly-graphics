use crate::painter::Painter;

pub mod pixel;
pub mod scanline;
pub mod stamp;

pub trait Operation<P> {
    type Output;
    fn draw_on(self, painter: &mut Painter<'_, P>) -> Self::Output;
}

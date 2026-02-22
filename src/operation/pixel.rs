use crate::painter::Painter;
use crate::strategy::Strategy;

use super::Operation;

pub struct Pixel<'a, P> {
    position: (i32, i32),
    value: Strategy<'a, P>,
}

impl<'a, P> Pixel<'a, P> {
    pub fn new<I>(position: I, value: Strategy<'a, P>) -> Self
    where
        I: Into<(i32, i32)>,
    {
        let position = position.into();
        Self { position, value }
    }
}

impl<'a, P> Operation<P> for Pixel<'a, P> {
    type Output = ();

    fn draw_on(self, painter: &mut Painter<'_, P>) -> Option<Self::Output> {
        painter.pixel(self.position, self.value);
        Some(())
    }
}

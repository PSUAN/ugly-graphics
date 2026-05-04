//! [`Compute`] operation applied to the entire painter.

use crate::operation::Operation;
use crate::painter::Painter;
use crate::strategy::apply;

/// An operation to be applied to every pixel of a painter.
#[derive(Clone, Copy)]
pub struct Compute<'a, P> {
    action: &'a dyn Fn((i32, i32), P) -> P,
}

impl<'a, P> Compute<'a, P> {
    /// Create a new instance using provided `action`.
    pub fn new(action: &'a dyn Fn((i32, i32), P) -> P) -> Self {
        Self { action }
    }
}

impl<'a, P> Operation<P> for Compute<'a, P>
where
    P: Clone,
{
    type Output = ();

    fn draw_on(self, painter: &mut Painter<'_, P>) -> Self::Output {
        let ((start_x, start_y), (width, height)) = painter.draw_zone();
        for y in start_x..height as i32 {
            for x in start_y..width as i32 {
                painter.pixel((x as _, y as _), &apply(&|v| (self.action)((x, y), v)));
            }
        }
    }
}

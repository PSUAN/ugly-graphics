use crate::operation::Operation;
use crate::painter::Painter;
use crate::strategy::IntoApply;

pub struct Compute<'a, P> {
    action: &'a dyn Fn((u32, u32), P) -> P,
}

impl<'a, P> Compute<'a, P> {
    pub fn new(action: &'a dyn Fn((u32, u32), P) -> P) -> Self {
        Self { action }
    }
}

impl<'a, P> Operation<P> for Compute<'a, P>
where
    P: Clone,
{
    type Output = ();

    fn draw_on(self, painter: &mut Painter<'_, P>) -> Self::Output {
        let (width, height) = painter.dimensions();
        for y in 0..height {
            for x in 0..width {
                painter.pixel((x as _, y as _), &(|v| (self.action)((x, y), v)).apply());
            }
        }
    }
}

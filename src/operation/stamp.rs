//! The [`Stamp`] applies an [`Image`] using provided [`Action`].

use crate::image::Image;
use crate::operation::Operation;
use crate::painter::Painter;
use crate::strategy::apply;

/// Action that computes the resulting pixel value `P` given the original `P`
/// and provided `S` values.
pub type Action<'a, P, S> = &'a dyn Fn(P, S) -> P;

/// The stamping operation to apply an [`Image`] over some [`Painter`].
#[derive(Clone, Copy)]
pub struct Stamp<'a, P, S> {
    position: (i32, i32),
    stamp: &'a dyn Image<Pixel = S>,
    action: Action<'a, P, S>,
}

impl<'a, P, S> Stamp<'a, P, S> {
    /// Create new instance for stamping provided `stamp` over the [`Painter`].
    pub fn new(
        position: (i32, i32),
        stamp: &'a dyn Image<Pixel = S>,
        action: Action<'a, P, S>,
    ) -> Self {
        Self {
            position,
            stamp,
            action,
        }
    }
}

impl<'a, P, S> Operation<P> for Stamp<'a, P, S>
where
    P: Clone,
    S: Clone,
{
    type Output = ();

    fn draw_on(self, painter: &mut Painter<'_, P>) -> Self::Output {
        let (x, y) = self.position;
        let (width, height) = self.stamp.dimensions();

        for stamp_y in 0..height {
            let target_y = y + stamp_y as i32;
            for stamp_x in 0..width {
                if let Some(stamp_pixel) = self.stamp.pixel((stamp_x, stamp_y)) {
                    let target_x = x + stamp_x as i32;

                    let strategy =
                        move |passed_pixel| (self.action)(passed_pixel, stamp_pixel.clone());
                    painter.pixel((target_x, target_y), &apply(&strategy));
                }
            }
        }
    }
}

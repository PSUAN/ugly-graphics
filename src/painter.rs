//! [`Painter`] is a handle to the stored [`ImageMut`].
//!
//! It provides basic API for pixel modification.

use core::ops::Range;

use crate::image::ImageMut;
use crate::operation::Operation;
use crate::strategy::Strategy;

/// An [`ImageMut`] wrapper.
pub struct Painter<'a, P> {
    target: &'a mut dyn ImageMut<Pixel = P>,
}

impl<'a, P> Painter<'a, P> {
    /// Create new [`Painter`] instance.
    pub fn new(target: &'a mut dyn ImageMut<Pixel = P>) -> Self {
        Self { target }
    }
}

impl<'a, P> Painter<'a, P> {
    /// Get dimensions of the internal [`ImageMut`].
    pub fn dimensions(&self) -> (u32, u32) {
        self.target.dimensions()
    }
}

impl<'a, P> Painter<'a, P>
where
    P: Clone,
{
    /// Draw the provided `operation` on this [`Painter`] instance.
    pub fn draw<O>(&mut self, operation: O) -> O::Output
    where
        O: Operation<P>,
    {
        operation.draw_on(self)
    }

    /// Apply the provided `strategy` on the `(x, y)` positions.
    ///
    /// Fails silently.
    pub fn pixel(&mut self, (x, y): (i32, i32), strategy: &Strategy<P>) {
        if let Ok(x) = x.try_into()
            && let Ok(y) = y.try_into()
        {
            match strategy {
                Strategy::Overwrite(value) => self.target.set_pixel((x, y), value.clone()),
                Strategy::Apply(function) => self.target.modify_pixel((x, y), function),
            }
        }
    }

    /// Apply the provided `strategy` on the range `x` at horizontal position
    /// `y`.
    ///
    /// Fails silently.
    pub fn horizontal_line(&mut self, x: Range<i32>, y: i32, strategy: &Strategy<P>) {
        if let Ok(y) = y.try_into() {
            if x.end < 0 {
                return;
            }
            let x = if x.start < 0 { 0..x.end } else { x };
            let total = (x.end - x.start) as u32;
            let x = x.start as u32;

            match strategy {
                Strategy::Overwrite(value) => {
                    self.target
                        .set_horizontal_line((x, y), total, value.clone())
                }
                Strategy::Apply(function) => {
                    self.target.modify_horizontal_line((x, y), total, function)
                }
            }
        }
    }
}

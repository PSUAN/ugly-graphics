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
    offset: (i32, i32),
}

impl<'a, P> Painter<'a, P> {
    /// Create new [`Painter`] instance.
    pub fn new(target: &'a mut dyn ImageMut<Pixel = P>) -> Self {
        let offset = (0, 0);
        Self { target, offset }
    }

    /// Build new [`Painter`] with provided `offset` value.
    pub fn with_offset(self, offset: (i32, i32)) -> Self {
        Self { offset, ..self }
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
        let mut region = DrawRegion { painter: self };
        operation.draw_on(&mut region)
    }
}

/// A region to perform drawing operations on.
pub struct DrawRegion<'p, 't, P> {
    painter: &'p mut Painter<'t, P>,
}

impl<'p, 't, P> DrawRegion<'p, 't, P> {
    /// Get draw zone origin and dimensions.
    pub fn draw_zone(&self) -> ((i32, i32), (u32, u32)) {
        let (offset_x, offset_y) = self.painter.offset;
        ((-offset_x, -offset_y), self.painter.target.dimensions())
    }

    /// Apply the provided `strategy` on the `(x, y)` positions.
    ///
    /// Fails silently.
    pub fn pixel(&mut self, (x, y): (i32, i32), strategy: &Strategy<P>)
    where
        P: Clone,
    {
        let (offset_x, offset_y) = self.painter.offset;
        let (x, y) = (x + offset_x, y + offset_y);

        if let Ok(x) = x.try_into()
            && let Ok(y) = y.try_into()
        {
            match strategy {
                Strategy::Overwrite(value) => self.painter.target.set_pixel((x, y), value.clone()),
                Strategy::Apply(function) => self.painter.target.modify_pixel((x, y), function),
            }
        }
    }

    /// Apply the provided `strategy` on the range `x` at horizontal position
    /// `y`.
    ///
    /// Fails silently.
    pub fn horizontal_line(&mut self, x: Range<i32>, y: i32, strategy: &Strategy<P>)
    where
        P: Clone,
    {
        let (offset_x, offset_y) = self.painter.offset;
        let (x, y) = ((x.start + offset_x)..(x.end + offset_x), y + offset_y);

        if let Ok(y) = y.try_into() {
            if x.end < 0 {
                return;
            }
            let x = if x.start < 0 { 0..x.end } else { x };
            let total = (x.end - x.start) as u32;
            let x = x.start as u32;

            match strategy {
                Strategy::Overwrite(value) => {
                    self.painter
                        .target
                        .set_horizontal_line((x, y), total, value.clone())
                }
                Strategy::Apply(function) => {
                    self.painter
                        .target
                        .modify_horizontal_line((x, y), total, function)
                }
            }
        }
    }
}

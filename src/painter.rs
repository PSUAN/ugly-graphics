//! [`Painter`] is a handle to the stored [`ImageWrite`] or [`ImageModify`].
//!
//! It provides basic API for pixel modification.

use core::ops::Range;

use crate::image::{Dimensions, ImageMut};
use crate::operation::Operation;

/// An [`ImageWrite`] or [`ImageModify`] wrapper.
pub struct Painter<T> {
    target: T,
    offset: (i32, i32),
}

impl<T> Painter<T> {
    /// Create new [`Painter`] instance.
    pub fn new(target: T) -> Self {
        let offset = (0, 0);
        Self { target, offset }
    }

    /// Build new [`Painter`] with provided `offset` value.
    pub fn with_offset(self, offset: (i32, i32)) -> Self {
        Self { offset, ..self }
    }
}

impl<T> Painter<T> {
    /// Draw the provided `operation` on this [`Painter`] instance.
    pub fn draw<O>(&mut self, operation: O) -> O::Output
    where
        O: Operation<T>,
    {
        let mut region = DrawRegion { painter: self };
        operation.draw_on(&mut region)
    }
}

/// A region to perform drawing operations on.
pub struct DrawRegion<'p, T> {
    painter: &'p mut Painter<T>,
}

impl<'p, T> DrawRegion<'p, T> {
    /// Get draw zone origin and dimensions.
    pub fn draw_zone(&self) -> ((i32, i32), (u32, u32))
    where
        T: Dimensions,
    {
        let (offset_x, offset_y) = self.painter.offset;
        ((-offset_x, -offset_y), self.painter.target.dimensions())
    }

    /// Apply the provided `writer` on the `(x, y)` positions.
    ///
    /// Fails silently.
    pub fn pixel<W>(&mut self, (x, y): (i32, i32), writer: &W)
    where
        T: ImageMut<W>,
    {
        let (offset_x, offset_y) = self.painter.offset;
        let (x, y) = (x + offset_x, y + offset_y);

        if let Ok(x) = x.try_into()
            && let Ok(y) = y.try_into()
        {
            self.painter.target.write_pixel((x, y), writer);
        }
    }

    /// Apply the provided `writer` on the range `x` at horizontal position
    /// `y`.
    ///
    /// Fails silently.
    pub fn horizontal_line<W>(&mut self, x: Range<i32>, y: i32, writer: &W)
    where
        T: ImageMut<W>,
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

            self.painter
                .target
                .write_horizontal_line((x, y), total, writer);
        }
    }
}

//! Line drawing using scanlines.

use core::ops::Range;

use crate::operation::Operation;
use crate::painter::DrawRegion;
use crate::strategy::Strategy;
use crate::utility;

/// Line primitive.
#[derive(Clone, Copy)]
pub struct Line<'a, P> {
    from: (i32, i32),
    to: (i32, i32),
    value: Strategy<'a, P>,
}

impl<'a, P> Line<'a, P> {
    /// Create a new straight line from given positions and provided `value`.
    pub fn new(from: (i32, i32), to: (i32, i32), value: Strategy<'a, P>) -> Self {
        Self { from, to, value }
    }
}

fn extend_range_to(range: Range<i32>, value: i32) -> Range<i32> {
    if value < range.start {
        value..range.end
    } else if value > range.end {
        range.start..value
    } else {
        range
    }
}

fn vertical_range_in_dimensions(
    start: (i32, i32),
    end: (i32, i32),
    (region_origin, region_size): ((i32, i32), (u32, u32)),
) -> Option<Range<i32>> {
    let (start, end) = utility::swap_if(start.1 > end.1, (start, end));

    // Early return if the lower point is too hight.
    if end.1 < region_origin.1 {
        return None;
    }
    // Early return if the higher point is too low.
    if start.1 >= region_origin.1 + region_size.1 as i32 {
        return None;
    }

    // Early check if we have a vertical line.
    if start.0 == end.0 {
        // Too much to the left.
        if start.0 < region_origin.0 {
            return None;
        }
        // Too much to the right.
        if start.0 >= region_origin.0 + region_size.0 as i32 {
            return None;
        }
        // Clamp to dimensions.
        return Some(super::clamp_range(
            start.1..end.1 + 1,
            region_origin.1,
            region_origin.1 + region_size.1 as i32,
        ));
    }

    // Now we know that our line:
    // - is not vertical;
    // - is at least partly in vertical bounds.
    let left = super::segment_scan(utility::swap(start), utility::swap(end), region_origin.0);
    let right = super::segment_scan(
        utility::swap(start),
        utility::swap(end),
        region_origin.0 + region_size.0 as i32,
    );
    let range = match (left, right) {
        (None, None) => {
            // Early return if the segment is outside.
            if start.0.min(end.0) < region_origin.0
                || start.0.max(end.0) >= region_origin.0 + region_size.0 as i32
            {
                return None;
            }
            // There are no intersections and the segment is inside.
            start.1..end.1 + 1
        }
        (None, Some(right)) => {
            if start.0 < end.0 {
                extend_range_to(right, start.1)
            } else {
                extend_range_to(right, end.1 + 1)
            }
        }
        (Some(left), None) => {
            if start.0 < end.0 {
                extend_range_to(left, end.1 + 1)
            } else {
                extend_range_to(left, start.1)
            }
        }
        (Some(left), Some(right)) => super::merge_ranges(left, right),
    };

    Some(super::clamp_range(
        range,
        region_origin.1,
        region_origin.1 + region_size.1 as i32,
    ))
}

impl<'a, P> Operation<P> for Line<'a, P>
where
    P: Clone,
{
    type Output = ();

    fn draw_on(self, painter: &mut DrawRegion<'_, '_, P>) -> Self::Output {
        let zone = painter.draw_zone();
        if let Some(scan) = vertical_range_in_dimensions(self.from, self.to, zone) {
            for scanline in scan {
                if let Some(range) = super::segment_scan(self.from, self.to, scanline) {
                    painter.horizontal_line(range, scanline, &self.value);
                }
            }
        }
    }
}

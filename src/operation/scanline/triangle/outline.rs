//! An outline triangle is drawn by connecting three vertices.

use core::ops::Range;

use crate::operation::{Operation, scanline};
use crate::painter::DrawRegion;
use crate::strategy::Strategy;
use crate::utility;

fn range_overlap_solver(first: Range<i32>, second: Range<i32>) -> [Range<i32>; 2] {
    let (left, right) = utility::swap_if(first.start >= second.start, (first, second));

    if left.end >= right.start {
        [left.start..left.end.max(right.end), 0..0]
    } else {
        [left, right]
    }
}

/// Outline triangle drawing operation.
///
/// Intended to not draw the same pixel more than once.
#[derive(Clone, Copy)]
pub struct OutlineTriangle<'a, P> {
    vertices: [(i32, i32); 3],
    value: Strategy<'a, P>,
}

impl<'a, P> OutlineTriangle<'a, P> {
    /// Create a new instance to draw using the provided `value`.
    pub fn new(vertices: [(i32, i32); 3], value: Strategy<'a, P>) -> Self {
        Self { vertices, value }
    }
}

impl<P> Operation<P> for OutlineTriangle<'_, P>
where
    P: Clone,
{
    type Output = ();

    fn draw_on(self, painter: &mut DrawRegion<'_, '_, P>) -> Self::Output {
        let ((_, origin_y), (_, height)) = painter.draw_zone();

        let (_, bounding_y) = scanline::estimate_bounding_box(&self.vertices).unwrap_or_default();
        let bounding_y = scanline::clamp_range(bounding_y, origin_y, height as i32);

        // Sort vertices by the y value.
        let [vertex_a, vertex_b, vertex_c] = self.vertices;
        let (vertex_a, vertex_b) = utility::swap_if(vertex_a.1 > vertex_b.1, (vertex_a, vertex_b));
        let (vertex_b, vertex_c) = utility::swap_if(vertex_b.1 > vertex_c.1, (vertex_b, vertex_c));
        let (vertex_a, vertex_b) = utility::swap_if(vertex_a.1 > vertex_b.1, (vertex_a, vertex_b));

        if vertex_a.1 == vertex_c.1 {
            let start = vertex_a.0.min(vertex_b.0).min(vertex_c.0);
            let end = vertex_a.0.max(vertex_b.0).max(vertex_c.0);
            let range = start..end;
            painter.horizontal_line(range, vertex_a.1, &self.value);
            return;
        }

        // Iterate from the top point to the middle point.
        let scan = scanline::clamp_range(vertex_a.1..vertex_b.1, bounding_y.start, bounding_y.end);
        for y in scan {
            if let Some(first) = scanline::segment_scan(vertex_a, vertex_b, y)
                && let Some(second) = scanline::segment_scan(vertex_a, vertex_c, y)
            {
                let scans = range_overlap_solver(first, second);
                for scan in scans {
                    painter.horizontal_line(scan, y, &self.value);
                }
            }
        }

        // Special case on the middle point.
        if let Some(first) = scanline::segment_scan(vertex_a, vertex_c, vertex_b.1)
            && let Some(second) = scanline::segment_scan(vertex_a, vertex_b, vertex_b.1)
            && let Some(third) = scanline::segment_scan(vertex_b, vertex_c, vertex_b.1)
        {
            let near_b = scanline::merge_ranges(second, third);
            let scans = range_overlap_solver(first, near_b);
            for scan in scans {
                painter.horizontal_line(scan, vertex_b.1, &self.value);
            }
        }

        // Iterate from the middle point to the end.
        let scan = scanline::clamp_range(
            (vertex_b.1 + 1)..(vertex_c.1 + 1),
            bounding_y.start,
            bounding_y.end + 1,
        );
        for y in scan {
            if let Some(first) = scanline::segment_scan(vertex_b, vertex_c, y)
                && let Some(second) = scanline::segment_scan(vertex_a, vertex_c, y)
            {
                let scans = range_overlap_solver(first, second);
                for scan in scans {
                    painter.horizontal_line(scan, y, &self.value);
                }
            }
        }
    }
}

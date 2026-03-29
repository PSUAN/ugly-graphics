use crate::operation::{Operation, scanline};
use crate::painter::Painter;
use crate::strategy::Strategy;
use crate::utility;

pub struct TilingTriangle<'a, P> {
    vertices: [(i32, i32); 3],
    value: Strategy<'a, P>,
}

impl<'a, P> TilingTriangle<'a, P> {
    pub fn new(vertices: [(i32, i32); 3], value: Strategy<'a, P>) -> Self {
        Self { vertices, value }
    }
}

impl<P> Operation<P> for TilingTriangle<'_, P>
where
    P: Clone,
{
    type Output = ();

    fn draw_on(self, painter: &mut Painter<'_, P>) -> Self::Output {
        let (_, height) = painter.dimensions();

        let (_, bounding_y) = scanline::estimate_bounding_box(&self.vertices).unwrap_or_default();
        let bounding_y = scanline::clamp_range(bounding_y, 0, height as i32);

        // Sort vertices by the y value.
        let [vertex_a, vertex_b, vertex_c] = self.vertices;
        let (vertex_a, vertex_b) = utility::swap_if(vertex_a.1 > vertex_b.1, (vertex_a, vertex_b));
        let (vertex_b, vertex_c) = utility::swap_if(vertex_b.1 > vertex_c.1, (vertex_b, vertex_c));
        let (vertex_a, vertex_b) = utility::swap_if(vertex_a.1 > vertex_b.1, (vertex_a, vertex_b));

        // We are on a horizontal line.
        if vertex_a.1 == vertex_c.1 {
            let start = vertex_a.0.min(vertex_b.0).min(vertex_c.0);
            let end = vertex_a.0.max(vertex_b.0).max(vertex_c.0);
            let range = start..end;
            painter.horizontal_line(range, vertex_a.1, &self.value);
            return;
        }

        // Iterate from the top point to the middle point.
        let scan = scanline::clamp_range(
            vertex_a.1..(vertex_b.1 + 1),
            bounding_y.start,
            bounding_y.end,
        );
        for y in scan {
            if let Some(first) = scanline::line_scan(vertex_a, vertex_b, y)
                && let Some(second) = scanline::line_scan(vertex_a, vertex_c, y)
            {
                let left = if first.start < second.start {
                    first.end
                } else {
                    second.end
                };
                let scan = scanline::merge_ranges(first, second);
                let scan = scanline::cut_from_left(scan, left);
                painter.horizontal_line(scan, y, &self.value);
            }
        }

        // Iterate from the middle point to the end.
        let scan = scanline::clamp_range(
            (vertex_b.1 + 1)..(vertex_c.1 + 1),
            bounding_y.start,
            bounding_y.end + 1,
        );
        for y in scan {
            if let Some(first) = scanline::line_scan(vertex_b, vertex_c, y)
                && let Some(second) = scanline::line_scan(vertex_a, vertex_c, y)
            {
                let left = if first.start < second.start {
                    first.end
                } else {
                    second.end
                };
                let scan = scanline::merge_ranges(first, second);
                let scan = scanline::cut_from_left(scan, left);
                painter.horizontal_line(scan, y, &self.value);
            }
        }
    }
}

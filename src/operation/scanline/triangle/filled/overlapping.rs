use crate::operation::{Operation, scanline};
use crate::painter::Painter;
use crate::strategy::Strategy;
use crate::utility;

pub struct OverlappingTriangle<'a, P> {
    vertices: [(i32, i32); 3],
    value: Strategy<'a, P>,
}

impl<'a, P> OverlappingTriangle<'a, P> {
    pub fn new(vertices: [(i32, i32); 3], value: Strategy<'a, P>) -> Self {
        Self { vertices, value }
    }
}

impl<'a, P> Operation<P> for OverlappingTriangle<'a, P>
where
    P: Clone,
{
    type Output = ();

    fn draw_on(self, painter: &mut Painter<'_, P>) -> Self::Output {
        let (_, height) = painter.dimensions();

        let (_, bounding_y) = scanline::estimate_bounding_box(&self.vertices).unwrap_or_default();
        let bounding_y = scanline::clamp_scan(bounding_y, height);

        // Sort vertices by the y value.
        let [vertex_a, vertex_b, vertex_c] = self.vertices;
        let (vertex_a, vertex_b) = utility::swap_if((vertex_a, vertex_b), vertex_a.1 > vertex_b.1);
        let (vertex_b, vertex_c) = utility::swap_if((vertex_b, vertex_c), vertex_b.1 > vertex_c.1);
        let (vertex_a, vertex_b) = utility::swap_if((vertex_a, vertex_b), vertex_a.1 > vertex_b.1);

        // We are on a horizontal line.
        if vertex_a.1 == vertex_c.1 {
            let start = vertex_a.0.min(vertex_b.0).min(vertex_c.0);
            let end = vertex_a.0.max(vertex_b.0).max(vertex_c.0);
            let (start, total) = scanline::as_start_and_total(start, end);
            painter.horizontal_line((start, vertex_a.1), total, &self.value);
            return;
        }

        // Iterate from the top point to the middle point.
        let (start, total) = scanline::as_start_and_total(vertex_a.1, vertex_b.1);
        let scan = scanline::clamp_scan_to_scan((start, total).into(), bounding_y);
        for y in scan {
            if let Some(first) = scanline::line_scan(vertex_a, vertex_b, y)
                && let Some(second) = scanline::line_scan(vertex_a, vertex_c, y)
            {
                let scan = scanline::merge_scans(first, second);
                painter.horizontal_line((scan.start, y), scan.length, &self.value);
            }
        }

        // Iterate from the middle point to the end.
        let (start, total) = scanline::as_start_and_total(vertex_b.1 + 1, vertex_c.1);
        let scan = scanline::clamp_scan_to_scan((start, total).into(), bounding_y);
        for y in scan {
            if let Some(first) = scanline::line_scan(vertex_b, vertex_c, y)
                && let Some(second) = scanline::line_scan(vertex_a, vertex_c, y)
            {
                let scan = scanline::merge_scans(first, second);
                painter.horizontal_line((scan.start, y), scan.length, &self.value);
            }
        }
    }
}

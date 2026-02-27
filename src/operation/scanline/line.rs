use crate::operation::Operation;
use crate::painter::Painter;
use crate::strategy::Strategy;
use crate::utility;

pub struct Line<'a, P> {
    from: (i32, i32),
    to: (i32, i32),
    value: Strategy<'a, P>,
}

impl<'a, P> Line<'a, P> {
    pub fn new(from: (i32, i32), to: (i32, i32), value: Strategy<'a, P>) -> Self {
        Self { from, to, value }
    }
}

fn merge_scans(first: (i32, u32), second: (i32, u32)) -> (i32, u32) {
    let start = first.0.min(second.0);
    let end = (first.0 + first.1 as i32).max(second.0 + second.1 as i32);
    (start, (end - start) as u32)
}

fn clamp_scan((start, total): (i32, u32), end: u32) -> (i32, u32) {
    let (start, total) = if start < 0 {
        (0, total - (-start) as u32)
    } else {
        (start, total)
    };
    if start + total as i32 >= end as i32 {
        (start, (end - start as u32))
    } else {
        (start, total)
    }
}

fn merge_scan_and_value((start, total): (i32, u32), value: i32) -> (i32, u32) {
    if value < start {
        (value, total + (start - value) as u32)
    } else if value > start + total as i32 {
        (start, (value - start + 1) as u32)
    } else {
        (start, total)
    }
}

fn scan_in_dimensions(
    start: (i32, i32),
    end: (i32, i32),
    dimensions: (u32, u32),
) -> Option<(i32, u32)> {
    let (start, end) = utility::swap_if((start, end), start.1 > end.1);

    // Early return if the lower point is too hight.
    if end.1 < 0 {
        return None;
    }
    // Early return if the lower point is too low.
    if start.1 >= dimensions.1 as i32 {
        return None;
    }

    // Early check if we have a vertical line.
    if start.0 == end.0 {
        // Too much to the left.
        if start.0 < 0 {
            return None;
        }
        // Too much to the right.
        if start.0 >= dimensions.0 as i32 {
            return None;
        }
        // Clamp to dimensisons.
        return Some(clamp_scan(
            (start.1, (end.1 - start.1 + 1) as u32),
            dimensions.1,
        ));
    }

    // Now we know that our line:
    // - is not vertical;
    // - is at least partly in vertical bounds.
    let left = super::scanline(utility::swap(start), utility::swap(end), 0);
    let right = super::scanline(
        utility::swap(start),
        utility::swap(end),
        dimensions.0 as i32 - 1,
    );
    let range = match (left, right) {
        (None, None) => {
            // Early return if the segment is outside.
            if start.0.min(end.0) < 0 || start.0.max(end.0) >= dimensions.0 as i32 {
                return None;
            }
            // There are on intersections and the segment is inside.
            (start.1, (end.1 - start.1 + 1) as u32)
        }
        (None, Some(right)) => {
            if start.0 < end.0 {
                merge_scan_and_value(right, start.1)
            } else {
                merge_scan_and_value(right, end.1)
            }
        }
        (Some(left), None) => {
            if start.0 < end.0 {
                merge_scan_and_value(left, end.1)
            } else {
                merge_scan_and_value(left, start.1)
            }
        }
        (Some(left), Some(right)) => merge_scans(left, right),
    };

    Some(clamp_scan(range, dimensions.1))
}

impl<'a, P> Operation<P> for Line<'a, P>
where
    P: Clone,
{
    type Output = ();

    fn draw_on(self, painter: &mut Painter<'_, P>) -> Self::Output {
        let dimensions = painter.dimensions();
        if let Some((y, total)) = scan_in_dimensions(self.from, self.to, dimensions) {
            for scanline in y..=(y + total as i32) {
                if let Some((x, total)) = super::scanline(self.from, self.to, scanline) {
                    painter.horizontal_line((x, scanline), total, &self.value);
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bounds_computation_is_proper() {
        assert_eq!(scan_in_dimensions((6, 1), (8, 3), (4, 4)), None);
        assert_eq!(scan_in_dimensions((1, 1), (3, 3), (8, 8)), Some((1, 3)));
        assert_eq!(scan_in_dimensions((1, 3), (3, 1), (8, 8)), Some((1, 3)));
        assert_eq!(scan_in_dimensions((-2, 1), (3, 6), (16, 16)), Some((3, 4)));
        assert_eq!(scan_in_dimensions((0, 0), (4, 16), (8, 8)), Some((0, 8)));
        assert_eq!(scan_in_dimensions((0, -4), (0, 16), (8, 8)), Some((0, 8)));
        assert_eq!(scan_in_dimensions((0, -4), (8, 16), (8, 8)), Some((0, 8)));
        assert_eq!(scan_in_dimensions((0, 4), (4, 4), (8, 8)), Some((4, 1)));
        assert_eq!(scan_in_dimensions((0, 4), (4, 4), (2, 2)), None);
        assert_eq!(scan_in_dimensions((0, 8), (4, 8), (4, 4)), None);
        assert_eq!(scan_in_dimensions((0, 8), (4, 16), (4, 4)), None);
    }
}

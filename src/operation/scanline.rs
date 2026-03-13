use core::ops::Range;

use crate::utility::{self, swap_if};

pub mod line;
pub mod triangle;

fn merge_ranges(first: Range<i32>, second: Range<i32>) -> Range<i32> {
    let start = first.start.min(second.start);
    let end = first.end.max(second.end);
    start..end
}

fn estimate_bounding_box(vertices: &[(i32, i32)]) -> Option<(Range<i32>, Range<i32>)> {
    let x = vertices.iter().map(|(x, _)| x);
    let min_x = *x.clone().min()?;
    let max_x = *x.max()?;

    let y = vertices.iter().map(|(_, y)| y);
    let min_y = *y.clone().min()?;
    let max_y = *y.max()?;

    Some((min_x..max_x, min_y..max_y))
}

pub fn clamp_range(range: Range<i32>, start: i32, end: i32) -> Range<i32> {
    let start = range.start.max(start);
    let end = range.end.min(end);
    start..end
}

pub fn line_scan(start: (i32, i32), end: (i32, i32), scan: i32) -> Option<Range<i32>> {
    // Sort to make the line go from "top" to "bottom".
    let (start, end) = utility::swap_if((start, end), start.1 > end.1);

    // Early return if scan is outside of line bounds.
    if scan < start.1 || scan > end.1 {
        return None;
    }

    let (delta_x, delta_y) = (end.0 - start.0, end.1 - start.1);

    // Early return if line is horizontal.
    if delta_y == 0 {
        let (start, end) = utility::swap_if((start, end), start.0 > end.0);
        return Some(start.0..end.0 + 1);
    }

    // Check if the line is steep (there are no cases of pixels touching horizontally).
    let steep = delta_x.abs() <= delta_y;
    let x_points = delta_x + delta_x.signum();
    let y_points = delta_y + 1;
    if steep {
        let x = start.0 + x_points * (scan - start.1) / y_points;
        Some(x..x + 1)
    } else {
        let first = start.0 + (scan - start.1) * x_points / y_points;
        let second = start.0 + (scan - start.1 + 1) * x_points / y_points - delta_x.signum();

        let (first, second) = swap_if((first, second), first > second);

        Some(first..second + 1)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn line_scan_works() {
        let (start, end) = ((0, 0), (5, 2));
        assert_eq!(line_scan(start, end, -1), None);
        assert_eq!(line_scan(start, end, 0), Some(0..2));
        assert_eq!(line_scan(start, end, 1), Some(2..4));
        assert_eq!(line_scan(start, end, 2), Some(4..6));
        assert_eq!(line_scan(start, end, 3), None);

        let (start, end) = ((1, 2), (3, 0));
        assert_eq!(line_scan(start, end, 0), Some(3..4));
        assert_eq!(line_scan(start, end, 1), Some(2..3));
        assert_eq!(line_scan(start, end, 2), Some(1..2));
        assert_eq!(line_scan(start, end, 3), None);

        let (start, end) = ((-1, 4), (3, 4));
        assert_eq!(line_scan(start, end, 3), None);
        assert_eq!(line_scan(start, end, 4), Some(-1..4));
        assert_eq!(line_scan(start, end, 5), None);
    }
}

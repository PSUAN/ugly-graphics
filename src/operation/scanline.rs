use core::ops::Range;

use crate::utility;

pub mod line;
pub mod triangle;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Scan {
    start: i32,
    length: u32,
}

impl Scan {
    pub fn start(self) -> i32 {
        self.start
    }

    pub fn length(self) -> u32 {
        self.length
    }
}

impl IntoIterator for Scan {
    type Item = i32;

    type IntoIter = Range<i32>;

    fn into_iter(self) -> Self::IntoIter {
        self.into()
    }
}

impl From<Scan> for Range<i32> {
    fn from(value: Scan) -> Self {
        value.start..value.start + value.length as i32
    }
}

impl From<(i32, u32)> for Scan {
    fn from((start, length): (i32, u32)) -> Self {
        Self { start, length }
    }
}

impl From<Scan> for (i32, u32) {
    fn from(value: Scan) -> Self {
        (value.start, value.length)
    }
}

fn merge_scans(first: Scan, second: Scan) -> Scan {
    let start = first.start.min(second.start);
    let end = (first.start + first.length as i32).max(second.start + second.length as i32);
    (start, (end - start) as u32).into()
}

fn estimate_bounding_box(vertices: &[(i32, i32)]) -> Option<(Scan, Scan)> {
    let x = vertices.iter().map(|(x, _)| x);
    let min_x = *x.clone().min()?;
    let max_x = *x.max()?;

    let y = vertices.iter().map(|(_, y)| y);
    let min_y = *y.clone().min()?;
    let max_y = *y.max()?;

    Some((
        (min_x, (max_x - min_x) as u32 + 1).into(),
        (min_y, (max_y - min_y) as u32 + 1).into(),
    ))
}

fn clamp_scan(Scan { start, length }: Scan, end: u32) -> Scan {
    let (start, total) = if start + (length as i32) < 0 {
        (0, 0)
    } else if start < 0 {
        (0, length - (-start) as u32)
    } else {
        (start, length)
    };

    if start + total as i32 >= end as i32 {
        (start, (end - start as u32)).into()
    } else {
        (start, total).into()
    }
}

fn clamp_scan_to_scan(
    Scan { start, length }: Scan,
    Scan {
        start: lower,
        length: max,
    }: Scan,
) -> Scan {
    let (start, total) = if start < lower {
        (lower, (start + length as i32 - lower) as u32)
    } else {
        (start, length)
    };
    if start + total as i32 >= lower + max as i32 {
        (start, ((lower + max as i32) - start) as u32).into()
    } else {
        (start, total).into()
    }
}

fn as_start_and_total(start: i32, end: i32) -> (i32, u32) {
    if start < end {
        (start, (end - start) as u32 + 1)
    } else {
        (end, (start - end) as u32 + 1)
    }
}

pub fn line_scan(start: (i32, i32), end: (i32, i32), scan: i32) -> Option<Scan> {
    // Sort to make the line go from "top" to "bottom".
    let (start, end) = utility::swap_if((start, end), start.1 > end.1);

    // Early return if scan is outside of line bounds.
    if scan < start.1 || scan > end.1 {
        return None;
    }

    let (delta_x, delta_y) = (end.0 - start.0, end.1 - start.1);

    // Early return if line is horizontal.
    if delta_y == 0 {
        return Some(as_start_and_total(start.0, end.0).into());
    }

    // Check if the line is steep (there are no cases of pixels touching horizontally).
    let steep = delta_x.abs() <= delta_y;
    let x_points = delta_x + delta_x.signum();
    let y_points = delta_y + 1;
    if steep {
        let x = start.0 + x_points * (scan - start.1) / y_points;
        Some((x, 1).into())
    } else {
        let first = start.0 + (scan - start.1) * x_points / y_points;
        let second = start.0 + (scan - start.1 + 1) * x_points / y_points - delta_x.signum();
        Some(as_start_and_total(first, second).into())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn line_scan_works() {
        let (start, end) = ((0, 0), (5, 2));
        assert_eq!(line_scan(start, end, -1), None);
        assert_eq!(line_scan(start, end, 0), Some((0, 2).into()));
        assert_eq!(line_scan(start, end, 1), Some((2, 2).into()));
        assert_eq!(line_scan(start, end, 2), Some((4, 2).into()));
        assert_eq!(line_scan(start, end, 3), None);

        let (start, end) = ((1, 2), (3, 0));
        assert_eq!(line_scan(start, end, 0), Some((3, 1).into()));
        assert_eq!(line_scan(start, end, 1), Some((2, 1).into()));
        assert_eq!(line_scan(start, end, 2), Some((1, 1).into()));
        assert_eq!(line_scan(start, end, 3), None);

        let (start, end) = ((-1, 4), (3, 4));
        assert_eq!(line_scan(start, end, 3), None);
        assert_eq!(line_scan(start, end, 4), Some((-1, 5).into()));
        assert_eq!(line_scan(start, end, 5), None);
    }
}

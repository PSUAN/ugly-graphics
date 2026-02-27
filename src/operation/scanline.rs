use crate::utility;

pub mod line;

fn as_start_and_total(a: i32, b: i32) -> (i32, u32) {
    if a < b {
        (a, (b - a) as u32 + 1)
    } else {
        (b, (a - b) as u32 + 1)
    }
}

pub fn scanline(start: (i32, i32), end: (i32, i32), scanline: i32) -> Option<(i32, u32)> {
    // Sort to make the line go from "top" to "bottom".
    let (start, end) = utility::swap_if((start, end), start.1 > end.1);

    // Early return if scanline is outside of line bounds.
    if scanline < start.1 || scanline > end.1 {
        return None;
    }

    let (delta_x, delta_y) = (end.0 - start.0, end.1 - start.1);

    // Early return if line is horizontal.
    if delta_y == 0 {
        return Some(as_start_and_total(start.0, end.0));
    }

    // Check if the line is steep (there are no cases of pixels touching horizontally).
    let steep = delta_x.abs() <= delta_y;
    let x_points = delta_x + delta_x.signum();
    let y_points = delta_y + 1;
    if steep {
        let x = start.0 + x_points * (scanline - start.1) / y_points;
        Some((x, 1))
    } else {
        let first = start.0 + (scanline - start.1) * x_points / y_points;
        let second = start.0 + (scanline - start.1 + 1) * x_points / y_points - delta_x.signum();
        Some(as_start_and_total(first, second))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn scanline_works() {
        let (start, end) = ((0, 0), (5, 2));
        assert_eq!(scanline(start, end, -1), None);
        assert_eq!(scanline(start, end, 0), Some((0, 2)));
        assert_eq!(scanline(start, end, 1), Some((2, 2)));
        assert_eq!(scanline(start, end, 2), Some((4, 2)));
        assert_eq!(scanline(start, end, 3), None);

        let (start, end) = ((1, 2), (3, 0));
        assert_eq!(scanline(start, end, 0), Some((3, 1)));
        assert_eq!(scanline(start, end, 1), Some((2, 1)));
        assert_eq!(scanline(start, end, 2), Some((1, 1)));
        assert_eq!(scanline(start, end, 3), None);

        let (start, end) = ((-1, 4), (3, 4));
        assert_eq!(scanline(start, end, 3), None);
        assert_eq!(scanline(start, end, 4), Some((-1, 5)));
        assert_eq!(scanline(start, end, 5), None);
    }
}

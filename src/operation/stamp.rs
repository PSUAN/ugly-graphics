//! The [`Stamp`] applies an [`Image`] using provided [`Action`].

use crate::image::Image;
use crate::operation::Operation;
use crate::painter::DrawRegion;
use crate::strategy::apply;

/// Action that computes the resulting pixel value `P` given the original `P`
/// and provided source `S` values.
pub type Action<'a, P, S> = &'a dyn Fn(P, S) -> P;

/// The stamping operation to apply an [`Image`] over some [`Painter`].
#[derive(Clone, Copy)]
pub struct Stamp<'a, P, S> {
    position: (i32, i32),
    stamp: &'a dyn Image<Pixel = S>,
    action: Action<'a, P, S>,
}

impl<'a, P, S> Stamp<'a, P, S> {
    /// Create new instance for stamping provided `stamp` over the [`Painter`].
    pub fn new(
        position: (i32, i32),
        stamp: &'a dyn Image<Pixel = S>,
        action: Action<'a, P, S>,
    ) -> Self {
        Self {
            position,
            stamp,
            action,
        }
    }
}

impl<'a, P, S> Operation<P> for Stamp<'a, P, S>
where
    P: Clone,
    S: Clone,
{
    type Output = ();

    fn draw_on(self, painter: &mut DrawRegion<'_, '_, P>) -> Self::Output {
        let ((draw_x, draw_y), (draw_width, draw_height)) = painter.draw_zone();

        let (x, y) = self.position;
        let (stamp_width, stamp_height) = self.stamp.dimensions();

        let (start_x, start_y) = (
            if x < draw_x { draw_x - x } else { 0 },
            if y < draw_y { draw_y - y } else { 0 },
        );
        let (start_x, start_y) = (start_x as u32, start_y as u32);

        let (end_x, end_y) = (
            stamp_width.min(draw_width.saturating_sub_signed(x - draw_x)),
            stamp_height.min(draw_height.saturating_sub_signed(y - draw_y)),
        );

        for stamp_y in start_y..end_y {
            let target_y = y + stamp_y as i32;

            for stamp_x in start_x..end_x {
                if let Some(stamp_pixel) = self.stamp.pixel((stamp_x, stamp_y)) {
                    let target_x = x + stamp_x as i32;

                    let strategy =
                        move |passed_pixel| (self.action)(passed_pixel, stamp_pixel.clone());
                    painter.pixel((target_x, target_y), &apply(&strategy));
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use core::cell::RefCell;
    use std::boxed::Box;
    use std::string::String;
    use std::{println, vec};

    use crate::image::slice_based::SliceBased;
    use crate::image::{Dimensions, Image};
    use crate::operation::stamp::Stamp;
    use crate::painter::Painter;

    struct ReadAwareImage {
        buffer: RefCell<Box<[(bool, u8)]>>,
        width: u32,
        height: u32,
    }

    impl ReadAwareImage {
        fn new(value: u8, (width, height): (u32, u32)) -> Self {
            let data = vec![(false, value); (width * height) as _].into_boxed_slice();
            let buffer = RefCell::new(data);
            Self {
                buffer,
                width,
                height,
            }
        }

        fn pixel_was_read(&self, (x, y): (u32, u32)) -> Option<bool> {
            if x >= self.width || y >= self.height {
                return None;
            }
            let index = (x + y * self.width) as usize;

            let data = self.buffer.borrow();

            let (marker, _) = data.get(index)?;
            Some(*marker)
        }
    }

    impl Dimensions for ReadAwareImage {
        fn dimensions(&self) -> (u32, u32) {
            (self.width, self.height)
        }
    }

    impl Image for ReadAwareImage {
        type Pixel = u8;

        fn pixel(&self, (x, y): (u32, u32)) -> Option<Self::Pixel> {
            if x >= self.width || y >= self.height {
                return None;
            }
            let index = (x + y * self.width) as usize;

            let mut data = self.buffer.borrow_mut();

            let (marker, pixel) = data.get_mut(index)?;
            *marker = true;
            Some(*pixel)
        }
    }

    #[test]
    fn no_unnecessary_read_in_negative_space() {
        const STAMP_WIDTH: u32 = 32;
        const STAMP_HEIGHT: u32 = 16;
        const X: i32 = -8;
        const Y: i32 = -6;
        const SURFACE_WIDTH: u32 = 64;
        const SURFACE_HEIGHT: u32 = 32;
        const OFFSET_X: i32 = 2;
        const OFFSET_Y: i32 = 3;

        let image = ReadAwareImage::new(1, (STAMP_WIDTH, STAMP_HEIGHT));
        let stamp = Stamp::new((X, Y), &image, &|original, stamp| original + stamp);

        let surface_data = vec![0; (SURFACE_WIDTH * SURFACE_HEIGHT) as _].into_boxed_slice();

        let mut surface = SliceBased::new(surface_data, SURFACE_WIDTH).unwrap();
        let mut painter = Painter::new(&mut surface).with_offset((OFFSET_X, OFFSET_Y));

        painter.draw(stamp);

        for (index, chunk) in image.buffer.borrow().chunks(STAMP_WIDTH as _).enumerate() {
            let line = chunk
                .iter()
                .map(|(read, _)| if *read { '+' } else { '.' })
                .collect::<String>();
            println!("{:02}: {}", index, line);
        }

        for y in 0..STAMP_HEIGHT {
            for x in 0..STAMP_WIDTH {
                let pixel_was_read = image.pixel_was_read((x, y)).unwrap();
                let expected = x >= (-X - OFFSET_X) as u32 && y >= (-Y - OFFSET_Y) as u32;

                println!(
                    "At {}:{} pixel was {}READ, expecting it to {}been READ",
                    x,
                    y,
                    if pixel_was_read { "" } else { "NOT " },
                    if expected { "" } else { "NOT " }
                );

                assert_eq!(expected, pixel_was_read);
            }
        }

        for (index, chunk) in surface.data().chunks(SURFACE_WIDTH as _).enumerate() {
            let line = chunk
                .iter()
                .map(|value| if *value != 0 { '+' } else { '.' })
                .collect::<String>();
            println!("{:02}: {}", index, line);
        }

        for y in 0..SURFACE_HEIGHT {
            for x in 0..SURFACE_WIDTH {
                let pixel_value = surface.pixel((x, y)).unwrap();
                let expected = if (x as i32) >= X + OFFSET_X
                    && (y as i32) >= Y + OFFSET_Y
                    && (x as i32) < X + OFFSET_X + STAMP_WIDTH as i32
                    && (y as i32) < Y + OFFSET_Y + STAMP_HEIGHT as i32
                {
                    1
                } else {
                    0
                };

                assert_eq!(pixel_value, expected)
            }
        }
    }

    #[test]
    fn no_unnecessary_read_in_positive_space() {
        const STAMP_WIDTH: u32 = 32;
        const STAMP_HEIGHT: u32 = 16;
        const X: i32 = 60;
        const Y: i32 = 25;
        const SURFACE_WIDTH: u32 = 64;
        const SURFACE_HEIGHT: u32 = 32;
        const OFFSET_X: i32 = 2;
        const OFFSET_Y: i32 = 3;

        let image = ReadAwareImage::new(1, (STAMP_WIDTH, STAMP_HEIGHT));
        let stamp = Stamp::new((X, Y), &image, &|original, stamp| original + stamp);

        let surface_data = vec![0; (SURFACE_WIDTH * SURFACE_HEIGHT) as _].into_boxed_slice();

        let mut surface = SliceBased::new(surface_data, SURFACE_WIDTH).unwrap();
        let mut painter = Painter::new(&mut surface).with_offset((OFFSET_X, OFFSET_Y));

        painter.draw(stamp);

        for (index, chunk) in image.buffer.borrow().chunks(STAMP_WIDTH as _).enumerate() {
            let line = chunk
                .iter()
                .map(|(read, _)| if *read { '+' } else { '.' })
                .collect::<String>();
            println!("{:02}: {}", index, line);
        }

        for y in 0..STAMP_HEIGHT {
            for x in 0..STAMP_WIDTH {
                let pixel_was_read = image.pixel_was_read((x, y)).unwrap();
                let expected = x < (SURFACE_WIDTH - X as u32 - OFFSET_X as u32)
                    && y < (SURFACE_HEIGHT - Y as u32 - OFFSET_Y as u32);

                println!(
                    "At {}:{} pixel was {}READ, expecting it to {}been READ",
                    x,
                    y,
                    if pixel_was_read { "" } else { "NOT " },
                    if expected { "" } else { "NOT " }
                );

                assert_eq!(expected, pixel_was_read);
            }
        }

        for (index, chunk) in surface.data().chunks(SURFACE_WIDTH as _).enumerate() {
            let line = chunk
                .iter()
                .map(|value| if *value != 0 { '+' } else { '.' })
                .collect::<String>();
            println!("{:02}: {}", index, line);
        }

        for y in 0..SURFACE_HEIGHT {
            for x in 0..SURFACE_WIDTH {
                let pixel_value = surface.pixel((x, y)).unwrap();
                let expected = if (x as i32) >= X + OFFSET_X
                    && (y as i32) >= Y + OFFSET_Y
                    && (x as i32) < X + OFFSET_X + STAMP_WIDTH as i32
                    && (y as i32) < Y + OFFSET_Y + STAMP_HEIGHT as i32
                {
                    1
                } else {
                    0
                };

                assert_eq!(pixel_value, expected)
            }
        }
    }
}

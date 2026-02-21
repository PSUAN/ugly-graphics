use crate::image::{Dimensions, Image, ImageMut};

pub struct Sprite<C, const W: usize, const H: usize> {
    data: [[C; W]; H],
}

impl<C, const W: usize, const H: usize> Sprite<C, W, H> {
    pub fn from_copies(value: C) -> Self
    where
        C: Copy,
    {
        let data = [[value; W]; H];
        Self { data }
    }

    pub fn from_raw(data: [[C; W]; H]) -> Self {
        Self { data }
    }
}

impl<C, const W: usize, const H: usize> Dimensions for Sprite<C, W, H> {
    fn dimensions(&self) -> (i32, i32) {
        (W as _, H as _)
    }
}

impl<C, const W: usize, const H: usize> Image<C> for Sprite<C, W, H> {
    fn pixel(&self, (x, y): (i32, i32)) -> Option<&C> {
        let (x, y) = (usize::try_from(x).ok()?, usize::try_from(y).ok()?);
        self.data.get(y)?.get(x)
    }
}

impl<C, const W: usize, const H: usize> ImageMut<C> for Sprite<C, W, H>
where
    C: Clone,
{
    fn set_pixel(&mut self, (x, y): (i32, i32), value: C) {
        let indices = (usize::try_from(x).ok(), usize::try_from(y).ok());
        if let (Some(x), Some(y)) = indices
            && let Some(row) = self.data.get_mut(y)
            && let Some(pixel) = row.get_mut(x)
        {
            *pixel = value;
        }
    }

    fn modify_pixel(&mut self, (x, y): (i32, i32), function: &dyn Fn((i32, i32), C) -> C) {
        let indices = (usize::try_from(x).ok(), usize::try_from(y).ok());
        if let (Some(index_x), Some(index_y)) = indices
            && let Some(row) = self.data.get_mut(index_y)
            && let Some(pixel) = row.get_mut(index_x)
        {
            *pixel = function((x, y), pixel.clone());
        }
    }

    fn set_horizontal_line(&mut self, (x, y): (i32, i32), plus: u32, value: C) {
        let indices = (usize::try_from(x).ok(), usize::try_from(y).ok());
        let plus = usize::try_from(plus).ok();
        if let (Some(index_x), Some(index_y)) = indices
            && let Some(plus) = plus
            && let Some(row) = self.data.get_mut(index_y)
            && let Some(slice) = row.get_mut(index_x..(index_x + plus))
        {
            slice.fill_with(|| value.clone());
        }
    }

    fn modify_horizontal_line(
        &mut self,
        (x, y): (i32, i32),
        plus: u32,
        function: &dyn Fn((i32, i32), C) -> C,
    ) {
        let indices = (usize::try_from(x).ok(), usize::try_from(y).ok());
        let plus = usize::try_from(plus).ok();
        if let (Some(index_x), Some(index_y)) = indices
            && let Some(plus) = plus
            && let Some(row) = self.data.get_mut(index_y)
            && let Some(slice) = row.get_mut(index_x..(index_x + plus))
        {
            slice.iter_mut().enumerate().for_each(|(index, pixel)| {
                *pixel = function(((index + index_x) as _, y), pixel.clone());
            });
        }
    }

    fn set(&mut self, value: C) {
        for row in self.data.iter_mut() {
            row.fill_with(|| value.clone());
        }
    }

    fn modify(&mut self, function: &dyn Fn((i32, i32), C) -> C) {
        for (y, row) in self.data.iter_mut().enumerate() {
            let y = y as _;

            row.iter_mut()
                .enumerate()
                .for_each(|(index, pixel)| *pixel = function((index as _, y), pixel.clone()));
        }
    }
}

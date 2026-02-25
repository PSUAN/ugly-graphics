pub mod sprite;

pub trait Dimensions {
    fn dimensions(&self) -> (i32, i32);
}

impl<T> Dimensions for &T
where
    T: Dimensions,
{
    fn dimensions(&self) -> (i32, i32) {
        Dimensions::dimensions(*self)
    }
}

impl<T> Dimensions for &mut T
where
    T: Dimensions,
{
    fn dimensions(&self) -> (i32, i32) {
        Dimensions::dimensions(*self)
    }
}

pub trait Image<P>: Dimensions {
    fn pixel(&self, position: (i32, i32)) -> Option<P>;
}

impl<T, P> Image<P> for &T
where
    T: Image<P>,
{
    fn pixel(&self, position: (i32, i32)) -> Option<P> {
        Image::pixel(*self, position)
    }
}

impl<T, P> Image<P> for &mut T
where
    T: Image<P>,
{
    fn pixel(&self, position: (i32, i32)) -> Option<P> {
        Image::pixel(*self, position)
    }
}

pub trait ImageMut<P>: Dimensions {
    fn set_pixel(&mut self, position: (i32, i32), value: P);
    fn modify_pixel(&mut self, position: (i32, i32), function: &dyn Fn((i32, i32), P) -> P);

    fn set_horizontal_line(&mut self, position: (i32, i32), plus: u32, value: P);
    fn modify_horizontal_line(
        &mut self,
        position: (i32, i32),
        plus: u32,
        function: &dyn Fn((i32, i32), P) -> P,
    );

    fn set(&mut self, value: P);
    fn modify(&mut self, function: &dyn Fn((i32, i32), P) -> P);
}

impl<T, P> ImageMut<P> for &mut T
where
    T: ImageMut<P>,
{
    fn set_pixel(&mut self, position: (i32, i32), value: P) {
        ImageMut::set_pixel(*self, position, value);
    }

    fn modify_pixel(&mut self, position: (i32, i32), function: &dyn Fn((i32, i32), P) -> P) {
        ImageMut::modify_pixel(*self, position, function);
    }

    fn set_horizontal_line(&mut self, position: (i32, i32), plus: u32, value: P) {
        ImageMut::set_horizontal_line(*self, position, plus, value);
    }

    fn modify_horizontal_line(
        &mut self,
        position: (i32, i32),
        plus: u32,
        function: &dyn Fn((i32, i32), P) -> P,
    ) {
        ImageMut::modify_horizontal_line(*self, position, plus, function);
    }

    fn set(&mut self, value: P) {
        ImageMut::set(*self, value);
    }

    fn modify(&mut self, function: &dyn Fn((i32, i32), P) -> P) {
        ImageMut::modify(*self, function);
    }
}

use crate::image::ImageMut;
use crate::operation::Operation;
use crate::strategy::Strategy;

pub struct Painter<'a, P> {
    target: &'a mut dyn ImageMut<P>,
}

impl<'a, P> Painter<'a, P> {
    pub fn new(target: &'a mut dyn ImageMut<P>) -> Self {
        Self { target }
    }
}

impl<'a, P> Painter<'a, P> {
    pub fn draw<O>(&mut self, operation: O) -> Option<O::Output>
    where
        O: Operation<P>,
    {
        operation.draw_on(self)
    }

    pub fn pixel(&mut self, position: (i32, i32), strategy: Strategy<P>) {
        match strategy {
            Strategy::Overwrite(value) => self.target.set_pixel(position, value),
            Strategy::Apply(function) => self.target.modify_pixel(position, function),
        }
    }
}

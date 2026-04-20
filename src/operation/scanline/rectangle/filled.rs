use crate::operation::Operation;
use crate::painter::Painter;
use crate::strategy::Strategy;
use crate::utility;

pub struct FilledRectangle<'a, P> {
    from: (i32, i32),
    to: (i32, i32),
    value: Strategy<'a, P>,
}

impl<'a, P> FilledRectangle<'a, P> {
    pub fn new(from: (i32, i32), to: (i32, i32), value: Strategy<'a, P>) -> Self {
        Self { from, to, value }
    }
}

impl<P> Operation<P> for FilledRectangle<'_, P>
where
    P: Clone,
{
    type Output = ();

    fn draw_on(self, painter: &mut Painter<'_, P>) -> Self::Output {
        let (from, to) = {
            let x = utility::swap_if(self.from.0 > self.to.0, (self.from.0, self.to.0));
            let y = utility::swap_if(self.from.1 > self.to.1, (self.from.1, self.to.1));
            ((x.0, y.0), (x.1, y.1))
        };

        for y in from.1..to.1 + 1 {
            painter.horizontal_line(from.0..to.0 + 1, y, &self.value);
        }
    }
}

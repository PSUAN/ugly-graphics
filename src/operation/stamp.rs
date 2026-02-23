use crate::image::Image;
use crate::operation::Operation;
use crate::painter::Painter;
use crate::strategy::IntoApply;

pub type Action<'a, P, S> = &'a dyn Fn((i32, i32), P, (i32, i32), S) -> P;

pub struct Stamp<'a, P, S> {
    position: (i32, i32),
    stamp: &'a dyn Image<S>,
    action: Action<'a, P, S>,
}

impl<'a, P, S> Stamp<'a, P, S> {
    pub fn new(position: (i32, i32), stamp: &'a dyn Image<S>, action: Action<'a, P, S>) -> Self {
        Self {
            position,
            stamp,
            action,
        }
    }
}

impl<'a, P, S> Operation<P> for Stamp<'a, P, S>
where
    S: Clone,
{
    type Output = ();

    fn draw_on(self, painter: &mut Painter<'_, P>) -> Option<Self::Output> {
        let (x, y) = self.position;
        let (width, height) = self.stamp.dimensions();

        for stamp_y in 0..height {
            let target_y = y + stamp_y;
            for stamp_x in 0..width {
                if let Some(stamp_pixel) = self.stamp.pixel((stamp_x, stamp_y)) {
                    let target_x = x + stamp_x;

                    let strategy = move |passed_position, passed_pixel| {
                        (self.action)(
                            passed_position,
                            passed_pixel,
                            (stamp_x, stamp_y),
                            stamp_pixel.clone(),
                        )
                    };
                    painter.pixel((target_x, target_y), strategy.apply());
                }
            }
        }
        Some(())
    }
}

//! [`Operation`] is an action applied to a [`Painter`].

use crate::painter::DrawRegion;

pub mod compute;
pub mod pixel;
pub mod scanline;
pub mod stamp;

/// An operation applied to a [`Painter`].
///
/// May return additional data upon completion.
pub trait Operation<P> {
    /// The additional data to be returned.
    type Output;

    /// Draw `self` on a provided [`Painter`].
    fn draw_on(self, painter: &mut DrawRegion<'_, '_, P>) -> Self::Output;
}

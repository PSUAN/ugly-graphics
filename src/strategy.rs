//! [`Strategy`] allows to distinguish between overwrite and [`Modify`]
//! operations.

/// A computation operation over pixel.
pub type Modify<'a, P> = &'a dyn Fn(P) -> P;

/// A strategy to be applied to specific pixel set.
#[derive(Clone, Copy)]
pub enum Strategy<'a, P> {
    /// Overwrite the pixel.
    Overwrite(P),
    /// Apply new value using the [`modification`](`Modify`) to compute new
    /// value.
    Apply(Modify<'a, P>),
}

/// Trait to convert a specific pixel value into the overwrite [`Strategy`].
pub trait IntoOverwrite: Sized {
    /// Convert pixel value into the overwrite [`Strategy`].
    fn overwrite(self) -> Strategy<'static, Self>;
}

impl<P> IntoOverwrite for P {
    fn overwrite(self) -> Strategy<'static, Self> {
        Strategy::Overwrite(self)
    }
}

/// Trait to convert a specific action value into the apply [`Strategy`].
pub trait IntoApply<P> {
    /// Convert [`Modify`] into the apply [`Strategy`].
    fn apply<'a>(&'a self) -> Strategy<'a, P>;
}

impl<F, P> IntoApply<P> for F
where
    F: Fn(P) -> P,
{
    fn apply<'a>(&'a self) -> Strategy<'a, P> {
        Strategy::Apply(self)
    }
}

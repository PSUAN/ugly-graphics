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

/// Create an [`Overwrite`](`Strategy::Overwrite`) strategy.
pub fn overwrite<'a, P>(value: P) -> Strategy<'a, P> {
    Strategy::Overwrite(value)
}

/// Create an [`Apply`](`Strategy::Apply`) strategy.
pub fn apply<'a, P>(modifier: Modify<'a, P>) -> Strategy<'a, P> {
    Strategy::Apply(modifier)
}

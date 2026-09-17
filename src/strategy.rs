//! [`Strategy`] allows to distinguish between overwrite and [`Modify`]
//! operations.

/// A computation operation over pixel.
pub type Modify<'a, P> = &'a dyn Fn(P) -> P;

/// This is a strategy to replace pixels without reading.
#[derive(Clone, Copy)]
pub struct Overwrite<P>(pub P);

/// This is a strategy to compute new pixel value using the previous one.
#[derive(Clone, Copy)]
pub struct Apply<'a, P>(pub Modify<'a, P>);

/// Create an [`Overwrite`](`Strategy::Overwrite`) strategy.
pub fn overwrite<P>(value: P) -> Overwrite<P> {
    Overwrite(value)
}

/// Create an [`Apply`](`Strategy::Apply`) strategy.
pub fn apply<'a, P>(modifier: Modify<'a, P>) -> Apply<'a, P> {
    Apply(modifier)
}

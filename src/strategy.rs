pub type Modify<'a, P> = &'a dyn Fn((i32, i32), P) -> P;

pub enum Strategy<'a, P> {
    Overwrite(P),
    Apply(Modify<'a, P>),
}

pub trait IntoOverwrite: Sized {
    fn overwrite(self) -> Strategy<'static, Self>;
}

impl<P> IntoOverwrite for P {
    fn overwrite(self) -> Strategy<'static, Self> {
        Strategy::Overwrite(self)
    }
}

pub trait IntoApply<P> {
    fn apply<'a>(&'a self) -> Strategy<'a, P>;
}

impl<F, P> IntoApply<P> for F
where
    F: Fn((i32, i32), P) -> P,
{
    fn apply<'a>(&'a self) -> Strategy<'a, P> {
        Strategy::Apply(self)
    }
}

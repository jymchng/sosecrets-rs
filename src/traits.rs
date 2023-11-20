use core::ops::Add;
use typenum::{consts::U1, IsLess, Sum, True, Unsigned};

pub trait ExposeSecret<'max, T, MEC: Unsigned, EC: Unsigned>: Sized {
    type Exposed<'brand>
    where
        'max: 'brand;

    type Next: ExposeSecret<'max, T, MEC, Sum<EC, U1>>
    where
        EC: Add<U1> + Unsigned + IsLess<MEC>,
        Sum<EC, U1>: Unsigned + IsLess<MEC> + Add<U1>;

    fn expose_secret<ReturnType, ClosureType>(self, scope: ClosureType) -> (Self::Next, ReturnType)
    where
        for<'brand> ClosureType: FnOnce(Self::Exposed<'brand>) -> ReturnType,
        EC: Add<U1> + IsLess<MEC, Output = True>,
        Sum<EC, U1>: Unsigned + Add<U1> + IsLess<MEC>;
}

pub trait CloneableSecret<T: Clone, MEC: Unsigned, EC: Unsigned + IsLess<MEC> + Add<U1>>:
    Sized
{
    fn clone_secret(&self) -> Self;
}

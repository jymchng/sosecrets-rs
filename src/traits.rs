use core::ops::Add;
use typenum::{IsLessOrEqual, Sum, True, Unsigned, U1};
use zeroize::Zeroize;

pub trait ExposeSecret<'max, T, MEC: Unsigned, EC: Unsigned>: Sized {
    type Exposed<'brand>
    where
        'max: 'brand;

    type Next: ExposeSecret<'max, T, MEC, Sum<EC, U1>>
    where
        EC: Add<U1> + Unsigned + IsLessOrEqual<MEC, Output = True>,
        Sum<EC, U1>: Unsigned + IsLessOrEqual<MEC, Output = True> + Add<U1>;

    fn expose_secret<ReturnType, ClosureType>(self, scope: ClosureType) -> (Self::Next, ReturnType)
    where
        for<'brand> ClosureType: FnOnce(Self::Exposed<'brand>) -> ReturnType,
        EC: Add<U1> + IsLessOrEqual<MEC, Output = True>,
        Sum<EC, U1>: Unsigned + Add<U1> + IsLessOrEqual<MEC, Output = True>;
}

pub trait CloneableSecret<
    T: Clone,
    MEC: Unsigned,
    EC: Unsigned + IsLessOrEqual<MEC, Output = True> + Add<U1>,
>: Sized
{
    fn clone_secret(&self) -> Self;
}

pub trait SecretIntoInner<T, MEC, EC>: Sized
where
    T: Zeroize,
    MEC: Unsigned,
    EC: Unsigned + Add<U1> + IsLessOrEqual<MEC, Output = True>,
{
    fn into_inner(self) -> T;
}

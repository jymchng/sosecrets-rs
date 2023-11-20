use typenum::{consts::U1, IsLess, Sum, True, Unsigned};

pub trait ExposeSecret<'max, T, MEC: Unsigned, EC: Unsigned>: Sized {
    type Exposed<'brand>
    where
        'max: 'brand;

    type Next: ExposeSecret<'max, T, MEC, Sum<EC, U1>>
    where
        EC: core::ops::Add<U1> + Unsigned + typenum::IsLess<MEC>,
        Sum<EC, U1>: Unsigned + IsLess<MEC> + core::ops::Add<typenum::U1>;

    fn expose_secret<ReturnType, ClosureType>(self, scope: ClosureType) -> (Self::Next, ReturnType)
    where
        for<'brand> ClosureType: FnOnce(Self::Exposed<'brand>) -> ReturnType,
        EC: core::ops::Add<U1> + IsLess<MEC, Output = True>,
        Sum<EC, U1>: Unsigned + core::ops::Add<U1> + IsLess<MEC>;
}

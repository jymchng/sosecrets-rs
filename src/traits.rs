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

pub trait SecretIntoInner<T, MEC, EC>: Sized
where
    T: Zeroize,
    MEC: Unsigned,
    EC: Unsigned + Add<U1> + IsLessOrEqual<MEC, Output = True>,
{
    fn into_inner(self) -> T;
}

#[cfg(feature = "cloneable-secret")]
pub use self::cloneable_secret::CloneableSecret;

#[cfg(feature = "cloneable-secret")]
mod cloneable_secret {

    use core::clone::Clone;
    use zeroize::Zeroize;

    pub trait CloneableSecret: Clone + Zeroize {}

    impl<T: Clone + Zeroize, const N: usize> CloneableSecret for [T; N] {}
    impl CloneableSecret for String {}
    impl<T: Clone + Zeroize> CloneableSecret for Vec<T> {}
    crate::impl_cloneable_secret_for_numbers!(
        i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64
    );
}

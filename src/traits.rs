use crate::secret::{AddU1, ExposedSecret, Secret};
use typenum::{consts::U1, Bit, IsLess, True, Unsigned};
use zeroize::Zeroize;

pub trait ExposeSecret<
    T: Zeroize,
    MEC: Unsigned,
    EC: core::ops::Add<typenum::U1> + Unsigned + typenum::IsLess<MEC>,
>
{
    fn expose_secret<ReturnType>(
        self,
        scope: impl FnOnce(ExposedSecret<T, MEC, EC>) -> (ExposedSecret<T, MEC, EC>, ReturnType),
    ) -> (Secret<T, MEC, AddU1<EC>>, ReturnType)
    where
        AddU1<EC>: core::ops::Add<typenum::U1> + Unsigned + typenum::IsLess<MEC>,
        EC: IsLess<MEC, Output = True>;
}

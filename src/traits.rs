use crate::secret::{AddU1, Secret};
use typenum::{consts::U1, Bit, IsLess, Unsigned};
use zeroize::Zeroize;

pub trait ExposeSecret<
    'lt,
    T: Zeroize,
    MEC: Unsigned,
    EC: core::ops::Add<typenum::U1> + Unsigned + typenum::IsLess<MEC>,
>
{
    const ASSERT_EC_LESS_THAN_MEC: () = assert!(<<EC as IsLess<MEC>>::Output as Bit>::BOOL);
    type Exposed<'a>;

    fn expose_secret<ReturnType>(
        self,
        scope: impl for<'a> FnOnce(Self::Exposed<'a>) -> (Self::Exposed<'a>, ReturnType),
    ) -> (Self, ReturnType)
    where
        AddU1<EC>: core::ops::Add<typenum::U1> + Unsigned + typenum::IsLess<MEC>;
}

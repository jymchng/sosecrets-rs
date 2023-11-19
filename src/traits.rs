// use crate::secret::{AddU1, Secret};
// use typenum::{consts::U1, Bit, IsLess, True, Unsigned};
// use zeroize::Zeroize;

// pub trait ExposeSecret<
//     T: Zeroize,
//     MEC: Unsigned,
//     EC: core::ops::Add<typenum::U1> + Unsigned + typenum::IsLess<MEC>,
// >
// {
//     type Exposed<'brand>;

//     fn expose_secret<ReturnType, ClosureType>(self, scope: ClosureType) -> (Self, ReturnType)
//     where
//         AddU1<EC>: core::ops::Add<typenum::U1> + Unsigned + typenum::IsLess<MEC, Output = True>,
//         for<'a> Self::Exposed<'a>:,
//         for<'b> ClosureType: FnOnce(Self::Exposed<'b>) -> (Self::Exposed<'b>, ReturnType);
// }

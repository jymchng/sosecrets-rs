// use core::sync::atomic::Ordering;
// use crate::traits::ChooseMinimallyRepresentableUInt;
// use typenum::Unsigned;

// pub struct RTSyncSecret<T, MEC: Unsigned + ChooseMinimallyRepresentableUInt>(T, <MEC as ChooseMinimallyRepresentableUInt>::AtomicOutput);

// impl<T, MEC: Unsigned + ChooseMinimallyRepresentableUInt> RTSyncSecret<T, MEC> {
//     pub const fn new(value: T) -> Self {
//         Self(value, <MEC as ChooseMinimallyRepresentableUInt>::AtomicOutput::new(0))
//     }

//     pub fn new_with(f: impl FnOnce() -> T) -> Self {
//         Self(f(), <MEC as ChooseMinimallyRepresentableUInt>::AtomicOutput::new(0))
//     }

//     pub fn exposure_count(&self) -> usize {
//         self.1.load(Ordering::Relaxed)
//     }
// }

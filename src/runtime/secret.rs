use core::{cell::UnsafeCell, marker::PhantomData, ops::Deref};

extern crate std;

use crate::runtime::traits;
pub struct RTSecret<T, const MEC: usize>(T, UnsafeCell<usize>);
pub struct RTExposedSecret<'brand, T>(T, PhantomData<fn(&'brand ()) -> &'brand ()>);

impl<'brand, T> Deref for RTExposedSecret<'brand, &'brand T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<T, const MEC: usize> RTSecret<T, MEC> {
    pub const fn new(value: T) -> Self {
        Self(value, UnsafeCell::new(0))
    }

    pub fn new_with(f: impl FnOnce() -> T) -> Self {
        Self(f(), UnsafeCell::new(0))
    }
}

impl<'secret, T, const MEC: usize> traits::RTExposeSecret<'secret, &'secret T>
    for RTSecret<T, MEC>
{
    type Exposed<'brand> = RTExposedSecret<'brand, &'brand T>
    where
        'secret: 'brand;

    fn expose_secret<ReturnType, ClosureType>(&self, scope: ClosureType) -> ReturnType
    where
        for<'brand> ClosureType: FnOnce(RTExposedSecret<'brand, &'brand T>) -> ReturnType,
    {
        let ec_mut = unsafe { &mut *self.1.get() };
        *ec_mut += 1;
        if *ec_mut > MEC {
            panic!("`RTSecret` has been exposed {} times, more than what it is maximally allowed for: {} times", *ec_mut, MEC);
        };
        scope(RTExposedSecret(&self.0, PhantomData))
    }
}

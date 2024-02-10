use core::{cell::UnsafeCell, marker::PhantomData, ops::Deref};

extern crate std;

use crate::runtime::traits;
pub struct RunTimeSecret<T, const MEC: usize>(T, UnsafeCell<usize>);
pub struct ExposedSecret<'brand, T>(T, PhantomData<fn(&'brand ()) -> &'brand ()>);

impl<'brand, T> Deref for ExposedSecret<'brand, &'brand T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<T, const MEC: usize> RunTimeSecret<T, MEC> {
    pub const fn new(value: T) -> Self {
        Self(value, UnsafeCell::new(0))
    }

    pub fn new_with(f: impl FnOnce() -> T) -> Self {
        Self(f(), UnsafeCell::new(0))
    }
}

impl<'secret, T, const MEC: usize> traits::ExposeSecret<'secret, &'secret T>
    for RunTimeSecret<T, MEC>
{
    type Exposed<'brand> = ExposedSecret<'brand, &'brand T>
    where
        'secret: 'brand;

    fn expose_secret<ReturnType, ClosureType>(&self, scope: ClosureType) -> ReturnType
    where
        for<'brand> ClosureType: FnOnce(ExposedSecret<'brand, &'brand T>) -> ReturnType,
    {
        let ec_mut = unsafe { &mut *self.1.get() };
        *ec_mut += 1;
        if *ec_mut > MEC {
            panic!("`RunTimeSecret` has been exposed {} times, more than what it is maximally allowed for: {} times", *ec_mut, MEC);
        };
        scope(ExposedSecret(&self.0, PhantomData))
    }
}

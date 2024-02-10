use core::marker::PhantomData;

use crate::runtime::traits;
pub struct RunTimeSecret<T, const MEC: usize>(T, usize);
pub struct ExposedSecret<'brand, T>(T, PhantomData<fn(&'brand ()) -> &'brand ()>);

impl<T, const MEC: usize> RunTimeSecret<T, MEC> {
    pub const fn new(value: T) -> Self {
        Self(value, 0)
    }

    pub fn new_with(f: impl FnOnce() -> T) -> Self {
        Self(f(), 0)
    }
}

impl<'secret, T, const MEC: usize> traits::ExposeSecret<'secret, &'secret T>
    for RunTimeSecret<T, MEC>
{
    type Expose<'brand> = ExposedSecret<'brand, &'brand T>
    where
        'secret: 'brand;

    fn expose_secret<ClosureType, ReturnType>(&mut self, scope: ClosureType) -> ReturnType
    where
        ClosureType: for<'brand> FnOnce(Self::Expose<'brand>) -> ReturnType,
    {
        if self.1 > MEC {
            panic!("`RunTimeSecret` has been exposed {} times, more than what it is maximally allowed for", self.1)
        };
        self.1 += 1;
        scope(ExposedSecret(&*self.0, PhantomData))
    }
}

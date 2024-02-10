pub trait ExposeSecret<'secret, T> {
    type Expose<'brand>
    where
        'secret: 'brand;

    fn expose_secret<ClosureType, ReturnType>(&mut self, scope: ClosureType) -> ReturnType
    where
        ClosureType: for<'brand> FnOnce(Self::Expose<'brand>) -> ReturnType;

    // fn try_expose_secret<ClosureType, ReturnType>(&self, scope: ClosureType) -> Result<ReturnType, >
    // where
    //     ClosureType: for<'brand> FnOnce(Self::Expose<'brand>) -> ReturnType;
}

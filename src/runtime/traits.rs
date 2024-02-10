pub trait ExposeSecret<'secret, T> {
    type Exposed<'brand>
    where
        'secret: 'brand;

    fn expose_secret<ReturnType, ClosureType>(&self, scope: ClosureType) -> ReturnType
    where
        for<'brand> ClosureType: FnOnce(Self::Exposed<'brand>) -> ReturnType;

    // fn try_expose_secret<ClosureType, ReturnType>(&self, scope: ClosureType) -> Result<ReturnType, >
    // where
    //     ClosureType: for<'brand> FnOnce(Self::Expose<'brand>) -> ReturnType;
}

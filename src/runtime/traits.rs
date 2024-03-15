/// A trait for exposing secrets with runtime checking.
pub trait RTExposeSecret<'secret, T> {
    /// The type representing the `Error` variant as part of the `Result` returned type in `try_expose_secret`.
    type Error: core::fmt::Display + core::fmt::Debug;

    /// The type representing the exposed secret.
    type Exposed<'brand>
    where
        'secret: 'brand;

    /// Exposes the secret with runtime checking.
    ///
    /// # Parameters
    /// - `scope`: A closure that takes the exposed secret and returns a value of the `ReturnType`.
    ///
    /// # Returns
    /// The value returned by the closure.
    fn expose_secret<ReturnType, ClosureType>(&self, scope: ClosureType) -> ReturnType
    where
        for<'brand> ClosureType: FnOnce(Self::Exposed<'brand>) -> ReturnType;

    /// Tries to expose the secret with runtime checking.
    ///
    /// # Parameters
    /// - `scope`: A closure that takes the exposed secret and returns a value of the `ReturnType`.
    ///
    /// # Returns
    /// - `Ok`: The value returned by the closure.
    /// - `Err`: If there is an error during exposure, it returns an error of type `ErrorType`.
    fn try_expose_secret<ReturnType, ClosureType>(
        &self,
        scope: ClosureType,
    ) -> Result<ReturnType, Self::Error>
    where
        for<'brand> ClosureType: FnOnce(Self::Exposed<'brand>) -> ReturnType;
}

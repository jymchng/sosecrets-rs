#[macro_export]
macro_rules! generic_const_predicate {( $e:expr $(,)? ) => (
    [(); $e as bool as usize - 1]
)}

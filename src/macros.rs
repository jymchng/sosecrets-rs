#[macro_export]
macro_rules! generic_const_predicate {( $e:expr $(,)? ) => (
    [(); $e as bool as usize - 1]
)}

#[macro_export]
macro_rules! is_ec_less_than_mec {( $ec:ident, $mec:ident $(,)? ) => (
    [(); <<$ec as IsLess<$mec>>::Output as Bit>::BOOL as usize - 1]
)}


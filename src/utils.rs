/// Copied from https://github.com/matthieu-m/static-rc/blob/master/src/utils.rs
/// A work-around arithmetic conditions in `where` clauses.

#[cfg(feature = "compile-time-check")]
#[macro_export]
macro_rules! AssertLeType {
    ($left:expr, $right:expr) => {
        [(); $right - $left]
    };
}

#[cfg(not(feature = "compile-time-check"))]
#[macro_export]
macro_rules! AssertLeType {
    ($left:expr, $right:expr) => {
        ()
    };
}

#[macro_export]
macro_rules! AssertEqType {
    ($left:expr, $right: expr) => {
        (AssertLeType!($left, $right), AssertLeType!($right, $left))
    };
}

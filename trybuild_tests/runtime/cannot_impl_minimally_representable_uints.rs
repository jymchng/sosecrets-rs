use sosecrets_rs::{
    prelude::typenum::{Unsigned, U69, U8},
    runtime::traits::{MinimallyRepresentableUInt, __private},
};

fn main() {
    #[derive(Copy, Default, Clone)]
    struct A;

    // cannot impl Unsigned
    impl Unsigned for A {}

    // cannot impl MinimallyRepresentableUInt
    impl MinimallyRepresentableUInt for A {}

    // cannot call the method `cast_unsigned_to_self_type`
    let a = <U8 as MinimallyRepresentableUInt>::cast_unsigned_to_self_type::<U69>(
        __private::SealedToken {},
    );
}

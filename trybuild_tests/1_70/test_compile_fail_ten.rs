use sosecrets_rs::{prelude::*, traits::ExposeSecret};
use typenum::consts::U2;

#[derive(Default)]
struct AStruct {
    _inner: i32,
}

#[derive(Default)]
struct ReferenceWrapper<'a> {
    _inner: Option<&'a AStruct>,
}

fn make_return_secret<'a>(a_struct_: &'a AStruct) -> Secret<&'a AStruct, U2> {
    Secret::new(&a_struct_)
}

fn main() {
    let a_struct = AStruct { _inner: 69 };

    let secret_astruct = make_return_secret(&a_struct);

    let mut ref_wrapper: ReferenceWrapper = Default::default();

    let (_, _) = secret_astruct.expose_secret(|exposed_secret| {
        ref_wrapper._inner.insert(*exposed_secret);
    });

    assert_eq!(ref_wrapper._inner.take().unwrap()._inner, 69);
}

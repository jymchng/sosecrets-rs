use sosecrets_rs::{prelude::*, traits::ExposeSecret};
use typenum::consts::U2;
#[cfg(feature = "zeroize")]
use zeroize::Zeroize;

#[cfg(feature = "zeroize")]
impl Zeroize for AStruct {
    fn zeroize(&mut self) {
        self._inner.zeroize()
    }
}

struct AStruct {
    _inner: i32,
}

struct ReferenceWrapper {
    _inner: Option<AStruct>,
}

fn make_return_secret(a_struct_: AStruct) -> Secret<AStruct, U2> {
    Secret::new(a_struct_)
}

fn main() {
    let a_struct = AStruct { _inner: 69 };

    let secret_astruct = make_return_secret(a_struct);

    let mut ref_wrapper = ReferenceWrapper { _inner: None };

    let (_, _) = secret_astruct.expose_secret(|exposed_secret| {
        ref_wrapper._inner.insert(*exposed_secret);
    });

    assert_eq!(ref_wrapper._inner.take().unwrap()._inner, 69);
}

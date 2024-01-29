struct AStruct {
    _inner: i32,
}

struct ReferenceWrapper<'a> {
    _inner: Option<&'a AStruct>,
}

struct NormalSecret<T>(T);

struct ExposedNormalSecret<T>(T);

impl<T> NormalSecret<T> {
    fn expose_secret<ClosureType, ReturnType>(self, scope: ClosureType) -> (Self, ReturnType)
    where
        ClosureType: FnOnce(ExposedNormalSecret<&T>) -> ReturnType,
    {
        let returned_value = scope(ExposedNormalSecret(&self.0));
        return (Self(self.0), returned_value);
    }
}

impl<T> core::ops::Deref for ExposedNormalSecret<&'_ T> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &T {
        self.0
    }
}

fn make_return_secret<'a>(a_struct_: &'a AStruct) -> NormalSecret<&'a AStruct> {
    NormalSecret(a_struct_)
}

fn main() {
    let a_struct = AStruct { _inner: 69 };

    let secret_astruct = make_return_secret(&a_struct);

    let mut ref_wrapper: ReferenceWrapper = ReferenceWrapper { _inner: None };

    let (_, _) = secret_astruct.expose_secret(|exposed_secret| {
        let _ = ref_wrapper._inner.insert(*exposed_secret);
    });

    assert_eq!(ref_wrapper._inner.take().unwrap()._inner, 69);
}

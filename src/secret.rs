#![feature(generic_const_exprs)]
use std::{
    marker::PhantomData,
    mem::drop,
    ops::{Deref, Drop},
    ptr::NonNull,
    boxed::Box;
};
struct SecretBox<T> {
    pointer: NonNull<T>,
    _phantom: PhantomData<T>,
}

impl Drop for SecretBox<T> {
    fn drop(&mut self) {
        drop(self._phantom);
        
    }
}
struct Secret<T, const MEC: usize, const EC: usize = 0> {
    inner_box: SecretBox<T>,
}
struct SecretGuard<'a, T, const EC: usize> {
    inner_box: &'a SecretBox<T>,
}

impl<'a, T, const EC: usize> Deref for SecretGuard<'a, T, EC> {
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &T {
        unsafe { self.inner_box.pointer.as_ref() }
    }
}
impl<T, const MEC: usize> Secret<T, MEC> {
    #[inline(always)]
    pub fn new(value: T) -> Secret<T, MEC> {
        let sb = SecretBox {
            pointer: unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(value))) },
            _phantom: PhantomData,
        };
        Secret { inner_box: sb }
    }
}
impl<T, const MEC: usize, const EC: usize> Secret<T, MEC, EC> {
    const CHECK_EXPOSURE: () = assert!(EC < MEC, "Secret is over-exposed");
    #[inline(always)]
    #[must_use]
    pub fn expose_secret<'a, 'b: 'a>(
        &'b self,
    ) -> (Secret<T, MEC, { EC + 1 }>, SecretGuard<'a, T, { EC + 1 }>) {
        let _ = Self::CHECK_EXPOSURE;
        let next_secret = Secret {
            inner_box: SecretBox {
                pointer: unsafe { NonNull::new_unchecked(self.inner_box.pointer.as_ptr()) },
                _phantom: PhantomData,
            },
        };
        let secret_guard = SecretGuard {
            inner_box: &next_secret.inner_box,
        };
        (next_secret, secret_guard)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_one() {
        use super::*;

        struct UseSecret {
            inner: String,
        }
        impl UseSecret {
            fn new(value: String) -> Self {
                Self { inner: value }
            }
        }
        let new_secret: Secret<String, 2> = Secret::new("hello".into());
        let mut use_secret: UseSecret = UseSecret::new("".to_string());
        let new_secret = new_secret.expose_secret(|ref _secret_string_ref| {
            use_secret = UseSecret::new(_secret_string_ref.to_string());
        });
        let new_secret = new_secret.expose_secret(|ref _secret_string_ref| {
            use_secret = UseSecret::new(_secret_string_ref.to_string());
        });
        assert_eq!("hello".to_string(), *&use_secret.inner);
        assert_ne!("bye".to_string(), use_secret.inner);
        assert_eq!("hello".to_string(), use_secret.inner);
    }
}

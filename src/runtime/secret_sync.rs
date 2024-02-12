use core::sync::atomic::{AtomicUsize, Ordering};

pub struct RTSyncSecret<T, const MEC: usize>(T, AtomicUsize);

impl<T, const MEC: usize> RTSyncSecret<T, MEC> {
    pub const fn new(value: T) -> Self {
        Self(value, AtomicUsize::new(0))
    }

    pub fn new_with(f: impl FnOnce() -> T) -> Self {
        Self(f(), AtomicUsize::new(0))
    }

    pub fn exposure_count(&self) -> usize {
        self.1.load(Ordering::Relaxed)
    }
}

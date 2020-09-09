use std::sync::atomic::AtomicU64;

pub use std::sync::atomic::Ordering;
pub struct AtomicF64(AtomicU64);

impl AtomicF64 {
    /// FIXME should be const!
    pub fn new(x: f64) -> Self {
        AtomicF64(AtomicU64::new(x.to_bits()))
    }
    pub fn load(&self, order: Ordering) -> f64 {
        f64::from_bits(self.0.load(order))
    }
    pub fn store(&self, val: f64, order: Ordering) {
        self.0.store(val.to_bits(), order);
    }
    pub fn into_inner(self) -> f64 {
        f64::from_bits(self.0.into_inner())
    }

    /// Adds to the current value, returning the previous value.  This operation
    /// is *always* `Ordering::Relaxed`, and thus cannot be used to protect
    /// other memory or coordinate between threads.
    pub fn fetch_add(&self, val: f64) -> f64 {
        let mut old = self.load(Ordering::Relaxed);
        loop {
            let new = old + val;
            match self.compare_exchange_weak(old, new, Ordering::Relaxed, Ordering::SeqCst) {
                Ok(_) => return old,
                Err(x) => {
                    old = x;
                }
            }
        }
    }
    /// Subtracts from the current value, returning the previous value.  This operation
    /// is *always* `Ordering::Relaxed`, and thus cannot be used to protect
    /// other memory or coordinate between threads.
    pub fn fetch_sub(&self, val: f64) -> f64 {
        let mut old = self.load(Ordering::Relaxed);
        loop {
            let new = old - val;
            match self.compare_exchange_weak(old, new, Ordering::Relaxed, Ordering::SeqCst) {
                Ok(_) => return old,
                Err(x) => {
                    old = x;
                }
            }
        }
    }

    pub fn compare_exchange_weak(
        &self,
        current: f64,
        new: f64,
        success: Ordering,
        failure: Ordering,
    ) -> Result<f64, f64> {
        let c = current.to_bits();
        let n = new.to_bits();
        match self.0.compare_exchange_weak(c, n, success, failure) {
            Ok(x) => Ok(f64::from_bits(x)),
            Err(x) => Err(f64::from_bits(x))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{AtomicF64, Ordering};
    #[test]
    fn new_load() {
        assert_eq!(3.0, AtomicF64::new(3.0).load(Ordering::Relaxed));
    }
}

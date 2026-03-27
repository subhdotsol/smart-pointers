use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        // SAFETY: we know no one else is concurrently mutating self.value (because !Sync)
        // SAFETY: we know we're not invalidating any references because we never give any out
        unsafe {
            *self.value.get() = value;
        }
    }
}

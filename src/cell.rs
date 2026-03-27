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

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        // SAFETY: we know no one else is modifying this value since only this thread can mutate
        // because !Sync and it is the thread executing this function.
        unsafe { *self.value.get() }
    }
}

#[cfg(test)]
mod test {
    use super::Cell;

    #[test]
    fn cell_basic() {
        let c = Cell::new(42);
        assert_eq!(c.get(), 42);
        c.set(43);
        assert_eq!(c.get(), 43);
    }

    /* 
    // FAILING TEST: Cell<T> is !Sync because it contains UnsafeCell<T>.
    // This should not compile if uncommented.
    #[test]
    fn cell_sync_fail() {
        use std::sync::Arc;
        let x = Arc::new(Cell::new(42));
        let x1 = Arc::clone(&x);
        std::thread::spawn(move || {
            x1.set(43);
        });
    }
    */
}

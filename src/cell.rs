//! A simple implementation of Cell
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

    /*
    // FAILING TEST: Cell doesn'\''t let you get references to its interior.
    // This is because set() could invalidate them.
    #[test]
    fn cell_reference_fail() {
        let x = Cell::new(vec![42]);
        // This wouldn'\''t even compile because Cell::get returns a Copy of the value,
        // and Vec is not Copy. If we had a way to get a reference:
        // let first = &x.get_ref()[0];
        // x.set(vec![]); // This would make '\''first'\'' a dangling pointer.
    }
    */
}

// Example :
// struct Counter {
//     value: Cell<i32>,
// }

// impl Counter {
//     fn new() -> Self {
//         Counter {
//             value: Cell::new(0),
//         }
//     }

//     fn increment(&self) {
//         let v = self.value.get();
//         self.value.set(v + 1);
//     }

//     fn get(&self) -> i32 {
//         self.value.get()
//     }
// }

// fn main() {
//     let counter = Counter::new();

//     counter.increment();
//     counter.increment();

//     println!("{}", counter.get()); // 2
// }

use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

//! A simple implementation of RefCell
use crate::cell::Cell;
use std::cell::UnsafeCell;

#[derive(Copy, Clone)]
enum RefState {
    Unshared,
    Shared(usize),
    Exclusive,
}

pub struct RefCell<T> {
    value: UnsafeCell<T>,
    state: Cell<RefState>,
}

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            state: Cell::new(RefState::Unshared),
        }
    }

    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.state.get() {
            RefState::Unshared => {
                self.state.set(RefState::Shared(1));
                Some(Ref { refcell: self })
            }
            RefState::Shared(n) => {
                self.state.set(RefState::Shared(n + 1));
                Some(Ref { refcell: self })
            }
            RefState::Exclusive => None,
        }
    }

    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        if let RefState::Unshared = self.state.get() {
            self.state.set(RefState::Exclusive);
            Some(RefMut { refcell: self })
        } else {
            None
        }
    }
}

pub struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> std::ops::Deref for Ref<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Exclusive | RefState::Unshared => unreachable!(),
            RefState::Shared(1) => {
                self.refcell.state.set(RefState::Unshared);
            }
            RefState::Shared(n) => {
                self.refcell.state.set(RefState::Shared(n - 1));
            }
        }
    }
}

pub struct RefMut<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> std::ops::Deref for RefMut<'_, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> std::ops::DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.refcell.value.get() }
    }
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefState::Shared(_) | RefState::Unshared => unreachable!(),
            RefState::Exclusive => {
                self.refcell.state.set(RefState::Unshared);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::RefCell;

    #[test]
    fn refcell_basic() {
        let rc = RefCell::new(42);
        assert_eq!(*rc.borrow().unwrap(), 42);

        {
            let mut b = rc.borrow_mut().unwrap();
            *b = 43;
        }

        assert_eq!(*rc.borrow().unwrap(), 43);
    }

    #[test]
    fn refcell_multiple_borrow() {
        let rc = RefCell::new(42);
        let b1 = rc.borrow().unwrap();
        let b2 = rc.borrow().unwrap();
        assert_eq!(*b1, 42);
        assert_eq!(*b2, 42);
    }

    #[test]
    fn refcell_exclusive_rule() {
        let rc = RefCell::new(42);
        let _b1 = rc.borrow().unwrap();
        assert!(rc.borrow_mut().is_none());
    }
}

/*
// FAILING TEST: Multiple borrow_mut() calls.
// This will return None, failing the unwrap().
#[test]
fn refcell_multiple_borrow_mut_fail() {
    let rc = RefCell::new(42);
    let _bm1 = rc.borrow_mut().unwrap();
    let _bm2 = rc.borrow_mut().unwrap(); // This will panic if unwrapped.
}
*/

/*
// FAILING TEST: borrow_mut() while borrow() is active.
// This will return None, failing the unwrap().
#[test]
fn refcell_borrow_mut_while_shared_fail() {
    let rc = RefCell::new(42);
    let _b1 = rc.borrow().unwrap();
    let _bm1 = rc.borrow_mut().unwrap(); // This will panic if unwrapped.
}
*/

// example :

// fn main() {
//     // 1. Create RefCell
//     let rc = RefCell::new(42);

//     // 2. Immutable borrows (multiple allowed)
//     let b1 = rc.borrow().unwrap();
//     let b2 = rc.borrow().unwrap();

//     println!("b1 = {}, b2 = {}", *b1, *b2);

//     // Cannot mutably borrow while shared borrows exist
//     if rc.borrow_mut().is_none() {
//         println!("Cannot borrow_mut while shared borrows exist");
//     }

//     // Drop shared borrows
//     drop(b1);
//     drop(b2);

//     // 3. Mutable borrow (exclusive)
//     {
//         let mut bm = rc.borrow_mut().unwrap();
//         *bm += 1;
//         println!("Mutated value = {}", *bm);

//         // Cannot borrow again while mutable borrow is active
//         if rc.borrow().is_none() {
//             println!("Cannot borrow while mutable borrow exists");
//         }
//     } // <-- bm dropped here, state resets

//     // 4. Borrow again after mutable borrow is dropped
//     let b3 = rc.borrow().unwrap();
//     println!("Final value = {}", *b3);

//     // --------------------------------------------------
//     // 5. Real-world style usage
//     #[derive(Debug)]
//     struct Counter {
//         value: RefCell<i32>,
//     }

//     impl Counter {
//         fn new(v: i32) -> Self {
//             Self {
//                 value: RefCell::new(v),
//             }
//         }

//         fn increment(&self) {
//             *self.value.borrow_mut().unwrap() += 1;
//         }

//         fn get(&self) -> i32 {
//             *self.value.borrow().unwrap()
//         }
//     }

//     let counter = Counter::new(10);

//     counter.increment();
//     counter.increment();

//     println!("Counter value = {}", counter.get());

//     // --------------------------------------------------
//     // 6. Demonstrating runtime borrow rules clearly

//     let data = RefCell::new(100);

//     let r1 = data.borrow().unwrap();
//     println!("r1 = {}", *r1);

//     // This will fail (returns None)
//     if data.borrow_mut().is_none() {
//         println!("Runtime check: cannot mutably borrow while shared exists");
//     }

//     drop(r1);

//     let mut r2 = data.borrow_mut().unwrap();
//     *r2 += 50;
//     println!("r2 after mutation = {}", *r2);

//     // This will fail (returns None)
//     if data.borrow().is_none() {
//         println!("Runtime check: cannot borrow while mutable exists");
//     }

//     drop(r2);

//     let r3 = data.borrow().unwrap();
//     println!("r3 final = {}", *r3);

//     println!("Program completed successfully");
// }

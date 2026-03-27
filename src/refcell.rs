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

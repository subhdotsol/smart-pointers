//! A simple implementation of Rc
use crate::cell::Cell;
use std::marker::PhantomData;
use std::ptr::NonNull;

struct RcInner<T> {
    value: T,
    refcount: Cell<usize>,
}

pub struct Rc<T> {
    inner: NonNull<RcInner<T>>,
    _marker: PhantomData<RcInner<T>>,
}

impl<T> Rc<T> {
    pub fn new(v: T) -> Self {
        let inner = Box::new(RcInner {
            value: v,
            refcount: Cell::new(1),
        });

        Rc {
            // SAFETY: Box does not give us a null pointer.
            inner: unsafe { NonNull::new_unchecked(Box::into_raw(inner)) },
            _marker: PhantomData,
        }
    }
}

impl<T> std::ops::Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        // SAFETY: self.inner is a Box that is only deallocated when the last Rc goes away.
        // we have an Rc, therefore the Box has not been deallocated, so deref is fine.
        &unsafe { self.inner.as_ref() }.value
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();
        inner.refcount.set(c + 1);
        Rc {
            inner: self.inner,
            _marker: PhantomData,
        }
    }
}

// TODO: #[may_dangle]
impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.refcount.get();
        if c == 1 {
            // SAFETY: we are the _only_ Rc left, and we are being dropped.
            // therefore, after us, there will be no Rc's, and no references to T.
            let _ = unsafe { Box::from_raw(self.inner.as_ptr()) };
        } else {
            // there are other Rcs, so don't drop the Box!
            inner.refcount.set(c - 1);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Rc;

    #[test]
    fn test_rc_new() {
        let _rc = Rc::new(5);
    }

    #[test]
    fn test_rc_clone() {
        let rc1 = Rc::new(5);
        let rc2 = rc1.clone();
        assert_eq!(*rc1, *rc2);
    }

    #[test]
    fn test_rc_drop() {
        let rc1 = Rc::new(5);
        let rc2 = rc1.clone();
        drop(rc1);
        drop(rc2);
    }
}

// Example :
// fn main() {
//     // 1. Create a new Rc
//     let mut a = Rc::new(10);
//     println!("Initial value: {}", *a);

//     // 2. Clone it (increase refcount)
//     let b = a.clone();
//     let c = b.clone();

//     println!("After cloning:");
//     println!("a = {}, b = {}, c = {}", *a, *b, *c);

//     // 3. Check reference count
//     println!("Refcount = {}", Rc::strong_count(&a));

//     // 4. Try mutable access (should fail because count > 1)
//     if let Some(val) = Rc::get_mut(&mut a) {
//         *val = 20;
//     } else {
//         println!("Cannot mutate: multiple owners exist");
//     }

//     // 5. Drop clones
//     drop(b);
//     drop(c);

//     println!("After dropping b and c:");
//     println!("Refcount = {}", Rc::strong_count(&a));

//     // 6. Now mutation should work
//     if let Some(val) = Rc::get_mut(&mut a) {
//         *val = 30;
//         println!("Mutated value: {}", *a);
//     }

//     // 7. Try to unwrap (take ownership)
//     match Rc::try_unwrap(a) {
//         Ok(value) => {
//             println!("Successfully unwrapped Rc, value = {}", value);
//         }
//         Err(_) => {
//             println!("Failed to unwrap Rc (multiple owners exist)");
//         }
//     }

//     // --------------------------------------------------
//     // 8. Real use-case: shared data in a structure
//     #[derive(Debug)]
//     struct Node {
//         value: i32,
//         next: Option<Rc<Node>>,
//     }

//     let n1 = Rc::new(Node { value: 1, next: None });
//     let n2 = Rc::new(Node {
//         value: 2,
//         next: Some(n1.clone()),
//     });

//     println!("n1 value = {}", n1.value);
//     println!("n2 value = {}", n2.value);

//     println!(
//         "n1 refcount after sharing = {}",
//         Rc::strong_count(&n1)
//     );

//     // 9. Demonstrating drop behavior
//     {
//         let temp = n1.clone();
//         println!(
//             "n1 refcount inside scope = {}",
//             Rc::strong_count(&n1)
//         );
//         drop(temp);
//     }

//     println!(
//         "n1 refcount after scope = {}",
//         Rc::strong_count(&n1)
//     );

//     println!("Program completed without leaks (no cycles created)");
// }

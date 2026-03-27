use crate::cell::Cell;

struct RcInner<T> {
    value: T,
    refcount: Cell<usize>,
}

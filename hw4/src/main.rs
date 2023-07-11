use std::ptr;


fn main() {}
struct Victor<T> {
    ptr: *mut T,
    len: usize,
    capacity: usize
}
impl<T> Victor<T> {
    fn new() -> Self {
        Victor {
            ptr: ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }
    fn with_capacity() {}
    fn push() {}
    fn pop() {}
    fn remove() {}
    fn get() {}
    fn resize() {}
}
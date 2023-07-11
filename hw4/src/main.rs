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
            ptr: std::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }
}
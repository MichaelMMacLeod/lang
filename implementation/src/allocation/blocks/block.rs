use std::ptr::NonNull;

pub trait Block {
    fn non_null_ptr(&self) -> NonNull<[u8]>;

    fn ptr(&self) -> *mut [u8] {
        self.non_null_ptr().as_ptr()
    }

    fn start_ptr(&self) -> *mut u8 {
        self.ptr() as *mut u8
    }

    fn len(&self) -> usize {
        self.non_null_ptr().len()
    }
}
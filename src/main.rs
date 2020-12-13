use libc::{c_void, size_t};

#[link(name = "clib")]
extern "C" {
    fn heapalloc(size: size_t) -> *mut c_void; // return wrapped type
    fn heapfree(ptr: *mut c_void);
    fn rdtsc() -> u64; // allowed to return Rust's type here
}

struct Mem<'a>(&'a mut [u8]);

impl<'a> Mem<'a> {
    fn new(size: usize) -> Mem<'a> {
        unsafe {
            let ptr = heapalloc(size as size_t);
            Mem(std::slice::from_raw_parts_mut(ptr as *mut u8, size))
        }
    }
}

impl<'a> Drop for Mem<'a> {
    fn drop(&mut self) {
        unsafe {
            heapfree(self.0 as *mut _ as *mut c_void);
        }
    }
}

fn main() {
    let m = Mem::new(16);
    for b in m.0.iter() {
        print!("{:02x} ", b);
    }
    println!();
}

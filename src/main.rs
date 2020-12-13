use libc::{c_void, size_t};
use rand::Rng;

#[link(name = "alloc")]
extern "C" {
    fn heapalloc(size: size_t) -> *mut c_void; // return wrapped type
    fn heapfree(ptr: *mut c_void);
    fn rdtsc() -> u64; // allowed to return Rust's type here
}

struct Cycles(u64);

impl Cycles {
    fn start() -> Cycles {
        let start = unsafe { rdtsc() };
        Cycles(start)
    }

    fn stop(self) -> u64 {
        let Cycles(start) = self;
        unsafe { rdtsc() - start }
    }
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
    let mut blocks: Vec<Mem> = Vec::new(); // store all allocated memory blocks
    let mut cycles: Vec<u64> = vec![0; 14]; // store accumulated time
    let mut count: Vec<u64> = vec![0; 14]; // # of blocks of particular size
    let mut rng = rand::thread_rng();
    for _ in 0..100_000 {
        // repeat for reliable measurement
        let exponent = rng.gen_range(4, 14);
        let blocksize = 2usize.pow(exponent as u32); // blocksize between 16 bytes and 8K
        let c = Cycles::start(); // start the clock
        let block = Mem::new(blocksize); // request allocation

        // now, trick the compiler and actually force allocation by
        // accessing the memory block and printing its content
        // however, printing is slow and we actually do not want to do it
        // (but the compiler cannot know: we compare against a random number)
        if exponent == 100 {
            for byte in block.0.iter() {
                print!("{:02x} ", byte);
            }
        }
        count[exponent] += 1; // record count for block-size
        cycles[exponent] += c.stop(); // record run-time
        blocks.push(block); // store block
    }
    for i in 4..14 {
        println!("{}: {}", i, cycles[i] / count[i]); // pseudo-code, understood point-wise
    }
}

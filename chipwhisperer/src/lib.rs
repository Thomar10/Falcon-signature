#![no_std]
#![no_main]

//extern crate alloc;

use cortex_m as _;
use embedded_alloc::Heap;

use core::panic::PanicInfo;
use core;
//use core::alloc::{GlobalAlloc, Layout, System};

use falcon::keygen::keygen;
use falcon::shake::{i_shake256_init, i_shake256_inject, InnerShake256Context};

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

/*struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        System.alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout)
    }
}*/
/*
#[global_allocator]
static GLOBAL: MyAllocator = MyAllocator;*/

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[no_mangle]
pub extern "C" fn init_heap() -> (){
    // Initialize the allocator BEFORE you use it
    {
        use core::mem::MaybeUninit;
        const HEAP_SIZE: usize = 1024;
        static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
        unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
    }
}

/*#[no_mangle]
pub extern fn main(argc: i32, argv: *const *const u8) -> i32 {
    return 0;
}*/

#[no_mangle]
pub extern "C" fn rust_test(left: u32, right: u32) -> u32 {
    let mut result: u32 = 0;
    //let mut h: [u16; 1024] = [0; 1024];

    for _ in 0..50 {
        result += left + right;
        //h[0] = result as u16;
    }

    return result;
}

/*#[no_mangle]
pub extern "C" fn rust_genkey(input_ptr: *const u8, input_len: u32){

    let input: &[u8];

    unsafe {
        input = core::slice::from_raw_parts(input_ptr, input_len as usize);
    }

    const LOGN: usize = 10;
    const BUFFER_SIZE: usize = 8192 * 8;
    let mut h: [u16; 1024] = [0; 1024];
    let mut f: [i8; 1024] = [0; 1024];
    let mut g: [i8; 1024] = [0; 1024];
    let mut F: [i8; 1024] = [0; 1024];
    let mut G: [i8; 1024] = [0; 1024];

    let mut tmp_keygen: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    let mut rng_rust: InnerShake256Context = gen_rng(input);
    keygen(&mut rng_rust, &mut f, &mut g, &mut F, &mut G, &mut h, LOGN as u32, &mut tmp_keygen);
}*/

#[no_mangle]
pub extern "C" fn rust_genkey() -> u32{

    let input: [u8; 6] = [1, 2, 3, 4, 5, 6];

    const LOGN: usize = 10;
    const BUFFER_SIZE: usize = 8192 * 8;
    let mut h: [u16; 1024] = [0; 1024];
    let mut f: [i8; 1024] = [0; 1024];
    let mut g: [i8; 1024] = [0; 1024];
    let mut F: [i8; 1024] = [0; 1024];
    let mut G: [i8; 1024] = [0; 1024];

    let mut tmp_keygen: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    let mut rng_rust: InnerShake256Context = gen_rng(&input);
    keygen(&mut rng_rust, &mut f, &mut g, &mut F, &mut G, &mut h, LOGN as u32, &mut tmp_keygen);
    return h[0] as u32;
}

fn gen_rng(input: &[u8]) -> InnerShake256Context {
    let state: [u64; 25] = [0; 25];
    let dptr: u64 = 0;
    let mut sc_rust = InnerShake256Context { st: state, dptr};
    i_shake256_init(&mut sc_rust);
    i_shake256_inject(&mut sc_rust, &input);
    return sc_rust;
}

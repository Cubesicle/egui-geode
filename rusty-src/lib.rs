mod cpp_ffi_wrapper;

use cpp_ffi_wrapper::log_info;

#[no_mangle]
pub extern "C" fn bingus(this_ptr: isize, fn_ptr: isize) {
    println!("Hello from rust!! >:) {:#06x} {:#06x}", this_ptr, fn_ptr);
    log_info(format!("Yippee!!"));
    unsafe { std::mem::transmute::<_, fn(isize)>(fn_ptr)(this_ptr); }
}
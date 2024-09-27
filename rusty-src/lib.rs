mod geode;

use geode::log;

#[no_mangle]
pub extern "C" fn bingus(this_ptr: isize, fn_ptr: isize) {
    println!("Hello from rust!! >:) {:#06x} {:#06x}", this_ptr, fn_ptr);
    log::debug(format!("Yippee!!"));
    log::info(format!("Yippee!!"));
    log::warn(format!("Yippee!!"));
    log::error(format!("Yippee!!"));
    unsafe { std::mem::transmute::<_, fn(isize)>(fn_ptr)(this_ptr); }
}
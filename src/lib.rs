//#[no_mangle]
//pub extern "C" fn peepeepoopoo(callback: fn()) {
//    println!("Hello from rust!! >:)");
//    callback();
//}

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn bingus();
    }
}

#[cxx::bridge(namespace = "geode::log")]
mod ffi2 {
    unsafe extern "C++" {
        include!("Geode/loader/Log.hpp");
        fn info(str: &str);
    }
}

pub fn bingus() {
}
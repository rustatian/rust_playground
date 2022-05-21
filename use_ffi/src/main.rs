extern crate libloading;

use libloading::{Library, Symbol};

// #[link(name = "ffi")]
// extern {
//     fn foo() -> i16;
// }

type AddFunc = unsafe fn() -> i16;

fn main() {
    let lib = unsafe { Library::new("libffi.so") }.unwrap();

    unsafe {
        let ff: Symbol<AddFunc> = lib.get(b"foo").unwrap();
        let answer = ff();
        println!("{}", answer);
    }

    lib.close().unwrap();

}

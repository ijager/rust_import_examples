extern crate somepackage;

use somepackage::indirect_access;
use somepackage::somemod;

// import from other local file
mod otherfile;

// import c code
extern { fn c_function_example(); }

use otherfile::mod_in_otherfile;

fn main() {
    println!("Hello, world!");
    somepackage::public_function();
    indirect_access();
    somemod::public_mod_function();

    otherfile::fn_from_otherfile();
    mod_in_otherfile::some_func();

    unsafe {
        c_function_example();
    }
}

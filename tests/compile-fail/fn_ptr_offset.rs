// Validation makes this fail in the wrong place
// compile-flags: -Zmir-emit-validate=0

use std::mem;

fn f() {}

fn main() {
    let x : fn() = f;
    let y : *mut u8 = unsafe { mem::transmute(x) };
    let y = y.wrapping_offset(1);
    let x : fn() = unsafe { mem::transmute(y) };
    x(); //~ ERROR constant evaluation error
    //~^ NOTE tried to use a function pointer after offsetting it
}

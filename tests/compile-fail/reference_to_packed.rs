// This should fail even without validation
// compile-flags: -Zmir-emit-validate=0

#![allow(dead_code, unused_variables)]

#[repr(packed)]
struct Foo {
    x: i32,
    y: i32,
}

fn main() {
    let foo = Foo {
        x: 42,
        y: 99,
    };
    let p = unsafe { &foo.x };
    let i = *p; //~ ERROR constant evaluation error
    //~^ NOTE tried to access memory with alignment 1, but alignment 4 is required
}

fn f() {}

fn main() {
    let x: i32 = unsafe {
        *std::mem::transmute::<fn(), *const i32>(f) //~ ERROR constant evaluation error: tried to dereference a function pointer
    };
    panic!("this should never print: {}", x);
}

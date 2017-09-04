// FIXME: investigate again once #296 is fixed
// compile-flags: -Zmir-emit-validate=0

#![feature(advanced_slice_patterns)]
#![feature(slice_patterns)]

fn bar(a: &'static str, b: &'static str) -> [&'static str; 4] {
    [a, b, b, a]
}

fn main() {
    let out = bar("baz", "foo");
    let [a, xs.., d] = out;
    assert_eq!(a, "baz");
    assert_eq!(xs, ["foo", "foo"]);
    assert_eq!(d, "baz");
}
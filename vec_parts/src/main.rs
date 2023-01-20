#![feature(vec_into_raw_parts)]

// I was TRYING to create a test that would exhaust memory
// by leaking it from Vecs broken into_raw_parts, but it does
// NOT exhaust memory.

struct Foo {
    vec: Vec<u64>
}

impl Foo {
    fn new() -> Self {
        Self { vec: Vec::with_capacity(1_000_000_000) }
    }
}

fn mk_a_foo() {
    let mut foo = Foo::new();
    foo.vec.push(3);
    let (ptr, len, cap) = foo.vec.into_raw_parts();
    let _vec = unsafe { Vec::from_raw_parts(ptr, len, cap) };
}
fn main() {
    println!("Start");
    for _ in 0..1_000_000 {
        mk_a_foo();
    }
    println!("End");
}

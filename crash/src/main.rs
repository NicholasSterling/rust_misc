fn main() {
    println!("Hello, world!");
    foo();
}

fn foo() {
    bar();
}

fn bar() {
    assert_eq!(1,2);
}

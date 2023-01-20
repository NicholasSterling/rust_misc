
pub trait Mark {}

impl Mark for i32 {}

trait Bar {
    type T;
    fn bar(&self) -> Self::T;
}

impl<T: Into<i32> + Mark> Bar for T {
    type T = usize;
    fn bar(&self) -> Self::T { 7usize }
}

// impl Bar for i32 {
//     type T = usize;
//     fn bar(&self) -> Self::T { 7usize }
// }

impl Bar for char {
    type T = (usize, usize);
    fn bar(&self) -> Self::T { (3usize, 4usize) }
}

impl Bar for &str {
    type T = usize;
    fn bar(&self) -> Self::T { 5usize }
}


// This fails with "conflicting implementations of trait Bar for type i32.
// See https://stackoverflow.com/a/39161143/464309
impl<'a, I> Bar for I where I: Iterator<Item = &'a str> {
    type T = usize;
    fn bar(&self) -> Self::T { 1 }
}

fn main() {
    dbg!(7.bar());
    dbg!('A'.bar());
    dbg!("foo".bar());
}

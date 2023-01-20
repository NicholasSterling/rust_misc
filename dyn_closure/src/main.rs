use std::fmt::Display;

fn main() {

    let u = 2;

    let a = 7;
    let b = 3.2;
    let x: &dyn Display = if u <= 1 {&a as &dyn Display} else {&b as &dyn Display};
    println!("{x}");

    let y = 7;
    //type A = Box<dyn Fn(&Image, &mut Bkg) -> ()>;
    type F = Box<dyn Fn(i32) -> i32>;
    let foo: F = match u {
        1 => Box::new(move |x| x + y + 1),
        2 => Box::new(move |x| x + y + 2),
        _ => Box::new(move |x| x + y + 3)
    };
    dbg!(foo(9));
}

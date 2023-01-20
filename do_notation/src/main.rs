use do_notation::m;

fn f(n: i32) -> Option<i32> {
    if n == 0 {
        None
    } else {
        Some(7/n)
    }
}

fn main() {
    let r1 = m! {
        a <- f(1);
        b <- f(2);
        c <- f(3);
        return Some(a + b + c)
    };
    let r2 = m! {
        a <- f(1);
        b <- f(0);
        c <- f(3);
        return Some(a + b + c)
    };
    assert_eq!(r1, Some(12));
    assert_eq!(r2, None);
}

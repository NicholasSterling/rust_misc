fn main() {
    dbg!(fib().take(7).collect::<Vec<_>>());
    dbg!(fob().take(7).collect::<Vec<_>>());
    // dbg!(itertools::iterate((1,1), |&(a,b)| (b, a+b)).map(|p| p.0).take(6).collect::<Vec<_>>());
}

// Returns an Iterator for the Fibonacci sequence: 1 1 2 3 5 8 ...
fn fib() -> impl Iterator<Item = i32> {
    // iterize((1,1), |p| (p.1, p.0 + p.1))
    iterize((1,1), |(a,b)| (b, a+b))
}

fn fob() -> impl Iterator<Item = i32> {
    iterize((1,0), |(n,r)|
        if r == 0 {
            (n+1, n)
        } else {
            (n, r-1)
        }
    )
}

// Produces an Iterator by induction.
// Given an initial state of type (R,S) and a function that produces
// the next state from an existing state, we return an Iterator for the Rs.
// So in (R,S), R is the part that gets (R)eturned by the Iterator,
// and S is any additional (S)tate used internally.
pub fn iterize<R: Copy, S: Copy, F>(s0: (R,S), f: F) -> impl Iterator<Item = R>
where F: Fn((R,S)) -> (R,S)
{
    let mut state = s0;
    std::iter::repeat_with(
        move || { state.swap(f(state)).0 }
    )
}

// a.swap(b) sets a to b and returns the old value of a.
pub trait Swap: Sized {
    fn swap(&mut self, value: Self) -> Self;
}
impl<T> Swap for T {
    fn swap(&mut self, new: Self) -> Self {
        std::mem::replace(self, new)
    }
}
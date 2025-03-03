#![feature(generic_const_exprs)]
#![allow(incomplete_features, unused)]

// NOTE
// The 'fixed' crate gives you types like
// FixedI32<U4>
// which has alias I28F4
// https://docs.rs/fixed/latest/fixed/

// An integer with N bits for a fractional part.
// e.g. Fixed::<4>(3) is 3/16 (2^4 = 16)
struct Fixed<const N: i32>(i32);

// The result of a multiply has M + N fractional part bits.
fn mul<const M: i32, const N: i32>(a: Fixed<M>, b: Fixed<N>) -> Fixed<{ M + N }> {
    Fixed(a.0 * b.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = Fixed::<2>(17);
        let b = Fixed::<3>(9);
        assert_eq!(mul(a, b).0, 153);
    }
}

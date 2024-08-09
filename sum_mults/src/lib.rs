#![warn(clippy::pedantic)]

use num::{Unsigned, Integer};
use std::cmp::PartialOrd;
use std::fmt::Debug;

pub fn sum_multiples<T>(limit: T, factors: &[T]) -> T
where T: Unsigned + Integer + PartialOrd + Copy + Debug
{
  // dbg!(&factors);

  // The constants 0, 1, and 2 in whatever unsigned integer type we are using.
  let c0 = T::zero();
  let c1 = T::one();
  let c2 = c1 + c1;

  let mut sum = c0;
    
  for (i, factor) in factors.iter().enumerate() {

      //dbg!(&factor);
      let n = limit / *factor;  // # of multiples of factor to sum
      let sum_of_multiples_of_factor = *factor * (
        // n * (n+1) might overflow, so do the /2 first, to the even number.
        if n.is_odd() {
          (n+c1)/c2 * n
        } else {
          n/c2 * (n+c1)
        }
      );
      let new_factors: Vec<_> = factors[..i].iter()
        .map(|&prev_factor| prev_factor.lcm(factor))
        .collect();
      //dbg!(&new_factors);
      let sum_of_previously_seen_multiples_of_factor =
        sum_multiples(limit, &new_factors[..]);  // <-- RECURSION

      sum = sum
        + sum_of_multiples_of_factor
        - sum_of_previously_seen_multiples_of_factor;
  }

  sum

}


#[cfg(test)]
mod tests {

    use super::sum_multiples;

    // A slow but known-good version of the same function.
    pub fn slow_reliable(limit: u64, factors: &[u64]) -> u64 {
        (1..=limit).filter( |&i|
            factors.iter().any(|&j| i % j == 0)
        ).sum()
    }

    #[test]
    fn no_factors() {
      let limit = 1_000_000;
      let factors = [];
      let ours = sum_multiples(limit, &factors);
      let good = slow_reliable(limit, &factors);
      assert_eq!(ours, good);
    }

    #[test]
    fn limit_0() {
      let limit = 0;
      let factors = [3,5,7];
      let ours = sum_multiples(limit, &factors);
      let good = slow_reliable(limit, &factors);
      assert_eq!(ours, good);
    }

    #[test]
    fn manual_test() {
      let limit = 1_000_000;
      //let factors = [3,6];
      // let factors = [6,10,15,3];
      let factors = [6,10,15,7,3,42,63,70];
      // let factors = [30,30];
      let ours = sum_multiples(limit, &factors);
      let good = slow_reliable(limit, &factors);
      assert_eq!(ours, good);
    }

    #[test]
    fn show_result() {
      let limit = 20u8;
      let factors = [3,5,15,30];
      let ours = sum_multiples(limit, &factors);
      dbg!(ours);
    }

    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_result_matches(limit in 1..100_000u64, factors in proptest::collection::vec(1..50u64, 1..10)) {
            let ours = sum_multiples(limit, &factors[..]);
            let good = slow_reliable(limit, &factors[..]);
            prop_assert_eq!(ours, good);
        }
    }

}

#![warn(clippy::pedantic)]

// TODO:
// This version is recursive, and not tail-recursive.  Is that OK?
// It is u64-only, and needs to be genericized over unsigned integers.

pub fn sum_multiples(limit: u64, factors: &[u64]) -> u64 {

  dbg!(&factors);

  // Returns the lowest common multiple of a and b.
  fn lcm(a: u64, b: u64) -> u64 {
    a*b / gcd(a,b)
  }

  // Returns the greatest common divisor of a and b.
  fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {a} else { gcd(b, a%b) }
  }

  fn sum_from_ix(i: usize, limit: u64, factors: &[u64]) -> u64 {
    if i == factors.len() {  // we've processed all factors
      0
    } else {
      let factor = factors[i];
      //dbg!(&factor);
      let n = limit / factor;  // # of multiples of factor to sum
      // Returns the sum of the integers from 1 to n.
      let sum_1_to = |n|
        // n * (n+1) might overflow, so do the /2 first, to the even number.
        if n&1 != 0 {
          (n+1)/2 * n  // n is odd
        } else {
          n/2 * (n+1)  // n is even
        };
      let sum_of_multiples_of_factor = factor * sum_1_to(n);
      let new_factors: Vec<_> = factors[..i].iter()
        .map(|&prev_factor| lcm(prev_factor, factor))
        .filter(|&new_factor| new_factor <= limit)
        .collect();
      //dbg!(&new_factors);
      let sum_of_previously_seen_multiples_of_factor =
        sum_from_ix(0, limit, &new_factors[..]);  // <-- RECURSION
      let sum_of_multiples_of_rest_of_factors =
        sum_from_ix(i+1, limit, factors);                 // <-- RECURSION
      sum_of_multiples_of_factor
        - sum_of_previously_seen_multiples_of_factor
        + sum_of_multiples_of_rest_of_factors
    }
  }

  sum_from_ix(0, limit, factors)

}

      /*
      if factors[0..i].iter().any(|prev_factor| factor % prev_factor == 0) {
        dbg!("skipped because factor is divisible by one of these");
        dbg!(&factors[0..i]);
        0
     */

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

pub fn fast_sum_of_multiples(limit: u64, factors: &[u64]) -> u64 {
  fn lcm(a: u64, b: u64) -> u64 { a*b / gcd(a,b) }
  fn gcd(a: u64, b: u64) -> u64 { if b == 0 {a} else { gcd(b, a%b) } }
  fn sum_from_ix(i: usize, limit: u64, factors: &[u64]) -> u64 {
    if i == factors.len() {  // we've processed all factors
      0
    } else {
      let factor = factors[i];
      let n = limit / factor;  // # of multiples of factor to sum
      let sum_of_multiples_of_factor = factor * (n*(n+1)/2);
      let new_factors: Vec<_> = factors[..i].iter()
        .map(|&prev_factor| lcm(prev_factor, factor))
        .filter(|&factor| factor <= limit)
        .collect();
      let sum_of_previously_seen_multiples_of_factor =
        sum_from_ix(0, limit, &new_factors[..]);
      let sum_of_multiples_of_rest_of_factors =
        sum_from_ix(i+1, limit, factors);
      sum_of_multiples_of_factor
        - sum_of_previously_seen_multiples_of_factor
        + sum_of_multiples_of_rest_of_factors
    }
  };
  sum_from_ix(0, limit, factors)
}

#[cfg(test)]
mod tests {

    pub fn naive_sum(limit: u32, factors: &[u32]) -> u32 {
        (1..=limit).filter( |&i|
            factors.iter().any(|&j| i % j == 0)
        ).sum()
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

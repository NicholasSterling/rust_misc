// gradual_math/int_sqrt/floor

use std::ops::{Add, Sub, AddAssign, SubAssign, Mul};

struct Calculator<S,T> {
    sqrt: S,
    lo: T,
    hi: T,
}

impl<S,T> Calculator<S,T>
where S: Copy + Ord + Add<Output = S> + Sub<Output = S> + AddAssign + SubAssign + From<u8> + Into<T>,
      T: Copy + Ord + Add<Output = T> + Sub<Output = T> + AddAssign + SubAssign + From<u8> + Mul<Output = T>,
{
    pub fn with_init_sqrt(sqrt: S) -> Calculator<S,T> {
        let s: T = sqrt.into();
        let lo = s * s;
        let hi = lo + s + s + 1.into();
        Calculator { sqrt, lo, hi }
    }

    fn increment(&mut self) {
        self.sqrt += 1.into();
        let s: T = self.sqrt.into();
        self.lo = self.hi + 1.into();
        self.hi = self.lo + s + s;
    }

    fn decrement(&mut self) {
        self.sqrt -= 1.into();
        let s: T = self.sqrt.into();
        self.lo = self.hi - 1.into();
        self.hi = self.lo - s - s;
    }

    pub fn sqrt_at(&mut self, t: T) -> S {
        if self.hi < t {
            loop {
                self.increment();
                if self.hi >= t {
                    break;
                }
            }
        } else {
            while self.lo > t {
                self.decrement();
            }
        }
        self.sqrt
    }

    pub fn sqrt_at_higher(&mut self, t: T) -> S {
        while self.hi < t {
            self.increment();
        }
        self.sqrt
    }

    pub fn sqrt_at_lower(&mut self, t: T) -> S {
        while self.lo > t {
            self.decrement();
        }
        self.sqrt
    }

    pub fn lo(&self) -> T {
        self.lo
    }

    pub fn hi(&self) -> T {
        self.hi
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u8() {
        // let calc = Calculator<u8,u16>::with_init_sqrt(0);
        let mut calc: Calculator<u8,u16> = Calculator::with_init_sqrt(0);
        let result: Vec<u8> = (0u16..16)
            .chain((0u16..16).rev())
            //.map(calc.sqrt_at )
            .map(|x| calc.sqrt_at(x) )
            .collect();
        let expected: Vec<u8> = vec![
            // 0 1 2 3 4 5 6 7 8 9 9 8 7 6 5 4 3 2 1 0
            0,1,1,2,2,2,2,3,3,3,3,3,3,2,2,2,2,1,1,0
        ];
        assert_eq!(result, expected);
    }

}

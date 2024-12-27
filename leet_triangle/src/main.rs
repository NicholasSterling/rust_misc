
// Calls the specified function with pair windows where the
// second is mutable, e.g. for a,b,c,d it would call
// f(&a, &mut b), f(&b, &mut c), f(&c, &mut d)
fn pairs_mut<'a, Iter, T: 'a, Func>(xs: Iter, mut f: Func)
    where Iter: Iterator<Item = &'a mut T>,
          Func: FnMut(&T, &mut T)
{
    let mut prev = Option::<&T>::None;
    for x in xs {
        if let Some(prev) = prev {
            f(prev, x);
        }
        prev = Some(&*x);
    }
}

// Like pairs_mut(), but also folds an initial value through the pairs.
fn fold_pairs_mut<'a, Iter, T: 'a, R, Func>(init: R, xs: Iter, mut f: Func) -> R
where Iter: Iterator<Item = &'a mut T>,
    Func: FnMut(R, &T, &mut T) -> R
{
    let mut prev = Option::<&T>::None;
    let mut acc = init;
    for x in xs {
        if let Some(prev) = prev {
            acc = f(acc, prev, x);
        }
        prev = Some(&*x);
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pairs_mut() {
        let mut v = vec!(1, 2, 3, 4);
        pairs_mut(v.iter_mut(), |prev, this| *this += *prev);
        assert_eq!(v, vec!(1, 3, 6, 10));
    }

    #[test]
    fn test_fold_pairs_mut() {
        let mut v = vec!(1, 2, 3, 4);
        let count = fold_pairs_mut(0, v.iter_mut(), |acc, prev, this| {
            *this += *prev;
            acc + 1
        });
        assert_eq!(v, vec!(1, 3, 6, 10));
        assert_eq!(count, 3);
    }
}
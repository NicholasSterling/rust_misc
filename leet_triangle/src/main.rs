
// Calls the specified function with pair windows where the
// second is mutable, e.g. for a,b,c,d it would call
// f(&a, &mut b), f(&b, &mut c), f(&c, &mut d)

fn pairs_mut<'a, Iter, T: 'a, Func>(xs: Iter, f: Func)
    where Iter: IntoIterator<Item = &'a mut T>,
          Func: FnMut(&T, &mut T)
{
    fn inner<'a, Iter, T: 'a, Func>(mut xs: Iter, mut f: Func)
        where Iter: Iterator<Item = &'a mut T>,
            Func: FnMut(&T, &mut T)
    {
        if let Some(mut prev) = xs.next() {
            for x in xs {
                f(prev, x);
                prev = x;
            }
        }
    }
    inner(xs.into_iter(), f);
}

// Like pairs_mut(), but also folds an initial value through the pairs.
fn fold_pairs_mut<'a, Iter, T: 'a, R, Func>(init: R, xs: Iter, f: Func) -> R
where Iter: IntoIterator<Item = &'a mut T>,
    Func: FnMut(R, &T, &mut T) -> R
{
    fn inner<'a, Iter, T: 'a, R, Func>(init: R, mut xs: Iter, mut f: Func) -> R
    where Iter: Iterator<Item = &'a mut T>,
        Func: FnMut(R, &T, &mut T) -> R
    {
        let mut acc = init;
        if let Some(mut prev) = xs.next() {
            for x in xs {
                acc = f(acc, prev, x);
                prev = x;
            }
        }
        acc
    }
    inner(init, xs.into_iter(), f)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pairs_mut() {
        let mut v = vec!(1, 2, 3, 4);
        pairs_mut(&mut v, |prev, this| *this += *prev);
        assert_eq!(v, vec!(1, 3, 6, 10));
    }

    #[test]
    fn test_fold_pairs_mut() {
        let mut v = vec!(1, 2, 3, 4);
        let count = fold_pairs_mut(0, &mut v, |acc, prev, this| {
            *this += *prev;
            acc + 1
        });
        assert_eq!(v, vec!(1, 3, 6, 10));
        assert_eq!(count, 3);
    }
}
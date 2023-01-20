

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

fn main() {

    vec![vec![2],vec![3,4],vec![6,5,7],vec![4,1,8,3]]));

    let mut v = vec!(
        Foo { field: 0 }, Foo { field: 1 },
        Foo { field: 2 }, Foo { field: 3 },
    );
    println!("{:?}", &v);
    assign_fields(&mut v);
    println!("{:?}", &v);
    assign_fields2(v.iter_mut());
    println!("{:?}", &v);
    pairs_mut(v.iter_mut(), |prev, this| this.field += prev.field );
    println!("{:?}", &v);
    let count = fold_pairs_mut(0, v.iter_mut(), |acc, prev, this| {
        this.field += prev.field;
        acc + 1
    });
    println!("{:?}, {}", &v, count);
}

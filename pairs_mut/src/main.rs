// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=0dfd92b736e3faad4973ec255ca7c1a0

// The point here is to show how to modify elements of a vector (or Iterator)
// based on the previous element.

#[derive(Debug)]
struct Foo {
    field: i32,
}

fn assign_fields(vector: &mut Vec<Foo>) {
    let mut prev = Option::<&Foo>::None;
    for item in vector {
        if let Some(prev) = prev {
            item.field += prev.field;
        }
        prev = Some(&*item);
    }
}

fn assign_fields2<'a>(iter: impl Iterator<Item = &'a mut Foo>) {
    let mut prev = Option::<&Foo>::None;
    for item in iter {
        if let Some(prev) = prev {
            item.field += prev.field;
        }
        prev = Some(&*item);
    }
}

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

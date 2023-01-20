// Ah, I see what's going on.  I was thinking that if I had a &&mut Thing
// then I could deref it to a &mut Thing.  But obviously if that were possible
// then you could easily get around the exclusivity of mutable refs:
// the &&mut Thing could be shared and derefed by multiple parties,
// allowing them all to mutate.
// So that couldn't work.  Ultimately the problem is that min_by_key gives
// you a shared pointer; you could imagine having Iterator::min_by_key_mut,
// but it doesn't exist.  Oh, apparently it CAN'T exist:
// https://users.rust-lang.org/t/how-to-get-min-by-key-in-iter-mut/66916
// Looks like I could get around this using min_by rather than min_by_key.

//use std::cmp::Ordering;

#[derive(Debug)]
struct Thing(i32);

#[derive(Debug)]
struct Group ([Thing; 2]);

impl Group {
    fn min_by_key_mut_oops<T: Ord>(&mut self, f: impl Fn(&Thing) -> T) -> &mut Thing {
        self.0.iter_mut()
            .min_by_key( |t| f(t) )
            // .min_by_key( |t| f(*t) )  Does not work if f is FnMut!
            .unwrap() // things is never empty
    }
    // fn min_by_key_mut<T>(&mut self, f: impl Fn(&Thing) -> T) -> &mut Thing {
        // self.0.iter_mut()
            // .min_by( |a,b| f(a).cmp(f(b)) )
            // .min_by_key( |t| f(*t) )  Does not work if f is FnMut!
            // .unwrap() // things is never empty
    // }
    fn min_by_key_mut<F, T: Ord>(&mut self, f: F) -> (usize, &mut Thing)
    where F: FnMut(&mut Thing) -> T
    {
        self.0.iter_mut()
            .enumerate()
            .map( |(i, thing)| (i, thing, f(thing)) )
            .min_by_key( |(_, _, key)| key )
            .map( |(i, thing, key)| (i, thing) )
            .unwrap() // things is never empty
    }
    fn min_with_ix_by_key_mut<T: Ord>(&mut self, f: impl FnMut(&mut Thing) -> T) -> (usize, &mut Thing) {
        self.0.iter_mut()
            .enumerate()
            // .min_by_key( |t| f(t.1) )
            .min_by( |(_,a), (_,b)| (a.0).cmp(&b.0))
            .unwrap() // things is never empty
    }
}
fn main() {
    let mut group = Group([Thing(7), Thing(3)]);

    println!("group = {:?}", &group);
    let (i, min) = group.min_by_key_mut( |t| t.0 );
    println!("min = {}", min.0);
    min.0 = 4;  // min is a mutable ref
    println!("group = {:?}", &group);

    let (i, min) = group.min_with_ix_by_key_mut( |t| t.0 );
    println!("i = {}, min = {}", i, min.0);
    min.0 = 5;  // min is a mutable ref
    println!("group = {:?}", &group);
}

use itertools::Itertools;

fn main() {

    // Access arguments.  But really, use clap.
    for arg in ::std::env::args() {
        // fmt: https://fmt.dev/latest/syntax.html
        println!("{0:>30}", arg);  // first arg (0 optional here) right-justified in 30 columns
        // println!("{:<{}}", arg, 10);  // right-just. too, but specifying width (10) dynamically
        // Why doesn't that work?  A nightly thing?
    }
    dbg!(::std::env::args().collect::<Vec<_>>());

    // Initialize an array using a function.
    let squares: [i32; 20] = (1..20).map(|i| i*i).collect();
    dbg!(squares);

    // A bit of filtering.
    let v1 = vec![1,2,3];
    let v2 = v1.iter().any(|&x| x != 2);
    println!("v1 = {:?}, v2 = {:?}", v1, v2);
    dbg!(v1, v2);

    // Ranges, and filtering with/out references.
    // Note that iterating over a Range consumes (moves) it.
    // fn r() -> Range<u32> { 0..15 }
    let r = { || 0..15 };
    dbg!(r());
    fn is_even1(x:  u32) -> bool {  x % 2 == 0 }
    fn is_even2(x: &u32) -> bool { *x % 2 == 0 }
    let is_even3 = | x: &u32| *x % 2 == 0;
    let is_even4 = |&x: &u32|  x % 2 == 0;
    // println!("{:?}", r().filter(is_even1 ).collect::<Vec<u32>>());
            // expected: for<'r> fn(&'r {integer}) -> _
            // found:            fn(u32) -> _
    println!("{:?}", r().filter(|&x| is_even1(x)).collect::<Vec<u32>>());
    println!("{:?}", r().filter(is_even2).collect::<Vec<u32>>());
    println!("{:?}", r().filter(is_even3).collect::<Vec<u32>>());
    println!("{:?}", r().filter(is_even4).collect::<Vec<u32>>());

    // Dynamic polymorphism.
    trait Canine         { fn id(&self) -> u32;      }
    impl  Canine for Dog { fn id(&self) -> u32 { 1 } }
    impl  Canine for Fox { fn id(&self) -> u32 { 2 } }
    fn generic_fn<T: Canine>(it: &T         ) -> u32 { it.id() }  // gets monomorphized
    fn dynamic_fn           (it: &dyn Canine) -> u32 { it.id() }  // uses a vtable
    // ... without Box
    struct Dog; let dog = Dog;
    struct Fox; let fox = Fox;
    let it: &dyn Canine = &dog;
    dbg!(it.id());
    dbg!(generic_fn(&fox));
    dbg!(dynamic_fn(&dog));
    dbg!(dynamic_fn(it));
    // ... with Box
    let boxed_dog = Box::new(dog);
    dbg!(boxed_dog.id());       // Note that we treat the Box<Dog> like a Dog.
    let boxed_canine: Box<dyn Canine> = Box::new(fox);
    dbg!(boxed_canine.id());    // This uses a vtable.

    // Recursion
    fn sum1(n: i64) -> i64 {
        if n > 0 { n + sum1(n-1) } else { 0 }
    }
    dbg!(sum1(5));
    fn _sum2(n: i64) -> i64 {
        fn recur(n: i64, acc: i64) -> i64 {
            if n > 0 { recur(n-1, n+acc) } else { acc }
        }
        recur(n, 0)
    }
    // dbg!(sum2(5));

    // dbg!(sum1( 100_000));  // 5000050000
    // dbg!(sum1( 1000_000_000));  // stack overflow running it here

    // dbg!(sum2( 100_000));  // 5000050000
    // dbg!(sum2( 1_000_000_000_000_000));  // stack overflow running it here

    // But compile with rustc -C opt-level=2 src/main.rs and it works.
    // https://stackoverflow.com/questions/59257543/when-is-tail-recursion-guaranteed-in-rust
    // BOTH sum1 and sum2 get optimized!
    // Oh, but wait -- the optimizer is not doing TCO, it's just optimizing
    // the whole thing to a constant, I think.


    // Returns an iterator that generates the Fibonacci sequence.
    fn fib() -> impl Iterator<Item = u32> {
        let mut nums = (0, 1);
        std::iter::repeat_with(
            move || {
                nums = (nums.1, nums.0 + nums.1);
                nums.0
            }
        )
    }
    dbg!(fib().take(6).collect::<Vec<u32>>());
    dbg!(fib()
        .take_while(|&x| x < 100)
        .filter(|x| x % 2 == 0)
        .sum::<u32>()
    );

/////////////////////////////////////////////

    #[derive(Debug)]
    struct Foo {
        id: u32,
        name: String,
    }
    let foos = vec![
        Foo { id: 1, name: "Bill".to_string() },
        Foo { id: 2, name: "Mary".to_string() },
    ];
    println!("{:?}", foos);
    println!("{:?}", map_by(foos.iter(), |f|  f.id));     // borrow  the Foos
    println!("{:?}", map_by(foos.iter(), |f| &f.name));   // borrow  the Foos
    println!("{:?}", map_by(foos.iter(), |f|  f.name.as_str()));   // borrow  the Foos
    println!("{:?}", map_by(foos,        |f|  f.id));  // consume the Foos
    // println!("{:?}", map_by(foos,        |f| &f.name));   // ERROR: cannot infer lifetime
    // println!("{:?}", map_by(foos,        |f| f.name.as_str()));   // ERROR: cannot infer lifetime


    // Creating a HashMap from KV tuples.
    let tuples = vec![("one", 1), ("two", 2), ("three", 3)];
    let m: HashMap<_, _> = tuples.into_iter().collect();
    println!("{:?}", m);
    let tuples = vec![(1, "one"), (2, "two"), (3, "three")];
    let m: HashMap<_, _> = tuples.into_iter().collect();
    println!("{:?}", m);
    // Now with an array.
    let tuples = [("one", 1), ("two", 2), ("three", 3)];
    let m: HashMap<_, _> = tuples.into_iter().collect();
    println!("{:?}", m);
    let tuples = [(1, "one"), (2, "two"), (3, "three")];
    let m: HashMap<_, _> = tuples.iter().map(|(k,v)| (k, v.to_string())).collect();
    println!("{:?}", m);

    make_people();

/////////////////////////////////////////////

    // Removing duplicates in unordered data.
    fn dedup(v: &mut Vec<i32>) {
        let mut set = HashSet::new();
        v.retain(|&x| set.insert(x));
    }
    let mut nums = vec![3,7,1,3,2,1,5];
    dedup(&mut nums);
    dbg!(nums);

/////////////////////////////////////////////

    // Transmuting data.
    let array = [10,11,12,13];
    let _slice = &array[..];
    let addr = &array[0];
    dbg!(addr);
    let ptr: &[[i32; 2]; 2]  = unsafe { std::mem::transmute(addr) };
    dbg!(*ptr);
    dbg!(ptr[1][0]);
    dbg!(ptr);

/////////////////////////////////////////////

    // Generic arithmetic.
    use std::ops::Div;
    use num::Integer;
    fn div_rounded<T: Integer + Div + From<usize> + Copy>(numer: T, denom: T) -> T {
        (numer + denom/2.into()) / denom
    }
    dbg!(div_rounded(7,3));  // 2
    dbg!(div_rounded(8,3));  // 3

/////////////////////////////////////////////

    // Balanced pairs.
    // Here's a clever solution: https://exercism.org/tracks/rust/exercises/matching-brackets/solutions/michaelmez39
    // My solution: https://exercism.org/tracks/rust/exercises/matching-brackets/solutions/NicholasSterling
    fn is_balanced(string: &str) -> bool {
        let mut need = Vec::<char>::new();
        for c in string.chars() {
            match c {

                // If it's an opener, push the corresponding closer.
                '{' => need.push('}'),
                '[' => need.push(']'),
                '(' => need.push(')'),

                // If it's a closer, it should be what we are expecting.
                c @ ( '}' | ']' | ')' ) =>
                    if need.pop() != Some(c) {
                        return false;
                    },

                _ => ()
            }
        }
        need.is_empty()
    }
    dbg!(is_balanced(""));
    dbg!(is_balanced("{}"));
    dbg!(is_balanced("{}{}"));
    dbg!(is_balanced("{[]}"));
    dbg!(is_balanced("{[]{}}<>"));
    dbg!(is_balanced("}{}"));
    dbg!(is_balanced("{}}{}"));

    fn is_balanced(string: &str, pairs: &[(char, char)]) -> bool {
        let mut need = Vec::new();
        for c in string.chars() {
            if let Some((_, close)) = pairs.iter().find( |(open, _)| c == *open ) {
                need.push(*close);
            } else if PAIRS.iter().any( |(_, close)| c == *close ) && need.pop() != Some(c) {
                return false;
            }
        }
        need.is_empty()
    }
    const PAIRS: [(char,char); 3] = [ ('(',')'), ('[',']'), ('{','}') ];

}

/////////////////////////////////////////////

// https://play.rust-lang.org/?version=stable&mode=debug&edition=2018&gist=700f859b212e3fb5d3a9b3c1f7614abc
// https://play.rust-lang.org/?version=nightly&mode=debug&edition=2018&gist=6dbf3004a59886a893f1a8347c0c0d1f
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

fn map_by<I,K,V>(iterable: I, f: impl Fn(&V) -> K) -> HashMap<K,V>
where I: IntoIterator<Item = V>,
      K: Eq + Hash
{
    iterable.into_iter().map(|v| (f(&v), v)).collect()
}

/////////////////////////////////////////////

// collect_tuple

#[derive(Debug)]
struct Person<'a> {
    first: &'a str,
    last:  &'a str,
}

impl<'a> Person<'a> {
    // Create a Person from a str of form "last,first".
    fn from_csv(s: &'a str) -> Option<Self> {
        s.split(',').collect_tuple().map(
            |(last, first)| Person { first, last }
        )
    }
}

fn make_people() {
    dbg!(Person::from_csv("Doe"));          // None
    dbg!(Person::from_csv("Doe,John"));     // Some(...)
    dbg!(Person::from_csv("Doe,John,foo")); // None
    let p1 = Person::from_csv("Doe,Jane").unwrap();
    dbg!(p1.first);
    dbg!(p1.last);
}

/////////////////////////////////////////////

// Make array on the heap without first making it on the stack.
// https://users.rust-lang.org/t/u8-1024-1024-or-a-vec-u8/106542/16
pub fn make_boxed_array<T, const N: usize, F: Fn(usize) -> T>(f: F) -> Box<[T; N]> {
    (0..N).map(f).collect::<Box<[_]>>().try_into().ok().unwrap()
}



trait Locatable {
    type T;
    fn locate<L: Locator<T = Self::T>>(&self, c: char) -> L;
}

impl Locatable for &str {
    type T = usize;
    fn locate<L: Locator<T = Self::T>>(&self, c: char) -> L {
        StrLocator { c, iter: self.chars().enumerate() }
    }
}

trait Locator {
    type T;
}

struct StrLocator<Iter>
    where Iter: Iterator<Item = (usize, char)>
{
    c: char,
    iter: Iter
}

impl<Iter> Locator for StrLocator<Iter>
    where Iter: Iterator<Item = (usize, char)>
{
    type T = usize;

}

impl<Iter> Iterator for StrLocator<Iter>
    where Iter: Iterator<Item = (usize, char)>
{
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item> {
        self.find( |c| c == self.c )
    }
}

fn main() {
    let foo = "hello".chars();
}

/*
impl<'a, Iter> Locatable for &Iter
where Iter: Iterator<Item = &'a str> + Clone
{
    type T = (usize, usize);
    fn locate(&self, c: char) -> Option<Self::T> { Locate { iter: &mut self } }
        self.enumerate().find_map( |(row, s)| s.find('a').map( |col| (row, col) ) )
let clone = (*self).clone();
clone.enumerate().find_map( |(row, s)| s.locate(c).map( |col| (row, col) ) )
}
}

struct StrIterLocator<'a, Iter>
where Iter: Iterator<Item = &'a str>
{
    iter: &'a mut Iter
}

impl<'a, Iter> Locatable for Locate<'a, Iter>
where Iter: Iterator<Item = &'a str>
{
    type T = (usize, usize);
    fn locate(&self, c: char) -> Option<Self::T> {
        self.iter.enumerate().find_map( |(row, s)| s.locate(c).map( |col| (row, col) ) )
    }
}

// impl<'a, Iter> Locate for &Iter
// where Iter: Iterator<Item = &'a str> + Clone
//                               { type T = (usize, usize); fn locate(&self, c: char) -> Option<Self::T>
//     {
//         self.enumerate().find_map( |(row, s)| s.find('a').map( |col| (row, col) ) )
        // let clone = (*self).clone();
        // clone.enumerate().find_map( |(row, s)| s.locate(c).map( |col| (row, col) ) )
    // }
// }

fn main() {
    dbg!("Now is the time".locate('w'));
    dbg!(&mut Text { iter: "Now is the time\nfor all good men".lines() }.locate('w'));
}

*/

/*

impl<'a, II: IntoIterator<Item = &'a S1>> Foo for II {
    type T = u32;
    fn foo(&mut self) -> (Self::T, i32) {
        //(3, self.fold(0, |acc, s| s.n as i32 + acc))
        (3, (&mut self).into_iter().map( |s1| s1.n ).sum())
        //(3, <&Iter as Into<T>>::into(self).map( |s1| s1.n ).sum())
        //(3, <&Iter as Into<Item = &'a S1, IntoIter = Iter>>::into(self).map( |s1| s1.n ).sum())
    }
}

fn main() {
    let s1 = S1 { n: 1 }; dbg!(s1.foo());
    let s2 = S2 { n: 2 }; dbg!(s2.foo());
    let _ = vec![s1];
    // dbg!(a.foo());
}

 */

// Simple Vec<f64>

fn _dot_product1(xs: &Vec<f64>, ys: &Vec<f64>) -> f64
{
    xs.into_iter().zip(ys).map( |(x,y)| x*y ).sum()
}

// IntoIterator

fn _dot_product2<'a, XS, YS>(xs: XS, ys: YS) -> f64
    where XS: IntoIterator<Item = &'a f64>,
          YS: IntoIterator<Item = &'a f64>
{
    xs.into_iter().zip(ys).map( |(x,y)| x*y ).sum()
}

/*  No, not working.
// Into<Iterator>

fn dot_product2a<'a, XS, YS, XI, YI>(xs: XS, ys: YS) -> f64
    where XS: Into<XI>,
          YS: Into<YI>,
          XI: Iterator<Item = f64>,
          YI: Iterator<Item = f64>
{
    xs.into().zip(ys.into()).map( |(x,y)| x*y ).sum()
}
 */

// Generic
// Why does this need to be Copy?

use std::ops::Mul;
use std::iter::Sum;

fn dot_product<'a, XS, YS, T: 'a>(xs: XS, ys: YS) -> T
    where XS: IntoIterator<Item = &'a T>,
          YS: IntoIterator<Item = &'a T>,
          T: Mul<Output = T> + Sum<T> + Copy
{
    xs.into_iter().zip(ys).map( |(&x, &y)| x*y ).sum()
}

///////////////////////

fn main() {
    let foo = vec![7f64, 3.0, 4.0, 1.0];
    let bar = vec![7f64, 3.0, 4.0, 1.0];
    dbg!(dot_product(&foo, &bar));
    // dbg!(dot_product2a(&foo, &bar));
}

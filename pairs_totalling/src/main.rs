// You could also take a cloneable iterator.

pub fn pairs_totalling<'a>(xs: &'a [i32], target: i32) -> impl Iterator<Item = (usize, usize)> + 'a {
    xs.iter().enumerate()
    .flat_map(move |(i, &x)|
        xs.iter().enumerate()
            .skip(i+1)
            .filter_map(move |(j, &y)| (x+y == target).then(|| (i,j)))
    )
}

/* This version returns the numbers themselves, not their indices.
pub fn pairs_totalling<'a>(xs: &'a [i32], target: i32) -> impl Iterator<Item = (i32,i32)> + 'a {
    xs.iter().enumerate()
    .flat_map(move |(i,&x)|
        xs[i+1..].iter()
            .filter_map(move |&y| (x+y == target).then(|| (x,y)))
    )
}
*/

pub fn main() {

    // Use it with a Vec.
    let xs = vec![1,7,2,3,0,4,2,9,6,5,3,8];
    let pairs: Vec<_> = pairs_totalling(&xs, 9).collect();
    dbg!(pairs);
    
    // Use it with an array.  And we only want the first
    // two pairs, so don't waste time computing the rest.
    let xs: [i32; 12] = [1,7,2,3,0,4,2,9,6,5,3,8];
    let pairs: Vec<_> = pairs_totalling(&xs, 9).take(2).collect();
    dbg!(pairs);
}
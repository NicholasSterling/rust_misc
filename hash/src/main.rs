use std::{fs::File, io::BufReader, io::Error};
use std::io::{BufRead, Lines};

fn main() -> Result<(), Error> {
    let file = File::open("it")?;
    let br: BufReader<File> = BufReader::new(file);
    let lines: Lines<BufReader<File>> = br.lines();
    let mut hashes: Vec<usize> = lines
        .map(|line| line.expect("IO error"))
        .map(|word| { dbg!(&word); word } )
        .map(|word| {
            let first  = word.chars().nth(0).unwrap() as usize;
            let last   = word.chars().last().unwrap() as usize;
            let middle = word.chars().nth(word.len() / 2).unwrap() as usize;
            let len = word.len();
            //(first << 14) + (last << 9) + (middle << 4) + len
            //(first << 6) ^ (last << 4) ^ (middle << 2) ^ len
            //((first << 0) + (last << 5) + (middle << 3) + (len << 5))
            //let hash = ((first << 6) ^ (last << 4) ^ (middle << 2) ^ (len << 0));  // works
            let hash = ((first << 0) ^ (last << 4) ^ (middle << 2) ^ (len << 7)) - 399;  // Winner 0..1565
            //let hash = ((first << 3) ^ (last << 0) ^ (middle * 5) ^ (len << 5));  // fail
            //let hash = ((first << 3) ^ (last << 0) ^ (middle * 9) ^ (len << 4));  // fail
            let hash = ((hash >> 4) ^ hash) & 511;
            hash
        })
        .map(|hash| { dbg!(hash); hash } )
        .collect();
    dbg!(hashes.len());
    hashes.sort();
    //dbg!(&hashes);
    hashes.dedup();
    dbg!(hashes.len());
    dbg!(hashes.iter().min());
    dbg!(hashes.iter().max());
    Ok(())
}

use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("input.txt")?;
    let lines = BufReader::new(f).lines();

    let mut l1;
    let mut l2;
    let mut s: u32 = 0;

    for l in lines {
        let l = l.as_ref().unwrap();
        l1 = &l[0..l.len() / 2];
        l2 = &l[l.len() / 2..l.len()];
        'outer: for c1 in l1.chars() {
            for c2 in l2.chars() {
                if c1 == c2 {
                    if (c1 as u32) < 96 {
                        s += c1 as u32 - 38;
                    } else {
                        s += c1 as u32 - 96;
                    }
                    break 'outer;
                }
            }
        }
    }
    println!("Score: {}", s);

    Ok(())
}

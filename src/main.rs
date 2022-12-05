use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("input.txt")?;
    let lines = BufReader::new(f).lines();

    let mut comp1;
    let mut comp2;
    let mut score: u32 = 0;

    for line in lines {
        let line = line.as_ref().unwrap();
        comp1 = &line[0..line.len() / 2];
        comp2 = &line[line.len() / 2..line.len()];
        'outer: for c1 in comp1.chars() {
            for c2 in comp2.chars() {
                if c1 == c2 {
                    if (c1 as u32) < 96 {
                        score += c1 as u32 - 38;
                        println!("{} {}", comp1, comp2);
                        println!("{} {} {}", c1, c1 as u32 - 38, score);
                        break 'outer;
                    } else {
                        score += c1 as u32 - 96;
                        println!("{} {}", comp1, comp2);
                        println!("{} {} {}", c1, c1 as u32 - 96, score);
                        break 'outer;
                    }
                }
            }
        }
    }
    println!("Score: {}", score);

    Ok(())
}

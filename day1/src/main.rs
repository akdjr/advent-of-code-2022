use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./input.txt")?;
    let lines = io::BufReader::new(file).lines();

    let mut elves: Vec<u32> = vec![0];
    let mut current_elf: usize = 0;

    for line in lines {
        if let Ok(lp) = line {
            if lp != "" {
                let calories: u32 = lp.trim().parse().unwrap();
                elves[current_elf] += calories;
            }
            

            if lp == "" {
                elves.push(0);
                current_elf += 1;
            }
        }
    }

    elves.sort();
    elves.reverse();

    let top3_total = elves[0] + elves[1] + elves[2];

    println!("top 3 elves: {}, {}, {} for a total of {}", elves[0], elves[1], elves[2], top3_total);

    Ok(())
}

use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::thread::current;

pub struct Elf {
    food: Vec<u32>
}

impl Elf {
    pub fn new() -> Elf {
        Elf {
            food: vec![]
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("./input.txt")?;
    let lines = io::BufReader::new(file).lines();

    let mut elves: Vec<Elf> = vec![Elf::new()];
    let mut current_elf: usize = 0;

    let mut elf_with_most_calories: usize = 0;
    let mut sum: u32 = 0;
    let mut largest_sum: u32 = 0;

    for line in lines {
        if let Ok(lp) = line {
            if lp != "" {
                let calories: u32 = lp.trim().parse().unwrap();

                sum += calories;
                elves[current_elf].food.push(calories)
            }
            

            if lp == "" {
                println!("elf {} is carrying {} calories.", current_elf, sum);

                if sum > largest_sum {
                    largest_sum = sum;
                    elf_with_most_calories = current_elf;
                }

                sum = 0;

                elves.push(Elf::new());

                current_elf += 1;
            }
        }
    }

    println!("elf {} has the most calories ({})", elf_with_most_calories, largest_sum);

    Ok(())
}

use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Copy, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissors
}

#[derive(Copy, Clone)]
enum MatchResult {
    Win,
    Loss,
    Draw
}

pub struct Match {
    your_shape: Shape,
    opponent_shape: Shape
}

impl Match {
    fn shape_from_char(shape: char) -> Shape {
        match shape {
            'A' => Some(Shape::Rock),
            'B' => Some(Shape::Paper),
            'C' => Some(Shape::Scissors),
            _ => None
        }.unwrap()
    }

    fn result_from_char(result: char) -> MatchResult {
        match result {
            'X' => Some(MatchResult::Loss),
            'Y' => Some(MatchResult::Draw),
            'Z' => Some(MatchResult::Win),
            _ => None
        }.unwrap()
    }

    fn your_shape_for_result(&mut self, result: MatchResult) {
        self.your_shape = match (self.opponent_shape, result) {
            (Shape::Rock, MatchResult::Win) => Shape::Paper,
            (Shape::Rock, MatchResult::Loss) => Shape::Scissors,
            (Shape::Rock, MatchResult::Draw) => Shape::Rock,

            (Shape::Paper, MatchResult::Win) => Shape::Scissors,
            (Shape::Paper, MatchResult::Loss) => Shape::Rock,
            (Shape::Paper, MatchResult::Draw) => Shape::Paper,

            (Shape::Scissors, MatchResult::Win) => Shape::Rock,
            (Shape::Scissors, MatchResult::Loss) => Shape::Paper,
            (Shape::Scissors, MatchResult::Draw) => Shape::Scissors,
        }
    }

    fn evaluate(&self) -> u32 {
        let shape_points = match self.your_shape {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3
        };

        let match_points = match (self.your_shape, self.opponent_shape) {
            (Shape::Rock, Shape::Paper) => 0,
            (Shape::Rock, Shape::Rock) => 3,
            (Shape::Rock, Shape::Scissors) => 6,
            (Shape::Paper, Shape::Scissors) => 0,
            (Shape::Paper, Shape::Paper) => 3,
            (Shape::Paper, Shape::Rock) => 6,
            (Shape::Scissors, Shape::Rock) => 0,
            (Shape::Scissors, Shape::Scissors) => 3,
            (Shape::Scissors, Shape::Paper) => 6,
        };

        return shape_points + match_points;
    }
}

fn create_part1_match(your_shape: char, opponent_shape: char) -> Match {
    Match {
        your_shape: match your_shape {
            'X' => Some(Shape::Rock),
            'Y' => Some(Shape::Paper),
            'Z' => Some(Shape::Scissors),
            _ => None
        }.unwrap(),

        opponent_shape: match opponent_shape {
            'A' => Some(Shape::Rock),
            'B' => Some(Shape::Paper),
            'C' => Some(Shape::Scissors),
            _ => None
        }.unwrap()
    }
}

fn create_part2_match(opponent_shape: char, result: char) -> Match {
    let mut new_match = Match {
        opponent_shape: Match::shape_from_char(opponent_shape),
        your_shape: Shape::Rock
    };

    new_match.your_shape_for_result(Match::result_from_char(result));

    new_match
}

fn part1() -> Result<(), Box<dyn Error>>{
    let file = File::open("./input.txt")?;
    let lines = io::BufReader::new(file).lines();

    let mut total_score: u32 = 0;

    let mut current_match: Match;

    for line in lines {
        if let Ok(lp) = line {
            if lp == "" {
                continue;
            }

            let mut iter = lp.chars();
            let opponent = iter.next().unwrap();
            iter.next().unwrap();
            let you = iter.next().unwrap();

            current_match = create_part1_match(you, opponent);           

            total_score += current_match.evaluate();
        }
    }

    println!("Part 1: your total score: {}", total_score);

    Ok(())
}

fn part2() -> Result<(), Box<dyn Error>> {
    let file = File::open("./input.txt")?;
    let lines = io::BufReader::new(file).lines();

    let mut total_score: u32 = 0;

    let mut current_match: Match;

    for line in lines {
        if let Ok(lp) = line {
            if lp == "" {
                continue;
            }

            let mut iter = lp.chars();
            let opponent = iter.next().unwrap();
            iter.next().unwrap();
            let result = iter.next().unwrap();

            current_match = create_part2_match(opponent, result);     

            total_score += current_match.evaluate();
        }
    }

    println!("Part 2: your total score: {}", total_score);

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    part1()?;
    part2()?;

    Ok(())
}

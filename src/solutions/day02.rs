// https://adventofcode.com/2022/day/2

use std::fs::read_to_string;

#[derive(PartialEq, Clone, Copy)]
enum Shape {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl Shape {
    fn new(value: char) -> Self {
        match value {
            'A' | 'X' => Shape::Rock,
            'B' | 'Y' => Shape::Paper,
            'C' | 'Z' => Shape::Scissors,
            _ => panic!("Invalid value for Shape initialization!"),
        }
    }

    fn winning(&opp: &Shape) -> Self {
        match opp {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }

    fn losing(&opp: &Shape) -> Self {
        match opp {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }

    fn fight(self, &opp: &Shape) -> u32 {
        if self == opp {
            self as u32 + 3
        } else {
            match (self, opp) {
                (Shape::Rock, Shape::Scissors) => self as u32 + 6,
                (Shape::Paper, Shape::Rock) => self as u32 + 6,
                (Shape::Scissors, Shape::Paper) => self as u32 + 6,
                _ => self as u32
            }
        }
    }
}

pub fn part1() -> u32 {
    let content = read_to_string("data/day02.txt").unwrap();

    content.lines().fold(0, |acc, line| {
        let mut lc = line.chars();
        let opp = Shape::new(lc.next().unwrap());
        lc.next();
        let me = Shape::new(lc.next().unwrap());

        acc + me.fight(&opp)
    })
}

pub fn part2() -> u32 {
    let content = read_to_string("data/day02.txt").unwrap();

    content.lines().fold(0, |acc: u32, line| {
        let mut lc = line.chars();
        let opp = Shape::new(lc.next().unwrap());
        lc.next();

        let result = match lc.next() {
            Some('X') => Shape::losing(&opp).fight(&opp),
            Some('Y') => opp.fight(&opp),
            Some('Z') => Shape::winning(&opp).fight(&opp),
            _ => panic!("Invalid value for match result!"),
        };

        acc + result
    })
}
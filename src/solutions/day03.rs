// https://adventofcode.com/2022/day/3

use std::collections::HashMap;
use std::fs::read_to_string;

fn priority(c: char) -> u32 {
    let casu = c as u32;
    match casu {
        97..=122 => casu - 96,
        65..=90 => casu - 38,
        _ => panic!("Cannot assign priority to this char: {}", c),
    }
}

pub fn part1() -> u32 {
    let content = read_to_string("data/day03.txt").unwrap();

    content.lines().fold(0u32, |acc, line| {
        let n = line.len() / 2;
        let mut types_in_first: HashMap<char, u32> = HashMap::new();

        for (i, c) in line.chars().enumerate() {
            if i < n {
                types_in_first.entry(c).or_insert(priority(c));
            } else if let Some(p) = types_in_first.get(&c) {
                return *p + acc;
            }
        }
        acc
    })
}

pub fn part2() -> u32 {
    let lines = read_to_string("data/day03.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect::<Vec<_>>();
    let chunks = lines.chunks(3);
    let mut acc: u32 = 0;

    for chunk in chunks {
        let badge = chunk[0]
            .chars()
            .find(|c| chunk[1].contains(*c) && chunk[2].contains(*c)
            );
        if let Some(c) = badge {
            acc += priority(c);
        }
    }

    acc
}


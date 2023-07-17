// https://adventofcode.com/2022/day/1

use std::cmp::max;
use std::fs::read_to_string;

pub fn part1() -> i32 {
    let content = read_to_string("data/day1-1.txt").unwrap();

    let mut running_sum = 0;
    let mut max_sum = 0;

    for line in content.lines() {
        match line.parse::<i32>() {
            Ok(kcal) => running_sum += kcal,
            Err(_) => {
                max_sum = max(max_sum, running_sum);
                running_sum = 0;
            }
        }
    }

    max_sum
}
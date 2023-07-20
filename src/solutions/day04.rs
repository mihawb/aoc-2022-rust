// https://adventofcode.com/2022/day/4

use std::fs::read_to_string;

fn total_overlap(t: (u8, u8, u8, u8)) -> bool {
    (t.0 <= t.2 && t.1 >= t.3) || (t.0 >= t.2 && t.1 <= t.3)
}

fn some_overlap(t: (u8, u8, u8, u8)) -> bool {
    t.0 <= t.3 && t.1 >= t.2
}

enum Overlap {
    Some,
    Total,
}

fn the_thing(ovr: Overlap) -> u32 {
    let content = read_to_string("data/day04.txt").unwrap();

    content
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            let (a, b) = x.split_once('-').unwrap();
            let (c, d) = y.split_once('-').unwrap();
            (
                a.parse::<u8>().unwrap(),
                b.parse::<u8>().unwrap(),
                c.parse::<u8>().unwrap(),
                d.parse::<u8>().unwrap(),
            )
        })
        .filter(|t| match ovr {
            Overlap::Some => some_overlap(*t),
            Overlap::Total => total_overlap(*t),
        })
        .count() as u32
}

pub fn part1() -> u32 {
    the_thing(Overlap::Total)
}

pub fn part2() -> u32 {
    the_thing(Overlap::Some)
}
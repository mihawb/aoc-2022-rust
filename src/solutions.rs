// use std::fs::{File, read_to_string};
// use std::io::{self, BufRead};
// use std::path::Path;
//
// fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
//     where P: AsRef<Path>, {
//     let file = File::open(filename)?;
//     Ok(io::BufReader::new(file).lines())
// }
//
// fn file_as_vec(filename: &str) -> Vec<String> {
//     read_to_string(filename)
//         .unwrap()
//         .lines()
//         .map(String::from)
//         .collect()
// }

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
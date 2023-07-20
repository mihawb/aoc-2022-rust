mod solutions;

fn main() {
    println!("Day 1: Calorie Counting, part 1 -> {}", solutions::day01::part1());
    println!("Day 1: Calorie Counting, part 2 -> {}", solutions::day01::part2());

    println!();

    println!("Day 2: Rock Paper Scissors, part 1 -> {}", solutions::day02::part1());
    println!("Day 2: Rock Paper Scissors, part 2 -> {}", solutions::day02::part2());

    println!();

    println!("Day 3: Rucksack Reorganization, part 1 -> {}", solutions::day03::part1());
    println!("Day 3: Rucksack Reorganization, part 2 -> {}", solutions::day03::part2());

    println!();

    println!("Day 4: Camp Cleanup, part 1 -> {}", solutions::day04::part1());
    println!("Day 4: Camp Cleanup, part 2 -> {}", solutions::day04::part2());
}

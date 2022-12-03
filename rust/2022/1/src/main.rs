mod part1;
mod part2;
use aoc_utils::input::input_as_string;

fn main() {
    let input: String = input_as_string("2022/1/res/input.txt", true);

    let elves: Vec<Vec<i32>> = input.split("\n\n")
        .into_iter()
        .map(|elf| elf.split('\n')
            .map(|number| number.parse::<i32>().unwrap())
            .collect()
        )
        .collect();

    println!("Part 1 answer: {}", part1::solve(&elves));
    println!("Part 2 answer: {}", part2::solve(&elves));
}
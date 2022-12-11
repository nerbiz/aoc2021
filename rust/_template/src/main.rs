mod part1;
use aoc_utils::input::input_as_lines;

fn main() {
    let input: Vec<String> = input_as_lines("20xx/x/res/input-test.txt", true);

    println!("Part 1 answer: {}", part1::solve(&input));
}

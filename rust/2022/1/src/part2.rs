pub fn solve(elves: &Vec<Vec<i32>>) -> i32 {
    let mut calories: Vec<i32> = elves.iter()
        .map(|elf| elf.iter().sum::<i32>())
        .collect();

    calories.sort_by(|a, b| b.cmp(a));
    calories[0..3].iter().sum::<i32>()
}

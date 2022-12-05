static INPUT: &'static str = include_str!("input.txt");

pub fn part_one() -> u32 {
    return INPUT
        .split("\n\n")
        .map(|block| {
            block
                .split('\n')
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap();
}

pub fn part_two() -> u32 {
    let mut block_values: Vec<u32> = INPUT
        .split("\n\n")
        .map(|block| {
            block
                .split('\n')
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect();

    block_values.sort_by(|a, b| b.cmp(a));

    return block_values[0..3].into_iter().sum();
}

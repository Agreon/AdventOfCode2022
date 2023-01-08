static INPUT: &str = include_str!("input.txt");

pub fn part_one() -> u32 {
    INPUT
        .split("\n\n")
        .map(|block| {
            block
                .split('\n')
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap()
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

    block_values[0..3].iter().sum()
}

#[cfg(test)]
mod tests {
    use super::part_one;
    use super::part_two;

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(), 71934)
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(), 211447)
    }
}

use std::ops::RangeInclusive;

advent_of_code::solution!(2);

fn parse_input(input: &str) -> impl Iterator<Item = RangeInclusive<u64>> {
    input
        .split(',')
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let mut items = line.split('-').filter_map(|item| item.parse().ok());
            items.next().zip(items.next()).map(|(a, b)| a..=b)
        })
}

pub fn part_one(input: &str) -> Option<u64> {
    let ret = parse_input(input)
        .flatten()
        .filter(|num| {
            let num = num.to_string();
            let len = num.len();
            len.is_multiple_of(2) && num[..len / 2] == num[len / 2..]
        })
        .sum();

    Some(ret)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ret = parse_input(input)
        .flatten()
        .filter(|num| {
            let num = num.to_string().chars().collect::<Vec<_>>();
            let len = num.len();

            (1..=len / 2)
                .filter(|&k| len.is_multiple_of(k))
                .filter_map(|k| {
                    let mut chunks = num.chunks(k);
                    chunks
                        .next()
                        .map(|first| chunks.all(|chunk| first == chunk))
                })
                .any(|current| current)
        })
        .sum();

    Some(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}

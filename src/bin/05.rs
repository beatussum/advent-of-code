use itertools::Itertools;
use std::{collections::HashSet, hash::RandomState, ops::RangeInclusive};

advent_of_code::solution!(5);

fn parse_input(input: &str) -> (Vec<RangeInclusive<u64>>, impl Iterator<Item = u64>) {
    let ranges = input
        .split('\n')
        .take_while(|line| !line.is_empty())
        .filter_map(|line| {
            let mut line = line.split('-').filter_map(|item| item.parse().ok());
            line.next().zip(line.next()).map(|(a, b)| a..=b)
        })
        .collect();

    let numbers = input
        .split('\n')
        .skip_while(|line| !line.is_empty())
        .filter(|line| !line.is_empty())
        .filter_map(|line| line.parse().ok());

    (ranges, numbers)
}

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges, numbers) = parse_input(input);

    let ret = numbers
        .filter(|number| ranges.iter().any(|range| range.contains(number)))
        .count();

    Some(ret as _)
}

fn merge_range(lhs: RangeInclusive<u64>, rhs: RangeInclusive<u64>) -> Option<RangeInclusive<u64>> {
    if lhs == rhs {
        None
    } else if rhs.contains(lhs.start()) || rhs.contains(lhs.end()) {
        let ret = [rhs.start(), rhs.end(), lhs.start(), lhs.end()]
            .into_iter()
            .copied()
            .minmax()
            .into_option()
            .map(|(a, b)| a..=b)
            .unwrap();

        Some(ret)
    } else {
        None
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let (ranges, _) = parse_input(input);
    let mut ranges = HashSet::<_, RandomState>::from_iter(ranges);

    let mut previous_len = 0;
    let mut current_len = ranges.len();

    while previous_len != current_len {
        let to_update = ranges
            .iter()
            .cloned()
            .flat_map(|lhs| ranges.iter().map(move |rhs| (lhs.clone(), rhs.clone())))
            .map(|(lhs, rhs)| merge_range(lhs.clone(), rhs.clone()).map(|new| (lhs, rhs, new)))
            .find(|option| option.is_some())
            .flatten();

        if let Some((lhs, rhs, new)) = to_update {
            ranges.remove(&lhs);
            ranges.remove(&rhs);
            ranges.insert(new);
        }

        previous_len = current_len;
        current_len = ranges.len();
    }

    let ret = ranges.into_iter().map(RangeInclusive::count).sum::<usize>();
    Some(ret as _)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}

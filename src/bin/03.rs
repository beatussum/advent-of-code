use cached::{Cached, UnboundCache, proc_macro::cached};
use std::sync::LockResult;

advent_of_code::solution!(3);

fn parse_input(input: &str) -> impl Iterator<Item = &str> {
    input.split('\n').filter(|line| !line.is_empty())
}

pub fn part_one(input: &str) -> Option<u64> {
    let ret = parse_input(input)
        .filter_map(|input| {
            let iter = input.chars().filter_map(|c| c.to_digit(10)).map(u64::from);

            iter.clone()
                .enumerate()
                .flat_map(|(i, a)| iter.clone().skip(i + 1).map(move |b| 10 * a + b))
                .max()
        })
        .sum();

    Some(ret)
}

#[cached(
    ty = "UnboundCache<(usize, u32), u64>",
    create = "{ UnboundCache::new() }",
    convert = r#"{ (input.len(), t) }"#
)]
fn part_two_wrapper(input: &[u64], t: u32, n: usize) -> u64 {
    if input.len() == (t + 1) as _ {
        (0..=t)
            .rev()
            .zip(input.iter().copied())
            .map(|(t, a)| a * 10_u64.pow(t))
            .sum()
    } else {
        input
            .first()
            .map(|first| {
                first * 10_u64.pow(t)
                    + if t == 0 {
                        0
                    } else {
                        part_two_wrapper(&input[1..], t - 1, n)
                    }
            })
            .into_iter()
            .chain(Some(part_two_wrapper(&input[1..], t, n)))
            .max()
            .unwrap_or_default()
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let ret = parse_input(input)
        .map(|input| {
            let digits = input
                .chars()
                .filter_map(|c| c.to_digit(10))
                .map(u64::from)
                .collect::<Vec<_>>();

            if let LockResult::Ok(mut locked) = PART_TWO_WRAPPER.lock() {
                locked.cache_clear();
            }

            part_two_wrapper(&digits, 11, digits.len())
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
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}

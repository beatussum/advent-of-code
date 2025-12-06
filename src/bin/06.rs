use itertools::Itertools;

advent_of_code::solution!(6);

#[derive(Clone, Copy, Debug)]
enum Operator {
    Add,
    Mul,
}

impl Operator {
    pub fn call(self, lhs: u64, rhs: u64) -> u64 {
        match self {
            Self::Add => lhs + rhs,
            Self::Mul => lhs * rhs,
        }
    }
}

impl TryFrom<char> for Operator {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '+' => Ok(Self::Add),
            '*' => Ok(Self::Mul),
            _ => Err("bad operators"),
        }
    }
}

fn parse_input_one(input: &str) -> Vec<(Vec<u64>, Operator)> {
    let parsed = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.split_ascii_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    parsed
        .split_last()
        .and_then(|(operators, numbers)| {
            let numbers = numbers.first().map(|first| {
                let (m, n) = (numbers.len(), first.len());

                (0..n)
                    .map(|j| {
                        (0..m)
                            .filter_map(|i| numbers.get(i).and_then(|row| row.get(j).copied()))
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
            });

            numbers.map(|numbers| (operators, numbers))
        })
        .map(|(operators, numbers)| {
            operators
                .iter()
                .copied()
                .filter_map(|operator| {
                    operator
                        .chars()
                        .next()
                        .and_then(|operator| TryFrom::try_from(operator).ok())
                })
                .zip(numbers)
                .map(|(operator, numbers)| {
                    let numbers = numbers
                        .iter()
                        .filter_map(|number| number.parse().ok())
                        .collect();

                    (numbers, operator)
                })
                .collect()
        })
        .unwrap_or_default()
}

fn parse_input_two(input: &str) -> Vec<(Vec<u64>, Operator)> {
    let parsed = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    parsed
        .split_last()
        .and_then(|(operators, numbers)| {
            let operators = operators
                .iter()
                .rev()
                .copied()
                .filter_map(|operator| TryFrom::try_from(operator).ok())
                .collect::<Vec<_>>();

            let numbers = numbers.first().map(|first| {
                let (m, n) = (numbers.len(), first.len());

                (0..n)
                    .map(|j| {
                        (0..m)
                            .filter_map(|i| numbers.get(i).and_then(|row| row.get(n - j).copied()))
                            .collect::<String>()
                            .trim()
                            .parse::<u64>()
                            .ok()
                    })
                    .chunk_by(|number| number.is_some())
                    .into_iter()
                    .filter_map(|(is_not_null, number)| {
                        is_not_null.then_some(number.flatten().collect::<Vec<_>>())
                    })
                    .collect::<Vec<_>>()
            });

            numbers.map(|numbers| (operators, dbg!(numbers)))
        })
        .map(|(operators, numbers)| numbers.into_iter().zip(operators).collect())
        .unwrap_or_default()
}

pub fn part_one(input: &str) -> Option<u64> {
    let ret = parse_input_one(input)
        .into_iter()
        .filter_map(|(numbers, operator)| {
            numbers
                .into_iter()
                .reduce(|lhs, rhs| operator.call(lhs, rhs))
        })
        .sum();

    Some(ret)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ret = parse_input_two(input)
        .into_iter()
        .filter_map(|(numbers, operator)| {
            numbers
                .into_iter()
                .reduce(|lhs, rhs| operator.call(lhs, rhs))
        })
        .sum();

    println!("{:?}", parse_input_two(input));

    Some(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}

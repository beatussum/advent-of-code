advent_of_code::solution!(1);

fn parse_input(input: &str) -> Vec<i16> {
    input
        .split('\n')
        .filter(|line| !line.is_empty())
        .flat_map(|line| {
            let val = line[1..].parse::<i16>();

            match line.chars().next() {
                Some('L') => val.map(|val| -val),
                Some('R') => val,
                _ => unreachable!(),
            }
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u64> {
    let ret = parse_input(input)
        .into_iter()
        .scan(50, |lhs, rhs| {
            *lhs = (*lhs + rhs).rem_euclid(100);
            Some(*lhs)
        })
        .filter(|&current| current == 0)
        .count();

    Some(ret as _)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ret = parse_input(input)
        .into_iter()
        .scan(50, |lhs @ &mut before, rhs| {
            let sum = before + rhs;
            *lhs = sum.rem_euclid(100);

            let ret = if rhs > 0 {
                sum / 100
            } else {
                let reversed = (100 - before).rem_euclid(100);
                (reversed - rhs) / 100
            };

            Some(ret)
        })
        .sum::<i16>();

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
        assert_eq!(result, Some(6));
    }
}

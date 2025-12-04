use std::iter::once;

advent_of_code::solution!(4);

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<bool>>,
}

impl Grid {
    pub fn new(grid: Vec<Vec<bool>>) -> Self {
        Self { grid }
    }

    pub fn get(&self, i: usize, j: usize) -> Option<bool> {
        self.grid.get(i).and_then(|row| row.get(j).copied())
    }

    pub fn get_mut(&mut self, i: usize, j: usize) -> Option<&mut bool> {
        self.grid.get_mut(i).and_then(|row| row.get_mut(j))
    }

    pub fn neighbors(&self, i: usize, j: usize) -> impl Iterator<Item = bool> {
        (i != 0)
            .then(|| i - 1)
            .into_iter()
            .chain(once(i))
            .chain(once(i.saturating_add(1)))
            .filter_map(|i| self.grid.get(i))
            .flat_map(move |row| {
                (j != 0)
                    .then(|| j - 1)
                    .into_iter()
                    .chain(once(j))
                    .chain(once(j.saturating_add(1)))
                    .filter_map(|j| row.get(j).copied())
            })
    }

    pub fn len(&self) -> Option<(usize, usize)> {
        self.grid
            .first()
            .map(|first| (self.grid.len(), first.len()))
    }

    pub fn remove(&mut self, i: usize, j: usize) {
        if let Some(cell) = self.get_mut(i, j) {
            *cell = false;
        }
    }
}

fn parse_input(input: &str) -> Grid {
    let grid = input
        .split('\n')
        .filter(|row| !row.is_empty())
        .map(|row| row.chars().map(|c| c == '@').collect())
        .collect();

    Grid::new(grid)
}

pub fn part_one(input: &str) -> Option<u64> {
    let input = parse_input(input);

    if let Some((m, n)) = input.len() {
        let ret = (0..m)
            .flat_map(|i| (0..n).map(move |j| (i, j)))
            .filter(|&(i, j)| {
                input.get(i, j).unwrap_or_default()
                    && input.neighbors(i, j).filter(|&current| current).count() <= 4
            })
            .count();

        Some(ret as _)
    } else {
        None
    }
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut input = parse_input(input);

    if let Some((m, n)) = input.len() {
        let mut ret = 0;

        loop {
            let forklift = (0..m)
                .flat_map(|i| (0..n).map(move |j| (i, j)))
                .filter(|&(i, j)| {
                    input.get(i, j).unwrap_or_default()
                        && input.neighbors(i, j).filter(|&current| current).count() <= 4
                })
                .collect::<Vec<_>>();

            if forklift.is_empty() {
                break;
            } else {
                for (i, j) in forklift {
                    input.remove(i, j);
                    ret += 1;
                }
            }
        }

        Some(ret)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}

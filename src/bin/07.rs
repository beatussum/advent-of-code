use std::{
    collections::{HashMap, HashSet},
    iter::once,
};

advent_of_code::solution!(7);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    Beam,
    Splitter,
    Start,
    Void,
}

impl TryFrom<char> for Cell {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '|' => Ok(Self::Beam),
            '^' => Ok(Self::Splitter),
            'S' => Ok(Self::Start),
            '.' => Ok(Self::Void),
            _ => Err("unrecognized character"),
        }
    }
}

#[derive(Debug)]
struct Situation {
    situation: Vec<Vec<Cell>>,
}

impl Situation {
    pub fn new(situation: Vec<Vec<Cell>>) -> Self {
        Self { situation }
    }

    pub fn get(&self, i: usize, j: usize) -> Option<&Cell> {
        self.situation.get(i).and_then(|row| row.get(j))
    }

    pub fn next_cells_mut(
        &mut self,
        i: usize,
        j: usize,
    ) -> impl Iterator<Item = ((usize, usize), &mut Cell)> {
        let will_be_split = self
            .get(i + 1, j)
            .map(|&next| next == Cell::Splitter)
            .unwrap_or_default();

        self.situation
            .get_mut(i + 1)
            .into_iter()
            .flatten()
            .enumerate()
            .filter_map(move |(jj, cell)| {
                (j != 0)
                    .then(|| j - 1)
                    .into_iter()
                    .chain(once(j + 1))
                    .filter(|_| will_be_split)
                    .chain((!will_be_split).then_some(j))
                    .any(|j| j == jj)
                    .then_some(((i + 1, jj), cell))
            })
    }
}

impl From<&str> for Situation {
    fn from(value: &str) -> Self {
        let situation = value
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(|row| {
                row.chars()
                    .filter_map(|cell| TryFrom::try_from(cell).ok())
                    .collect()
            })
            .collect();

        Self::new(situation)
    }
}

struct Manifold {
    situation: Situation,
    heads: HashSet<(usize, usize)>,
    head: (usize, usize),
}

impl Manifold {
    pub fn new(situation: Situation) -> Self {
        let heads = situation
            .situation
            .iter()
            .enumerate()
            .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, cell)| ((i, j), cell)))
            .filter_map(|(index, &cell)| (cell == Cell::Start).then_some(index))
            .collect::<HashSet<_>>();

        let head = heads.iter().next().copied().unwrap_or_default();

        Self {
            situation,
            heads,
            head,
        }
    }

    pub fn run(&mut self) {
        let mut heads = HashSet::default();

        for (i, j) in self.heads.iter().copied() {
            let iterator = self.situation.next_cells_mut(i, j);

            for (index, cell) in iterator {
                heads.insert(index);
                *cell = Cell::Beam;
            }
        }

        self.heads = heads;
    }

    pub fn can_run(&self) -> bool {
        !self.heads.is_empty()
    }

    pub fn count(&self) -> usize {
        self.situation
            .situation
            .iter()
            .enumerate()
            .flat_map(|(i, row)| row.iter().enumerate().map(move |(j, cell)| ((i, j), cell)))
            .filter_map(|(index, &cell)| (cell == Cell::Splitter).then_some(index))
            .filter(|&(i, j)| {
                if i != 0 {
                    self.situation.get(i - 1, j).copied() == Some(Cell::Beam)
                } else {
                    false
                }
            })
            .count()
    }

    fn timelines_wrapper(
        &self,
        i: usize,
        j: usize,
        memo: &mut HashMap<(usize, usize), usize>,
    ) -> usize {
        if !memo.contains_key(&(i, j)) {
            let to_insert = if (i, j) == self.head {
                1
            } else {
                (j != 0)
                    .then(|| j - 1)
                    .into_iter()
                    .chain(once(j + 1))
                    .filter(|&j| {
                        self.situation
                            .get(i, j)
                            .copied()
                            .map(|cell| cell == Cell::Splitter)
                            .unwrap_or_default()
                    })
                    .chain(once(j))
                    .filter(|&j| {
                        self.situation
                            .get(i - 1, j)
                            .copied()
                            .filter(|&cell| cell == Cell::Beam || cell == Cell::Start)
                            .is_some()
                    })
                    .map(|j| self.timelines_wrapper(i - 1, j, memo))
                    .sum()
            };

            memo.insert((i, j), to_insert);
        }

        memo.get(&(i, j)).copied().unwrap_or_default()
    }

    pub fn timelines(&self) -> usize {
        assert!(!self.can_run());

        let mut memo = HashMap::default();
        let m = self.situation.situation.len();

        self.situation
            .situation
            .last()
            .into_iter()
            .flatten()
            .copied()
            .enumerate()
            .filter_map(|(j, cell)| (cell == Cell::Beam).then_some((m - 1, j)))
            .map(|(i, j)| self.timelines_wrapper(i, j, &mut memo))
            .sum()
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let situation = From::from(input);
    let mut manifold = Manifold::new(situation);

    while manifold.can_run() {
        manifold.run();
    }

    Some(manifold.count() as _)
}

pub fn part_two(input: &str) -> Option<u64> {
    let situation = From::from(input);
    let mut manifold = Manifold::new(situation);

    while manifold.can_run() {
        manifold.run();
    }

    Some(manifold.timelines() as _)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}

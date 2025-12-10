use anyhow::Result;
use itertools::Itertools;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::read_to_string;

use winnow::combinator::{separated, separated_pair};
use winnow::{Parser, ascii::dec_uint};

type Pos = (usize, usize);

fn parse_(input: &mut &str) -> winnow::Result<Vec<Pos>> {
    separated(1.., separated_pair(dec_uint, ',', dec_uint), '\n').parse_next(input)
}

fn parse(input: &str) -> Result<Vec<Pos>> {
    parse_.parse(input).map_err(|e| anyhow::anyhow!("{e}"))
}

fn task1(input: &[Pos]) -> usize {
    input
        .iter()
        .enumerate()
        .flat_map(|(i, p1)| input.iter().skip(i + 1).map(move |p2| (*p1, *p2)))
        .map(|((x1, y1), (x2, y2))| (x2.abs_diff(x1) + 1) * (y2.abs_diff(y1) + 1))
        .max()
        .unwrap()
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Unknown,
    Empty,
    Red,
    Green,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unknown => write!(f, "_"),
            Self::Empty => write!(f, "."),
            Self::Red => write!(f, "#"),
            Self::Green => write!(f, "X"),
        }
    }
}

fn flood_fill(map: &mut Vec<Vec<Tile>>, (x, y): Pos, tile: Tile) {
    let mut queue = std::collections::VecDeque::new();
    queue.push_back((x, y));

    while let Some((x, y)) = queue.pop_front() {
        if let Some(t @ Tile::Unknown) = map.get_mut(y).and_then(|row| row.get_mut(x)) {
            *t = tile;
            for pos in [(1, 0), (-1, 0), (0, 1), (0, -1)]
                .iter()
                .filter_map(|&(dx, dy)| x.checked_add_signed(dx).zip(y.checked_add_signed(dy)))
                .filter_map(|(x, y)| (*map.get(y)?.get(x)? == Tile::Unknown).then_some((x, y)))
            {
                queue.push_back(pos);
            }
        }
    }
}

fn task2(input: &[Pos]) -> usize {
    use Tile::*;

    let xs: HashMap<usize, usize> = input
        .iter()
        .map(|&(x, _)| x)
        .unique()
        .sorted()
        .enumerate()
        .map(|(i, x)| (i + 1, x))
        .collect();
    let ys: HashMap<usize, usize> = input
        .iter()
        .map(|&(_, y)| y)
        .unique()
        .into_iter()
        .sorted()
        .enumerate()
        .map(|(i, y)| (i + 1, y))
        .collect();
    let xs_inv: HashMap<usize, usize> = xs.iter().map(|(&k, &v)| (v, k)).collect();
    let ys_inv: HashMap<usize, usize> = ys.iter().map(|(&k, &v)| (v, k)).collect();

    let input: Vec<_> = input
        .into_iter()
        .map(|(x, y)| (xs_inv[x], ys_inv[y]))
        .collect();

    let mut map = vec![vec![Unknown; xs.len() + 2]; ys.len() + 2];

    let wrap = [*input.first().unwrap(), *input.last().unwrap()];
    for edge in input.windows(2).chain([&wrap[..]]) {
        let ((x1, y1), (x2, y2)) = (edge[0], edge[1]);
        map[y1][x1] = Red;
        map[y2][x2] = Red;
        if x1 == x2 {
            for y in y1.min(y2) + 1..y1.max(y2) {
                map[y][x1] = Green;
            }
        } else {
            for x in x1.min(x2) + 1..x1.max(x2) {
                map[y1][x] = Green;
            }
        }
    }

    #[cfg(debug_assertions)]
    print(&map);

    flood_fill(&mut map, (0, 0), Empty);
    for t in map.iter_mut().flatten() {
        if *t == Unknown {
            *t = Green;
        }
    }

    #[cfg(debug_assertions)]
    print(&map);

    input
        .par_iter()
        .enumerate()
        .flat_map(|(i, p1)| input.par_iter().skip(i + 1).map(move |p2| (*p1, *p2)))
        .filter(|&((x1, y1), (x2, y2))| {
            (y1.min(y2)..=y1.max(y2))
                .all(|y| (x1.min(x2)..x1.max(x2)).all(|x| matches!(map[y][x], Red | Green)))
        })
        .map(|((x1, y1), (x2, y2))| {
            (xs[&x2].abs_diff(xs[&x1]) + 1) * (ys[&y2].abs_diff(ys[&y1]) + 1)
        })
        .max()
        .unwrap()
}

fn print(map: &Vec<Vec<Tile>>) {
    for line in map {
        let s: String = line.iter().map(|&t| format!("{:?}", t)).collect();
        println!("{}", s);
    }
}

fn main() -> Result<()> {
    let input = parse(&read_to_string("input.txt")?)?;
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_task1() -> Result<()> {
        let input = parse(INPUT)?;
        assert_eq!(task1(&input), 50);
        Ok(())
    }

    #[test]
    fn test_task2() -> Result<()> {
        let input = parse(INPUT)?;
        assert_eq!(task2(&input), 24);
        Ok(())
    }

    #[test]
    fn test_main() -> Result<()> {
        main()
    }
}

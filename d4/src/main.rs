use std::fs::read_to_string;

use anyhow::Result;
use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Roll,
}

fn parse(input: &str) -> Vec<Vec<Tile>> {
    let f = |c| match c {
        '.' => Tile::Empty,
        '@' => Tile::Roll,
        _ => unreachable!(),
    };
    input.lines().map(|l| l.chars().map(f).collect()).collect()
}

const STEPS: [isize; 3] = [-1, 0, 1];
type NeighborCache = Vec<Vec<Vec<(usize, usize)>>>;

fn precompute_neighbors(h: usize, w: usize) -> NeighborCache {
    let neighbors: Vec<_> = STEPS
        .into_iter()
        .cartesian_product(STEPS)
        .filter(|&s| s != (0, 0))
        .collect();
    let f = |y: usize, x: usize| {
        neighbors
            .iter()
            .filter_map(|&(dx, dy)| x.checked_add_signed(dx).zip(y.checked_add_signed(dy)))
            .filter(|&(nx, ny)| nx < w && ny < h)
            .collect()
    };
    (0..h).map(|y| (0..w).map(|x| f(y, x)).collect()).collect()
}

fn task1(input: &[Vec<Tile>], nc: &NeighborCache) -> impl Iterator<Item = (usize, usize)> {
    (0..input.len())
        .cartesian_product(0..input[0].len())
        .filter(move |&(x, y)| {
            input[y][x] == Tile::Roll
                && nc[y][x]
                    .iter()
                    .filter(|&&(x_, y_)| input[y_][x_] == Tile::Roll)
                    .count()
                    < 4
        })
}

fn task2(mut input: Vec<Vec<Tile>>, nc: &NeighborCache) -> usize {
    let mut count = 0;
    loop {
        let accessible_tiles: Vec<_> = task1(&input, nc).collect();

        if accessible_tiles.is_empty() {
            break;
        }
        count += accessible_tiles.len();
        for (x, y) in accessible_tiles {
            input[y][x] = Tile::Empty;
        }
    }

    count
}

fn main() -> Result<()> {
    let input = parse(&read_to_string("input.txt")?);
    let neighbor_cache = precompute_neighbors(input.len(), input[0].len());

    println!("Task 1: {}", task1(&input, &neighbor_cache).count());
    println!("Task 2: {}", task2(input.clone(), &neighbor_cache));
    Ok(())
}

#[cfg(test)]
mod tests;

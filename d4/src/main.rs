use std::fs::read_to_string;

use anyhow::Result;
use itertools::Itertools;
use winnow::combinator::{dispatch, empty, fail, repeat, separated};
use winnow::{Parser, token::take};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Roll,
}

fn parse_(input: &mut &str) -> winnow::Result<Vec<Vec<Tile>>> {
    let parse_tile = dispatch!(take(1usize);
        "." => empty.value(Tile::Empty),
        "@" => empty.value(Tile::Roll),
        _ => fail
    );

    separated(1.., repeat::<_, _, Vec<Tile>, _, _>(1.., parse_tile), '\n').parse_next(input)
}

fn parse(input: &str) -> Result<Vec<Vec<Tile>>> {
    parse_.parse(input).map_err(|e| anyhow::anyhow!("{e}"))
}

const STEPS: [isize; 3] = [-1, 0, 1];

fn task1(input: &[Vec<Tile>]) -> impl Iterator<Item = (usize, usize)> {
    (0..input.len())
        .cartesian_product(0..input[0].len())
        .filter(|&(x, y)| {
            input[y][x] == Tile::Roll
                && STEPS
                    .into_iter()
                    .cartesian_product(STEPS)
                    .filter_map(|(dx, dy)| {
                        input
                            .get(y.checked_add_signed(dy)?)?
                            .get(x.checked_add_signed(dx)?)
                    })
                    .filter(|&&tile| tile == Tile::Roll)
                    .count()
                    < 5
        })
}

fn task2(mut input: Vec<Vec<Tile>>) -> usize {
    let mut count = 0;
    loop {
        let accessible_tiles: Vec<_> = task1(&input).collect();
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
    // "/Users/astadnik/misc/advents_2025/d4/input.txt",
    let input = parse(&read_to_string("input.txt")?)?;
    println!("Task 1: {}", task1(&input).count());
    println!("Task 2: {}", task2(input.clone()));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_task1() -> Result<()> {
        let input = parse(INPUT)?;
        assert_eq!(task1(&input).count(), 13);
        Ok(())
    }

    #[test]
    fn test_task2() -> Result<()> {
        let input = parse(INPUT)?;
        assert_eq!(task2(input), 43);
        Ok(())
    }

    #[test]
    fn test_main() -> Result<()> {
        main()
    }
}

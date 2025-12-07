use anyhow::Result;
use std::{collections::HashSet, fs::read_to_string};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Split,
    Start,
}

type T = u64;
type Grid = Vec<Vec<Tile>>;

fn parse(input: &str) -> Grid {
    let f = |c| match c {
        '.' => Tile::Empty,
        '^' => Tile::Split,
        'S' => Tile::Start,
        _ => unreachable!(),
    };
    input.lines().map(|l| l.chars().map(f).collect()).collect()
}

fn task1(input: &Grid) -> T {
    let mut beams = Vec::from([input[0].iter().position(|&t| t == Tile::Start).unwrap()]);
    let mut ans = 0;

    for l in &input[1..] {
        beams = beams
            .into_iter()
            .flat_map(|i| match l[i] {
                Tile::Empty => vec![i],
                Tile::Split => {
                    ans += 1;
                    vec![i - 1, i + 1]
                }
                Tile::Start => unreachable!(),
            })
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
    }
    ans
}

fn task2(input: &Grid) -> T {
    let mut beams = vec![0; input[0].len()];
    beams[input[0].iter().position(|&t| t == Tile::Start).unwrap()] = 1;

    for l in &input[1..] {
        let mut new_beams = vec![0; input[0].len()];
        for (i, v) in l.iter().enumerate() {
            match v {
                Tile::Empty => new_beams[i] += beams[i],
                Tile::Split => {
                    new_beams[i - 1] += beams[i];
                    new_beams[i + 1] += beams[i];
                }
                Tile::Start => unreachable!(),
            }
        }
        beams = new_beams;
    }
    beams.iter().sum()
}

fn main() -> Result<()> {
    let input = parse(&read_to_string("input.txt")?);
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_task1() -> Result<()> {
        let input = parse(INPUT);
        assert_eq!(task1(&input), 21);
        Ok(())
    }

    #[test]
    fn test_task2() -> Result<()> {
        let input = parse(INPUT);
        assert_eq!(task2(&input), 40);
        Ok(())
    }

    #[test]
    fn test_main() -> Result<()> {
        main()
    }
}

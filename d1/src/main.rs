use anyhow::{Result, anyhow};
use std::fs::read_to_string;
use winnow::Parser;
use winnow::ascii::{alpha1, dec_uint};
use winnow::combinator::{dispatch, empty, fail, separated, seq};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    L,
    R,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rot(Direction, u32);

fn parse_(input: &mut &str) -> winnow::Result<Vec<Rot>> {
    let mut parse_direction = dispatch!(alpha1;
        "L" => empty.value(Direction::L),
        "R" => empty.value(Direction::R),
        _ => fail
    );
    separated(1.., seq! {Rot(parse_direction,dec_uint)}, "\n").parse_next(input)
}

fn parse(input: &str) -> Result<Vec<Rot>> {
    parse_.parse(input).map_err(|e| anyhow!("{e}"))
}

fn task1(input: &[Rot]) -> u32 {
    let mut rez = 0;
    let mut pos = 50;
    for rot in input {
        match rot.0 {
            Direction::L => pos -= rot.1 as i32,
            Direction::R => pos += rot.1 as i32,
        }
        pos = pos.rem_euclid(100);
        if pos == 0 {
            rez += 1;
        }
    }
    rez
}

fn task2(input: &[Rot]) -> u32 {
    let mut rez = 0;
    let mut pos = 50;
    for rot in input {
        eprintln!("Rot: {rot:?}, pos: {pos}, rez: {rez}");
        match rot.0 {
            Direction::L => {
                rez += (rot.1 + (100 - pos as u32) % 100) / 100;
                pos -= rot.1 as i32;
            }
            Direction::R => {
                rez += (rot.1 + pos as u32) / 100;
                pos += rot.1 as i32;
            }
        }
        pos = pos.rem_euclid(100);
    }
    rez
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

    const INPUT: &str = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_task1() -> Result<()> {
        assert_eq!(task1(&parse(INPUT)?), 3);
        Ok(())
    }

    #[test]
    fn test_task2() -> Result<()> {
        assert_eq!(task2(&parse(INPUT)?), 6);
        Ok(())
    }

    #[test]
    fn test_task2_1() -> Result<()> {
        assert_eq!(task2(&parse("R1000")?), 10);
        Ok(())
    }

    #[test]
    fn test_task2_2() -> Result<()> {
        assert_eq!(task2(&parse("L1000")?), 10);
        Ok(())
    }

    #[test]
    fn test_main() -> Result<()> {
        main()
    }
}

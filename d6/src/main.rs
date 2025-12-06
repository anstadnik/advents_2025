use anyhow::Result;
use std::fs::read_to_string;

use winnow::ascii::{dec_uint, space1};
use winnow::combinator::{separated, separated_pair};
use winnow::{Parser, token::one_of};

type T = u32;
type Input = (Vec<Vec<T>>, Vec<Vec<char>>);
fn parse_(input: &mut &str) -> winnow::Result<Input> {
    let parse_row = separated(1.., dec_uint::<_, T, _>, space1).map(|v: Vec<T>| v);
    let parse_nums = separated(1.., parse_row, "\n").map(|v: Vec<Vec<T>>| v);
    let parse_op_row = separated(1.., one_of(['+', '*']), space1).map(|v: Vec<char>| v);
    let parse_ops = separated(1.., parse_op_row, "\n").map(|v: Vec<Vec<char>>| v);
    separated_pair(parse_nums, '\n', parse_ops).parse_next(input)
}

fn parse(input: &str) -> Result<Input> {
    parse_.parse(input).map_err(|e| anyhow::anyhow!("{e}"))
}

fn task1(_input: &Input) -> Result<i32> {
    Ok(0)
}

fn task2(_input: &Input) -> Result<i32> {
    Ok(0)
}

fn main() -> Result<()> {
    let input = parse(&read_to_string("input.txt")?)?;
    println!("Task 1: {}", task1(&input)?);
    println!("Task 2: {}", task2(&input)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"123 328  51 64
     45 64  387 23
      6 98  215 314
    *   +   *   +  ";

    #[test]
    fn test_task1() -> Result<()> {
        let input = parse(INPUT)?;
        assert_eq!(task1(&input)?, 4277556);
        Ok(())
    }

    #[test]
    fn test_task2() -> Result<()> {
        let input = parse(INPUT)?;
        assert_eq!(task2(&input)?, 42);
        Ok(())
    }

    #[test]
    fn test_main() -> Result<()> {}
}

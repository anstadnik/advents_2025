use anyhow::Result;
use std::fs::read_to_string;

use winnow::combinator::{separated, separated_pair};
use winnow::{Parser, ascii::dec_uint};

type T = usize;
type Input = (Vec<(T, T)>, Vec<T>);

fn parse_(input: &mut &str) -> winnow::Result<Input> {
    let parse_ranges = separated(1.., separated_pair(dec_uint, '-', dec_uint), "\n");
    let parse_ingredients = separated(1.., dec_uint::<_, T, _>, '\n');
    separated_pair(parse_ranges, "\n\n", parse_ingredients).parse_next(input)
}

fn parse(input: &str) -> Result<Input> {
    parse_.parse(input).map_err(|e| anyhow::anyhow!("{e}"))
}

fn task1((ranges, ingredients): &Input) -> usize {
    let pred = |&&i: &&usize| ranges.iter().any(|&(min, max)| min <= i && i <= max);
    ingredients.iter().filter(pred).count()
}

fn task2((mut ranges, _): Input) -> usize {
    ranges.sort_unstable();
    let mut next_uncovered = 0;
    let mut count = 0;

    for &(min, max) in &ranges {
        if next_uncovered <= max {
            count += max - min.max(next_uncovered) + 1;
            next_uncovered = max + 1;
        }
    }
    count
}

fn main() -> Result<()> {
    let input = parse(&read_to_string("input.txt")?)?;
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(input));
    Ok(())
}

#[cfg(test)]
mod tests;

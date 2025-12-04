use std::cmp::Reverse;
use std::fs::read_to_string;

use anyhow::Result;
use winnow::ascii::dec_uint;
use winnow::combinator::{repeat, separated};
use winnow::{Parser, token::take};

fn parse_(input: &mut &str) -> winnow::Result<Vec<Vec<u8>>> {
    let line = repeat::<_, _, Vec<u8>, _, _>(1.., take(1usize).and_then(dec_uint::<_, u8, _>));
    separated(1.., line, '\n').parse_next(input)
}

fn parse(input: &str) -> Result<Vec<Vec<u8>>> {
    parse_.parse(input).map_err(|e| anyhow::anyhow!("{e}"))
}

fn task1(input: &[Vec<u8>], n: usize) -> Result<u64> {
    Ok(input
        .iter()
        .map(|row| {
            let mut sum = 0;
            let mut prev_best = 0;

            for it in (0..n).rev() {
                let (i, &n1) = row[prev_best..row.len() - it]
                    .iter()
                    .enumerate()
                    .max_by_key(|&(i, &v)| (v, Reverse(i)))
                    .unwrap();
                prev_best += i + 1;
                sum = sum * 10 + n1 as u64;
            }
            sum
        })
        .sum())
}

fn main() -> Result<()> {
    let input = parse(&read_to_string("input.txt")?)?;
    println!("Task 1: {}", task1(&input, 2)?);
    println!("Task 2: {}", task1(&input, 12)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_task1() -> Result<()> {
        let input = parse(INPUT)?;
        assert_eq!(task1(&input, 2)?, 357);
        Ok(())
    }

    #[test]
    fn test_task2() -> Result<()> {
        let input = parse(INPUT)?;
        assert_eq!(task1(&input, 12)?, 3121910778619);
        Ok(())
    }

    #[test]
    fn test_main() -> Result<()> {
        main()
    }
}

use std::fs::read_to_string;
use std::ops::RangeInclusive;

use anyhow::Result;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use winnow::combinator::{separated, separated_pair};
use winnow::{Parser, ascii::dec_uint};

type T = u64;
fn parse_(input: &mut &str) -> winnow::Result<Vec<(T, T)>> {
    separated(1.., separated_pair(dec_uint, '-', dec_uint), ',').parse_next(input)
}

fn parse(input: &str) -> Result<Vec<(T, T)>> {
    parse_.parse(input).map_err(|e| anyhow::anyhow!("{e}"))
}

fn task1(input: &[(T, T)], n_reps: RangeInclusive<usize>) -> Result<T> {
    Ok(input
        .par_iter()
        .flat_map(|&(start, end)| start..=end)
        .filter(|&n| {
            let s = n.to_string().into_bytes();
            let l = s.len();
            n_reps.clone().any(|n_rep| {
                let step = l / n_rep;
                l % n_rep == 0 && s.chunks(step).skip(1).all(|c| c == &s[..step])
            })
        })
        .sum())
}

fn main() -> Result<()> {
    let input = parse(&read_to_string("input.txt")?)?;
    println!("Task 1: {}", task1(&input, 2..=2)?);
    println!("Task 2: {}", task1(&input, 2..=6)?);
    Ok(())
}

#[cfg(test)]
mod tests {

    use super::*;

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
    1698522-1698528,446443-446449,38593856-38593862,565653-565659,
    824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_task1() -> Result<()> {
        let input = parse(&INPUT.replace([' ', '\n'], ""))?;
        assert_eq!(task1(&input, 2..=2)?, 1227775554);
        Ok(())
    }

    #[test]
    fn test_task2() -> Result<()> {
        let input = parse(&INPUT.replace([' ', '\n'], ""))?;
        assert_eq!(task1(&input, 2..=6)?, 4174379265);
        Ok(())
    }

    #[test]
    fn test_main() -> Result<()> {
        main()
    }
}

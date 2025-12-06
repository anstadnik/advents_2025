use anyhow::Result;
use std::fs::read_to_string;

use winnow::ascii::{dec_uint, space0, space1};
use winnow::combinator::{delimited, separated, separated_pair, terminated};
use winnow::{Parser, token::one_of};

type T = u64;
type Input = Vec<(Vec<T>, char)>;

fn parse_(input: &mut &str) -> winnow::Result<(Vec<Vec<T>>, Vec<char>)> {
    let parse_row = separated(1.., dec_uint::<_, T, _>, space1).map(|v: Vec<T>| v);
    let parse_nums = separated(1.., delimited(space0, parse_row, space0), "\n");
    let parse_ops = separated(1.., one_of(['+', '*']), space1);
    separated_pair(parse_nums, '\n', terminated(parse_ops, space0)).parse_next(input)
}

fn parse(input: &str) -> Result<Input> {
    let (rows, ops) = parse_.parse(input).map_err(|e| anyhow::anyhow!("{e}"))?;
    Ok((0..ops.len())
        .map(|i| rows.iter().map(|row| row[i]).collect())
        .zip(ops)
        .collect())
}

fn task1(input: &Input) -> T {
    input
        .iter()
        .map(|(row, op)| match op {
            '+' => row.iter().sum::<T>(),
            '*' => row.iter().product(),
            _ => unreachable!(),
        })
        .sum()
}

fn task2(input: &str) -> Result<T> {
    let data: Vec<_> = input.split('\n').map(str::as_bytes).collect();
    let (height, width, mut ans, mut nums) = (data.len(), data[0].len(), 0, Vec::new());

    for col in (0..width).rev() {
        let num = data[..height - 1]
            .iter()
            .filter_map(|row| row.get(col).filter(|&n| n.is_ascii_digit()))
            .fold(0, |acc, n| acc * 10 + (n - b'0') as T);
        if num != 0 {
            nums.push(num);
        }

        ans += match data[height - 1].get(col) {
            Some(b'+') => nums.iter().sum::<T>(),
            Some(b'*') => nums.iter().product::<T>(),
            Some(b' ') | None => continue,
            _ => unreachable!(),
        };
        nums.clear();
    }
    Ok(ans)
}

fn main() -> Result<()> {
    let s = read_to_string("input.txt")?;
    let input = parse(&s)?;
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&s)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "123 328  51 64 \n 45 64  387 23 \n  6 98  215 314\n*   +   *   +  ";

    #[test]
    fn test_task1() -> Result<()> {
        let input = parse(INPUT)?;
        assert_eq!(task1(&input), 4277556);
        Ok(())
    }

    #[test]
    fn test_task2() -> Result<()> {
        assert_eq!(task2(INPUT)?, 3263827);
        Ok(())
    }

    #[test]
    fn test_main() -> Result<()> {
        main()
    }
}

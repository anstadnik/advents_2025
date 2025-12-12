use anyhow::Result;
use itertools::Itertools;
use microlp::{ComparisonOp, OptimizationDirection, Problem};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::fs::read_to_string;
use std::usize;

use winnow::combinator::{delimited, dispatch, empty, fail, repeat, separated, seq};
use winnow::{Parser, ascii::dec_uint, token::take};

#[derive(Debug, Clone, PartialEq, Eq)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
}

fn parse_(input: &mut &str) -> winnow::Result<Vec<Machine>> {
    let mut parse_lights = delimited(
        '[',
        repeat(
            1..,
            dispatch! {
                take(1usize);
                "." => empty.value(false),
                "#" => empty.value(true),
                _ => fail
            },
        ),
        ']',
    );
    let mut parse_buttons = separated(
        1..,
        delimited('(', separated(1.., dec_uint::<_, usize, _>, ','), ')').map(|v: Vec<_>| v),
        ' ',
    );
    let mut parse_joltage = delimited('{', separated(1.., dec_uint::<_, usize, _>, ','), '}');
    let parse_machine = seq! {Machine {
        lights: parse_lights,
        _: ' ',
        buttons: parse_buttons,
        _: ' ',
        joltage: parse_joltage,

    }};
    separated(1.., parse_machine, "\n").parse_next(input)
}

fn parse(input: &str) -> Result<Vec<Machine>> {
    parse_.parse(input).map_err(|e| anyhow::anyhow!("{e}"))
}

fn task1(input: &[Machine]) -> usize {
    input
        .iter()
        .map(|m| {
            for n in 0..m.buttons.len() {
                for comb in m.buttons.iter().combinations(n) {
                    let mut light = vec![false; m.lights.len()];
                    for button in comb {
                        for i in button {
                            light[*i] ^= true;
                        }
                    }
                    if light.iter().zip(&m.lights).all(|(&a, &b)| a == b) {
                        return n;
                    }
                }
            }
            unreachable!()
        })
        .sum()
}

fn task2(input: Vec<Machine>) -> usize {
    input
        .into_par_iter()
        .map(|m| {
            let mut prob = Problem::new(OptimizationDirection::Minimize);

            let num_buttons = m.buttons.len();

            // Create variables for each button (how many times to press it)
            let vars: Vec<_> = (0..num_buttons)
                .map(|_| prob.add_integer_var(1.0, (0, 300)))
                .collect();

            for (joltage_idx, &target) in m.joltage.iter().enumerate() {
                let lhs = m
                    .buttons
                    .iter()
                    .zip(&vars)
                    .filter(|(button, _)| button.contains(&joltage_idx))
                    .map(|(_, var)| (*var, 1.0));
                prob.add_constraint(lhs, ComparisonOp::Eq, target as f64);
            }

            prob.solve().ok().unwrap().objective().round() as usize
        })
        .sum()
}

fn main() -> Result<()> {
    let input = parse(&read_to_string("input.txt")?)?;
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_task1() -> Result<()> {
        let input = parse(INPUT)?;
        assert_eq!(task1(&input), 7);
        Ok(())
    }

    #[test]
    fn test_task2() -> Result<()> {
        let input = parse(INPUT)?;
        assert_eq!(task2(input), 33);
        Ok(())
    }

    #[test]
    fn test_main() -> Result<()> {
        main()
    }
}

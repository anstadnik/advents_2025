use anyhow::Result;
use indicatif::ParallelProgressIterator;
use itertools::Itertools;
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

fn press_buttons(
    n_pressed: usize,
    min_n_pressed: &mut usize,
    button_id: usize,
    buttons: &[Vec<usize>],
    joltages: &mut Vec<usize>,
    target: &[usize],
    joltages_sum: usize,
    target_sum: usize,
    max_target: usize,
) {
    if n_pressed >= *min_n_pressed
        || joltages_sum > target_sum
        || button_id >= buttons.len()
        || joltages.iter().zip(target).any(|(&a, &b)| a > b)
    {
        return;
    }

    if target_sum == joltages_sum && joltages.iter().zip(target).all(|(&a, &b)| a == b) {
        *min_n_pressed = n_pressed;
        return;
    }

    let button = &buttons[button_id];
    for n in 0..=max_target {
        if n > 0 {
            for &i in button.iter() {
                joltages[i] += n;
            }
        }

        press_buttons(
            n_pressed + n,
            min_n_pressed,
            button_id + 1,
            buttons,
            joltages,
            target,
            joltages_sum + n * button.len(),
            target_sum,
            max_target,
        );

        if n > 0 {
            for &i in button.iter() {
                joltages[i] -= n;
            }
        }
    }
}

fn task2(input: Vec<Machine>) -> usize {
    input
        .into_par_iter()
        .progress()
        .map(|mut m| {
            m.buttons.sort_unstable_by_key(|v| v.len());
            m.buttons.reverse();
            let mut joltage = vec![0; m.lights.len()];
            let mut min_n_pressed = usize::MAX;
            press_buttons(
                0,
                &mut min_n_pressed,
                0,
                &m.buttons,
                &mut joltage,
                &m.joltage,
                0,
                m.joltage.iter().sum(),
                *m.joltage.iter().max().unwrap(),
            );
            min_n_pressed
        })
        .sum()
}

fn main() -> Result<()> {
    let input = parse(&read_to_string("input.txt")?)?;
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(input[..10].to_vec()));
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

    // #[test]
    // fn test_main() -> Result<()> {
    //     main()
    // }
}

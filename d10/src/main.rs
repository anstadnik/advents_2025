use anyhow::Result;
use indicatif::ParallelProgressIterator;
use indicatif::ProgressIterator;
use indicatif::{ProgressBar, ProgressStyle};
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::cmp::Reverse;
use std::cmp::max;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::read_to_string;
use std::usize;

use winnow::combinator::{delimited, dispatch, empty, fail, repeat, separated, seq};
use winnow::{Parser, ascii::dec_uint, token::take};

type T = u16;
#[derive(Debug, Clone, PartialEq, Eq)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<T>,
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
    let mut parse_joltage = delimited('{', separated(1.., dec_uint::<_, T, _>, ','), '}');
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

fn press_buttons(buttons: Vec<Vec<usize>>, target: &[T]) -> usize {
    println!("{buttons:?}, {target:?}");
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} [{elapsed_precise}] queue: {pos}, {msg}")
            .unwrap(),
    );

    let mut memory = HashMap::new();
    let mut queue = BinaryHeap::new();

    let start = vec![0; target.len()];
    queue.push(Reverse((*target.iter().max().unwrap(), 0, start)));

    let mut best_target: Option<usize> = None;
    while let Some(Reverse((max_dist, n_pressed, joltages))) = queue.pop() {
        if best_target.is_some_and(|best| n_pressed >= best)
            || memory.get(&joltages).is_some_and(|&v| n_pressed >= v)
        {
            continue;
        }

        pb.set_position(queue.len() as u64);
        pb.set_message(format!(
            "max dist = {max_dist}, best = {best_target:?}, n_pressed = {n_pressed}",
        ));

        if max_dist == 0 {
            best_target = Some(n_pressed);
        }

        memory.insert(joltages.clone(), n_pressed);

        queue.extend(buttons.iter().filter_map(|b| {
            let j = b.iter().fold(joltages.clone(), |mut acc, &i| {
                acc[i] += 1;
                acc
            });
            j.iter()
                .zip(target)
                .try_fold(0, |acc, (&a, &b)| b.checked_sub(a).map(|d| d.max(acc)))
                .map(|d| Reverse((d, n_pressed + 1, j)))
        }));
    }
    best_target.unwrap_or(0)
}

fn task2(input: Vec<Machine>) -> usize {
    input
        .into_iter()
        .progress()
        // .into_par_iter()
        // .progress()
        .map(|m| press_buttons(m.buttons, &m.joltage))
        .sum()
}

fn main() -> Result<()> {
    let input = parse(&read_to_string("input.txt")?)?;
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(input));
    // println!("Task 2: {}", task2(vec![input[6].clone()]));
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

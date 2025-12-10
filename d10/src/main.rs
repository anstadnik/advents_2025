use anyhow::Result;
use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::collections::HashMap;
use std::fs::read_to_string;
use std::usize;

use winnow::combinator::{delimited, dispatch, empty, fail, repeat, separated, seq};
use winnow::{Parser, ascii::dec_uint, token::take};

// type T = u16;
type T = usize;
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

fn get_joltages(
    buttons: &[Vec<usize>],
    target: Vec<usize>,
    map: &mut HashMap<Vec<usize>, Option<usize>>,
    depth_available: Option<usize>,
) -> Option<usize> {
    if let Some(&result) = map.get(&target) {
        return result;
    }

    if depth_available.is_some_and(|da| da == 0) {
        return None;
    }

    let mut best: Option<usize> = None;
    'outer: for button in buttons {
        let mut new_target = target.to_vec();
        for &b in button {
            if new_target[b] == 0 {
                continue 'outer;
            }
            new_target[b] -= 1;
        }
        let new_depth = match (best, depth_available) {
            (Some(b), Some(da)) => Some(b.min(da).saturating_sub(1)),
            (Some(b), None) => Some(b.saturating_sub(1)),
            (None, da) => da.map(|d| d.saturating_sub(1)),
        };
        if let Some(result) = get_joltages(buttons, new_target, map, new_depth) {
            best = Some(best.map_or(result + 1, |best| best.min(result + 1)));
        }
    }
    map.insert(target, best);
    best
}

fn task2(input: Vec<Machine>) -> usize {
    input
        // .into_iter()
        .into_par_iter()
        .progress()
        .map(|m| {
            let mut map = HashMap::new();
            map.insert(vec![0; m.joltage.len()], Some(0));
            get_joltages(&m.buttons, m.joltage, &mut map, None).unwrap()
        })
        .sum()
}

fn main() -> Result<()> {
    let input = parse(&read_to_string("input.txt")?)?;
    println!("Task 1: {}", task1(&input));
    // println!("Task 2: {}", task2(input[..10].to_vec()));
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

    // #[test]
    // fn test_main() -> Result<()> {
    //     main()
    // }
}

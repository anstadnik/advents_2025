use anyhow::Result;
use std::fs::read_to_string;
use std::iter::Flatten;

use winnow::combinator::{dispatch, empty, fail, repeat, separated, separated_pair, seq};
use winnow::{Parser, ascii::dec_uint, token::take};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Present {
    shape: [[bool; 3]; 3],
}

impl Present {
    fn rotate(&mut self) {
        let mut new_shape = [[false; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                new_shape[j][2 - i] = self.shape[i][j];
            }
        }
        self.shape = new_shape;
    }

    fn flip(&mut self) {
        let mut new_shape = [[false; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                new_shape[2 - i][j] = self.shape[i][j];
            }
        }
        self.shape = new_shape;
    }
}

impl IntoIterator for Present {
    type Item = bool;
    type IntoIter = Flatten<std::array::IntoIter<[bool; 3], 3>>;

    fn into_iter(self) -> Self::IntoIter {
        self.shape.into_iter().flatten()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Region {
    height: usize,
    width: usize,
    n_presents: Vec<usize>,
}

type Input = (Vec<Present>, Vec<Region>);
fn parse_(input: &mut &str) -> winnow::Result<Input> {
    let parse_present = seq! {Present {
        _: dec_uint::<_, usize, _>,
        _: ":\n",
        shape: separated(
            3,
            repeat(
                3,
                dispatch! {
                    take(1usize);
                    "#" => empty.value(true),
                    "." => empty.value(false),
                    _ => fail
                },
            ).map(|v: Vec<bool>| [v[0], v[1], v[2]]),
            "\n"
        ).map(|v: Vec<[bool; 3]>| [v[0], v[1], v[2]]),
    }};
    let parse_presents = separated(1.., parse_present, "\n\n");

    let parse_region = seq! {Region {
        width: dec_uint,
        _: 'x',
        height: dec_uint,
        _: ": ",
        n_presents: separated(1.., dec_uint::<_, usize, _>, ' ').map(|v: Vec<_>| v),
    }};
    let parse_regions = separated(1.., parse_region, "\n");

    separated_pair(parse_presents, "\n\n", parse_regions).parse_next(input)
}

fn parse(input: &str) -> Result<Input> {
    parse_.parse(input).map_err(|e| anyhow::anyhow!("{e}"))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Fitter<'a> {
    presents: &'a [Present],
    n_presents: Vec<usize>,
    region: Vec<Vec<bool>>,
}

impl<'a> Fitter<'a> {
    fn new(
        presents: &'a [Present],
        Region {
            height,
            width,
            n_presents,
        }: Region,
    ) -> Self {
        Self {
            presents,
            n_presents,
            region: vec![vec![false; width]; height],
        }
    }

    fn iter_present(
        &mut self,
        x: usize,
        y: usize,
        present: Present,
    ) -> impl Iterator<Item = &mut bool> + use<'_> {
        self.region[y..y + 3]
            .iter_mut()
            .flat_map(move |row| row[x..x + 3].iter_mut())
            .zip(present)
            .filter(|(_, p)| *p)
            .map(|(r, _)| r)
    }

    fn solve(&mut self) -> bool {
        if self
            .presents
            .iter()
            .zip(&self.n_presents)
            .map(|(p, n)| p.into_iter().filter(|&p| p).count() * n)
            .sum::<usize>()
            > self.region.len() * self.region[0].len()
        {
            return false;
        }

        if (self.region.len() / 3) * (self.region[0].len() / 3) >= self.n_presents.iter().sum() {
            return true;
        }

        self.solve_bruteforce()
    }

    fn solve_bruteforce(&mut self) -> bool {
        let Some(present_i) = self.n_presents.iter().position(|n| *n > 0) else {
            return true;
        };
        self.n_presents[present_i] -= 1;
        let mut present = self.presents[present_i];

        for _n_flip in 0..2 {
            for _n_rot in 0..4 {
                for x in 0..=self.region[0].len() - 3 {
                    for y in 0..=self.region.len() - 3 {
                        if self.iter_present(x, y, present).all(|v| !*v) {
                            for v in self.iter_present(x, y, present) {
                                *v = true;
                            }
                            if self.solve_bruteforce() {
                                return true;
                            }
                            for v in self.iter_present(x, y, present) {
                                *v = false;
                            }
                        }
                    }
                }
                present.rotate();
            }
            present.flip()
        }

        self.n_presents[present_i] += 1;
        false
    }
}

fn task1((presents, regions): &Input) -> usize {
    regions
        .iter()
        .map(|r| Fitter::new(&presents, r.clone()).solve() as usize)
        .sum()
}

fn main() -> Result<()> {
    let input = parse(&read_to_string("input.txt")?)?;
    println!("Task 1: {}", task1(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    //     const INPUT: &str = r"0:
    // ###
    // ##.
    // ##.

    // 1:
    // ###
    // ##.
    // .##

    // 2:
    // .##
    // ###
    // ##.

    // 3:
    // ##.
    // ###
    // ##.

    // 4:
    // ###
    // #..
    // ###

    // 5:
    // ###
    // .#.
    // ###

    // 4x4: 0 0 0 0 2 0
    // 12x5: 1 0 1 0 2 2
    // 12x5: 1 0 1 0 3 2";

    // #[test]
    // fn test_task1() -> Result<()> {
    //     let input = parse(INPUT)?;
    //     assert_eq!(task1(&input), 2);
    //     Ok(())
    // }

    #[test]
    fn test_main() -> Result<()> {
        main()
    }
}

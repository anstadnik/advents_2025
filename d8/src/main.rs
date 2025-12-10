use anyhow::Result;
use rayon::slice::ParallelSliceMut;
use std::fs::read_to_string;

use winnow::combinator::{separated, seq};
use winnow::{Parser, ascii::dec_uint};
mod union_find;
pub use union_find::UnionFind;

type Pos = (usize, usize, usize);
fn parse_(input: &mut &str) -> winnow::Result<Vec<Pos>> {
    let three_nums = seq!(dec_uint, _: ',', dec_uint, _: ',', dec_uint);
    separated(1.., three_nums, "\n").parse_next(input)
}

fn parse(input: &str) -> Result<Vec<Pos>> {
    parse_.parse(input).map_err(|e| anyhow::anyhow!("{e}"))
}

fn get_sorted_pairs(input: &[Pos]) -> Vec<(usize, usize)> {
    let n = input.len();
    let mut pairs: Vec<_> = (0..n)
        .flat_map(|i| (i + 1..n).map(move |j| (i, j)))
        .collect();
    pairs.par_sort_by_key(|(i, j)| {
        let (x1, y1, z1) = input[*i];
        let (x2, y2, z2) = input[*j];
        x1.abs_diff(x2).pow(2) + y1.abs_diff(y2).pow(2) + z1.abs_diff(z2).pow(2)
    });
    pairs
}

fn task1(input: &[Pos], pairs: &[(usize, usize)], n: usize) -> usize {
    let mut pairs_it = pairs.into_iter().copied();
    let mut uf = UnionFind::new(input.len());
    for _ in 0..n {
        let (i, j) = pairs_it.next().unwrap();
        uf.union(i, j);
    }
    let mut groups: Vec<_> = uf.groups().into_iter().map(|group| group.len()).collect();
    groups.sort_unstable();
    groups.into_iter().rev().take(3).product()
}

fn task2(input: &[Pos], pairs: &[(usize, usize)]) -> Result<usize> {
    let mut pairs_it = pairs.into_iter().copied();
    let mut uf = UnionFind::new(input.len());
    loop {
        let (i, j) = pairs_it.next().unwrap();
        uf.union(i, j);
        if uf.one_group() {
            return Ok(input[i].0 * input[j].0);
        }
    }
}

fn main() -> Result<()> {
    let input = parse(&read_to_string("input.txt")?)?;
    let pairs = get_sorted_pairs(&input);
    println!("Task 1: {}", task1(&input, &pairs, 1000));
    println!("Task 2: {}", task2(&input, &pairs)?);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_task1() -> Result<()> {
        let input = parse(INPUT)?;
        assert_eq!(task1(&input, &get_sorted_pairs(&input), 10), 40);
        Ok(())
    }

    #[test]
    fn test_task2() -> Result<()> {
        let input = parse(INPUT)?;
        assert_eq!(task2(&input, &get_sorted_pairs(&input))?, 25272);
        Ok(())
    }

    #[test]
    fn test_main() -> Result<()> {
        main()
    }
}

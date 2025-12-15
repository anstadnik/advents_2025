use anyhow::Result;
use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;

use winnow::{
    Parser,
    ascii::alpha1,
    combinator::{separated, separated_pair},
};

fn parse_<'a>(input: &mut &'a str) -> winnow::Result<Vec<(&'a str, Vec<&'a str>)>> {
    let parse_device = separated_pair(alpha1, ": ", separated(1.., alpha1, " ").map(|v: Vec<_>| v));
    separated(1.., parse_device, "\n")
        .map(|v: Vec<_>| v)
        .parse_next(input)
}

type Devices<'a> = HashMap<&'a str, Vec<&'a str>>;
fn parse(input: &str) -> Result<Devices<'_>> {
    parse_
        .parse(input)
        .map(|v| v.into_iter().collect())
        .map_err(|e| anyhow::anyhow!("{e}"))
}

fn task1(devices: &Devices) -> i32 {
    let mut queue = VecDeque::new();

    queue.push_back("you");
    let mut counter = 0;
    while let Some(device) = queue.pop_front() {
        if device == "out" {
            counter += 1;
        }
        if let Some(neighbors) = devices.get(device) {
            for neighbor in neighbors {
                queue.push_back(neighbor);
            }
        }
    }

    counter
}

// fft + dac, fft, dac, none
type Rez = (usize, usize, usize, usize);
fn get_count_for_device<'a>(
    input: &'a str,
    devices: &'a Devices,
    mem: &mut HashMap<&'a str, Rez>,
) -> Rez {
    if let Some(ret) = mem.get(input) {
        return *ret;
    }
    let Some(neighbors) = devices.get(input) else {
        return (0, 0, 0, 0);
    };
    println!("{}: {:?}", input, neighbors);
    let (fft_dac, fft, dac, none) =
        neighbors
            .iter()
            .fold((0, 0, 0, 0), |(fft_dac, fft, dac, none), neighbor| {
                let (fd, f, d, n) = get_count_for_device(neighbor, devices, mem);
                (fft_dac + fd, fft + f, dac + d, none + n)
            });
    let (fft_dac, fft, dac, none) = match input {
        "fft" => (fft_dac + dac, fft + none, 0, 0),
        "dac" => (fft_dac + fft, 0, dac + none, 0),
        _ => (fft_dac, fft, dac, none),
    };
    mem.insert(input, (fft_dac, fft, dac, none));

    (fft_dac, fft, dac, none)
}

fn task2(devices: &Devices) -> usize {
    let mut mem = HashMap::new();
    mem.insert("out", (0, 0, 0, 1));
    get_count_for_device("svr", devices, &mut mem).0
}

fn main() -> Result<()> {
    let s = read_to_string("input.txt")?;
    let input = parse(&s)?;
    println!("Task 1: {}", task1(&input));
    println!("Task 2: {}", task2(&input));
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task1() -> Result<()> {
        let input = r"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
        let input = parse(input)?;
        assert_eq!(task1(&input), 5);
        Ok(())
    }

    #[test]
    fn test_task2() -> Result<()> {
        let input = r"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
        let input = parse(input)?;
        assert_eq!(task2(&input), 2);
        Ok(())
    }

    #[test]
    fn test_main() -> Result<()> {
        main()
    }
}

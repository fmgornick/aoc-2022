#[macro_use]
extern crate scan_fmt;
use itertools::Itertools;
use std::collections::VecDeque;
use std::str;

pub struct Movement {
    from: usize,
    to: usize,
    num: usize,
}

pub fn parse_input(input: &str) -> (Vec<VecDeque<char>>, impl Iterator<Item = Movement> + '_) {
    let (crates, movements) = input.split("\n\n").into_iter().collect_tuple().unwrap();

    let crates: Vec<VecDeque<char>> = crates
        .lines()
        // map all lines of our input, producing 1 new vector
        .flat_map(|l| {
            l.chars()
                .skip(1) // first line is bracket so skip
                .step_by(4) // crate val every 4 chars
                .enumerate() // keep track if indicies
                .filter(|t| t.1.is_alphabetic()) // don't include nonexistent crate
        })
        .into_grouping_map() // Vec of tuples => hashmap: (i, c) => (k, v)
        .collect::<VecDeque<char>>() // collect as hashmap with Char pointers as values
        .into_iter() // turns hashmap back to vector of tuples
        .sorted_by_key(|t| t.0) // make sure indicies in ascending order
        .map(|t| t.1) // don't need index key values anymore
        .collect();

    let movements = movements
        .lines()
        .filter_map(|l| scan_fmt!(l, "move {d} from {d} to {d}", usize, usize, usize).ok())
        .map(|(n, f, t)| Movement {
            from: f - 1,
            to: t - 1,
            num: n,
        });

    (crates, movements)
}

pub fn part_one(input: &str) -> Option<String> {
    let (mut crates, movements) = parse_input(input);

    for mv in movements {
        for _ in 0..mv.num {
            let c = crates[mv.from].pop_front().unwrap();
            crates[mv.to].push_front(c);
        }
    }

    Some(
        crates
            .into_iter()
            .filter_map(|mut stack| stack.pop_front())
            .collect(),
    )
}

pub fn part_two(input: &str) -> Option<String> {
    let (mut crates, movements) = parse_input(input);

    for mv in movements {
        let stack = crates[mv.from].drain(0..mv.num).collect::<VecDeque<char>>();
        for s in stack.into_iter().rev() {
            crates[mv.to].push_front(s);
        }
    }

    Some(
        crates
            .into_iter()
            .filter_map(|mut stack| stack.pop_front())
            .collect(),
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Some(String::from("CMZ")));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Some(String::from("MCD")));
    }
}

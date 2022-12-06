use itertools::Itertools;
use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    for (i, (a, b, c, d)) in input.chars().tuple_windows().enumerate() {
        if a != b && a != c && a != d && b != c && b != d && c != d {
            return Some((i + 4) as u32);
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    for (i, w) in input.as_bytes().windows(14).enumerate() {
        if HashSet::<&u8>::from_iter(w).len() == 14 {
            return Some((i + 14) as u32);
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        let expected = vec![7, 5, 6, 10, 11];
        for (i, l) in input.lines().into_iter().enumerate() {
            assert_eq!(part_one(l), Some(expected[i]));
        }
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        let expected = vec![19, 23, 23, 29, 26];
        for (i, l) in input.lines().into_iter().enumerate() {
            assert_eq!(part_two(l), Some(expected[i]));
        }
    }
}

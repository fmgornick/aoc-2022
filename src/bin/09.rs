use itertools::Itertools;
use std::collections::HashSet;

pub fn parse(input: &str) -> Vec<((i32, i32), i32)> {
    input
        .lines()
        .map(|l| {
            let (dir, count) = l.split(' ').collect_tuple().unwrap();
            match (dir, count.parse::<i32>().unwrap()) {
                ("R", c) => ((1, 0), c),
                ("L", c) => ((-1, 0), c),
                ("U", c) => ((0, 1), c),
                ("D", c) => ((0, -1), c),
                _ => unreachable!(),
            }
        })
        .collect::<Vec<_>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let movements = parse(input);

    let (mut h, mut t) = ((0i32, 0i32), (0i32, 0i32));
    visited.insert((0, 0));
    for (d, c) in movements {
        for _ in 0..c {
            h = (h.0 + d.0, h.1 + d.1);
            if h.0.abs_diff(t.0) > 1 || h.1.abs_diff(t.1) > 1 {
                t = (h.0 - d.0, h.1 - d.1);
                visited.insert(t);
            }
        }
    }

    Some(visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let movements = parse(input);

    let mut rope: Vec<(i32, i32)> = vec![(0, 0); 10];
    visited.insert((0, 0));
    for (d, c) in movements {
        for _ in 0..c {
            rope[0] = (rope[0].0 + d.0, rope[0].1 + d.1);
            for i in 1..10 {
                let (mut dx, mut dy) = (rope[i - 1].0 - rope[i].0, rope[i - 1].1 - rope[i].1);
                if dx.abs() > 1 || dy.abs() > 1 {
                    dx = dx.checked_div(dx.abs()).unwrap_or(0);
                    dy = dy.checked_div(dy.abs()).unwrap_or(0);
                    rope[i] = (rope[i].0 + dx, rope[i].1 + dy);
                } else {
                    break;
                }
            }
            visited.insert(rope[9]);
        }
    }

    Some(visited.len() as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        let ex1 = *input.split("\n\n").collect::<Vec<&str>>().first().unwrap();
        assert_eq!(part_one(ex1), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        let ex2 = *input.split("\n\n").collect::<Vec<&str>>().last().unwrap();
        assert_eq!(part_two(ex2), Some(36));
    }
}

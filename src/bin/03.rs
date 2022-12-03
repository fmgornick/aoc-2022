use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().into_iter().fold(0, |acc, l| {
        let mut mask: u64 = 0;
        let mid: usize = l.len() / 2;
        let v: Vec<u8> = l
            .as_bytes()
            .iter()
            .map(|c| match c {
                97..=122 => (c - 96) as u8,
                65..=90 => (c - 38) as u8,
                _ => unreachable!(),
            })
            .collect();
        for (i, c) in v.iter().enumerate() {
            if i < mid {
                mask |= 1 << c;
            } else if mask & (1 << c) != 0 {
                return acc + (*c as u32);
            }
        }
        acc
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().chunks(3).into_iter().fold(0, |acc, sack| {
        let mut masks: Vec<u64> = vec![0, 0, 0];
        for (i, sack) in sack.enumerate() {
            for c in sack.as_bytes() {
                masks[i] |= 1
                    << match c {
                        97..=122 => (c - 96),
                        65..=90 => (c - 38),
                        _ => unreachable!(),
                    }
            }
        }
        acc + (masks[0] & masks[1] & masks[2]).trailing_zeros()
    }))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Some(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(70));
    }
}

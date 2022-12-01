use std::cmp::max;

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .into_iter()
            .fold((0, 0), |(c, m), line| {
                if line.is_empty() {
                    (0, max(c, m))
                } else {
                    (c + line.parse::<u32>().unwrap(), m)
                }
            })
            .1,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut vec = input
        .lines()
        .into_iter()
        .fold((0, Vec::new()), |(c, mut m), line| {
            if line.is_empty() {
                m.push(c);
                (0, m)
            } else {
                (c + line.parse::<u32>().unwrap(), m)
            }
        })
        .1;
    vec.sort_by(|a, b| b.cmp(a));
    Some(vec[0] + vec[1] + vec[2])
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(45000));
    }
}

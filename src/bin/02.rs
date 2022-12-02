pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let b = line.as_bytes();
                (
                    (b.first().unwrap() - 64) as i32,
                    (b.last().unwrap() - 87) as i32,
                )
            })
            .fold(0, |acc, (c1, c2)| match (c1 - c2).rem_euclid(3) {
                0 => acc + c2 + 3,
                1 => acc + c2,
                2 => acc + c2 + 6,
                _ => unreachable!(),
            }) as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|line| {
                let b = line.as_bytes();
                (
                    (b.first().unwrap() - 64) as i32,
                    (b.last().unwrap() - 87) as i32,
                )
            })
            .fold(0, |acc, (c1, c2)| {
                acc + (c2 - 1) * 3 + (c1 + c2).rem_euclid(3) + 1
            }) as u32,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}

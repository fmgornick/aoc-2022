pub fn part_one(input: &str) -> Option<u32> {
    Some(input.lines().into_iter().fold(0, |acc, l| {
        let v: Vec<u32> = l
            .split(&['-', ','][..])
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        if (v[0] <= v[2] && v[1] >= v[3]) || (v[0] >= v[2] && v[1] <= v[3]) {
            return acc + 1;
        }
        acc
    }))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().into_iter().fold(0, |acc, l| {
        let v: Vec<u32> = l
            .split(&['-', ','][..])
            .map(|s| s.parse::<u32>().unwrap())
            .collect();
        if (v[0] >= v[2] && v[0] <= v[3]) || (v[2] >= v[0] && v[2] <= v[1]) {
            return acc + 1;
        }
        acc
    }))
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Some(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Some(4));
    }
}

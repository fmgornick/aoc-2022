pub fn parse_input<'a>(input: &mut impl Iterator<Item = &'a str>) -> Vec<u64> {
    let (mut sum, mut subdirs) = (0, vec![]);
    loop {
        match input
            .next()
            .map(|s| s.split_whitespace().collect::<Vec<&str>>())
            .as_deref()
        {
            Some(["$", "cd", ".."]) | None => break,
            Some(["$", "cd", s]) if *s != "/" => {
                subdirs.extend(parse_input(input));
                sum += subdirs.last().unwrap();
            }
            Some([v, _]) if *v != "$" && *v != "dir" => {
                sum += v.parse::<u64>().unwrap();
            }
            _ => (),
        }
    }
    subdirs.push(sum);
    subdirs
}

pub fn part_one(input: &str) -> Option<u64> {
    Some(
        parse_input(&mut input.lines())
            .into_iter()
            .filter(|&x| x <= 100000)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u64> {
    let v = parse_input(&mut input.lines());
    let size = *v.last().unwrap();
    v.into_iter().filter(|&x| size - x <= 40000000).min()
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), Some(24933642));
    }
}

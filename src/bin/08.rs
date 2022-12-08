use std::collections::HashSet;

pub fn parse(input: &str) -> (Vec<Vec<u8>>, usize) {
    let forest: Vec<Vec<u8>> = input
        .lines()
        .into_iter()
        .map(|l| l.chars().map(|c| (c as u8) - 48).collect())
        .collect::<Vec<_>>();
    (forest, input.find('\n').unwrap())
}

pub fn part_one(input: &str) -> Option<u32> {
    let (forest, size) = parse(input);
    let mut visible: HashSet<(u8, u8)> = HashSet::new();

    for i in 0..size {
        let (mut left, mut top, mut right, mut bottom) = (0, 0, 0, 0);
        for j in 0..size {
            if forest[i][j] > left || j == 0 {
                left = forest[i][j];
                visible.insert((i as u8, j as u8));
            }
            if forest[j][i] > top || j == 0 {
                top = forest[j][i];
                visible.insert((j as u8, i as u8));
            }
            if forest[i][size - j - 1] > right || j == 0 {
                right = forest[i][size - j - 1];
                visible.insert((i as u8, (size - j - 1) as u8));
            }
            if forest[size - j - 1][i] > bottom || j == 0 {
                bottom = forest[size - j - 1][i];
                visible.insert(((size - j - 1) as u8, i as u8));
            }
        }
    }

    Some(visible.len() as u32)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (forest, size) = parse(input);
    let mut max: u64 = 0;

    for i in 1..size {
        for j in 1..size {
            let (mut n, mut s, mut e, mut w) = (0, 0, 0, 0);
            for k in (0..i).rev() {
                if forest[i][j] <= forest[k][j] || k == 0 {
                    n = (i - k) as u64;
                    break;
                }
            }
            for k in (i + 1)..size {
                if forest[i][j] <= forest[k][j] || k == size - 1 {
                    s = (k - i) as u64;
                    break;
                }
            }
            for k in (j + 1)..size {
                if forest[i][j] <= forest[i][k] || k == size - 1 {
                    e = (k - j) as u64;
                    break;
                }
            }
            for k in (0..j).rev() {
                if forest[i][j] <= forest[i][k] || k == 0 {
                    w = (j - k) as u64;
                    break;
                }
            }
            max = std::cmp::max(max, n * s * e * w);
        }
    }

    Some(max)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Some(8));
    }
}

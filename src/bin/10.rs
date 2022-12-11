use itertools::Itertools;

pub fn part_one(input: &str) -> Option<i32> {
    Some(
        input
            .lines()
            .map(|l| match &l[0..4] {
                "addx" => l[5..].parse::<i32>().unwrap(),
                "noop" => 0,
                _ => unreachable!(),
            })
            .fold((1, 0, 0), |(x, frame, total), val| {
                let (newx, newframe) = match val {
                    0 => (x, frame + 1),
                    v => (x + v, frame + 2),
                };
                let mut newtotal = total;
                for i in (20..=220).step_by(40) {
                    if frame < i && newframe >= i {
                        newtotal += i * x;
                    }
                }
                (newx, newframe, newtotal)
            })
            .2,
    )
}

pub fn part_two(input: &str) -> Option<String> {
    Some(
        input
            .lines()
            .map(|l| match &l[0..4] {
                "addx" => l[5..].parse::<i32>().unwrap(),
                "noop" => 0,
                _ => unreachable!(),
            })
            .fold((1, 0, String::from("")), |(x, frame, out), val| {
                let (newx, newframe) = match val {
                    0 => (x, frame + 1),
                    v => (x + v, frame + 2),
                };
                let mut newout = out;
                for i in frame..newframe {
                    if ((x - 1)..=(x + 1)).contains(&((i % 40) as i32)) {
                        newout.push('#');
                    } else {
                        newout.push('.');
                    }
                    if (40..=240).step_by(40).contains(&(i + 1)) {
                        newout.push('\n');
                    }
                }
                (newx, newframe, newout)
            })
            .2,
    )
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), Some(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(
            part_two(&input),
            Some(String::from(
                "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
            ))
        );
    }
}

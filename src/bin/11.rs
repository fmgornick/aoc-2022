use itertools::Itertools;

pub struct Monkey {
    items: Vec<usize>,
    op: Operation,
    modulus: usize,
    true_monkey: usize,
    false_monkey: usize,
}

enum Operation {
    Add(usize),
    Multiply(usize),
    MultiplySelf,
}

impl Monkey {
    fn parse(input: &str) -> Monkey {
        let monkey_data = input.lines().map(String::from).collect::<Vec<_>>();
        let items = monkey_data[1].trim()[16..]
            .split(", ")
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let op = match &monkey_data[2].trim()[21..] {
            "* old" => Operation::MultiplySelf,
            s => match s.split(' ').collect_tuple().unwrap() {
                ("+", val) => Operation::Add(val.parse::<usize>().unwrap()),
                ("*", val) => Operation::Multiply(val.parse::<usize>().unwrap()),
                _ => unreachable!(),
            },
        };
        let modulus = monkey_data[3].trim()[19..].parse::<usize>().unwrap();
        let true_monkey = monkey_data[4].trim()[25..].parse::<usize>().unwrap();
        let false_monkey = monkey_data[5].trim()[26..].parse::<usize>().unwrap();

        Monkey {
            items,
            op,
            modulus,
            true_monkey,
            false_monkey,
        }
    }
}

impl Operation {
    fn execute(&self, x: usize) -> usize {
        match self {
            Operation::Add(y) => x + y,
            Operation::Multiply(y) => x * y,
            Operation::MultiplySelf => x * x,
        }
    }
}

pub fn calculate_worry_level(
    mut monkeys: Vec<Monkey>,
    rounds: usize,
    resize_item: impl Fn(usize) -> usize,
) -> Option<u64> {
    let mut num_inspections = vec![0; monkeys.len()];

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let monkey = &mut monkeys[i];
            let items = monkey
                .items
                .drain(..)
                .map(|item| resize_item(monkey.op.execute(item)))
                .collect::<Vec<_>>();

            num_inspections[i] += items.len();
            let (modulus, true_monkey, false_monkey) =
                (monkey.modulus, monkey.true_monkey, monkey.false_monkey);

            for item in items {
                match item % modulus {
                    0 => monkeys[true_monkey].items.push(item),
                    _ => monkeys[false_monkey].items.push(item),
                }
            }
        }
    }

    num_inspections.sort_by(|a, b| b.cmp(a));
    Some((num_inspections[0] as u64) * (num_inspections[1] as u64))
}

pub fn part_one(input: &str) -> Option<u64> {
    let monkeys = input.split("\n\n").map(Monkey::parse).collect::<Vec<_>>();
    calculate_worry_level(monkeys, 20, |x| x / 3)
}

pub fn part_two(input: &str) -> Option<u64> {
    let monkeys = input.split("\n\n").map(Monkey::parse).collect::<Vec<_>>();
    let modulus = monkeys.iter().fold(1, |acc, m| acc * m.modulus);
    calculate_worry_level(monkeys, 10000, |x| x % modulus)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}

use util::*;

#[derive(Clone, Debug)]
enum Operation {
    Add(usize),
    Mul(usize),
    Square,
}

#[derive(Clone, Debug)]
struct Monkey {
    items: VecDeque<usize>,
    operation: Operation,
    test_div: usize,
    on_true: usize,
    on_false: usize,
}

fn parse(input: &str) -> Result<Vec<Monkey>, Error> {
    let mut monkeys = Vec::new();
    let mut it = input
        .trim()
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty());

    while let (Some(_), Some(items), Some(op), Some(test), Some(on_true), Some(on_false)) = (
        it.next(),
        it.next(),
        it.next(),
        it.next(),
        it.next(),
        it.next(),
    ) {
        let items = items
            .trim_start_matches("Starting items: ")
            .split(", ")
            .map(usize::from_str)
            .collect::<Result<_, _>>()?;

        let operation = if op == "Operation: new = old * old" {
            Operation::Square
        } else {
            let op = op.trim_start_matches("Operation: new = old ");
            let add = op.trim_start_matches("+ ");
            let mul = op.trim_start_matches("* ");

            if add.len() < op.len() {
                Operation::Add(add.parse()?)
            } else if mul.len() < op.len() {
                Operation::Mul(mul.parse()?)
            } else {
                bail!("Invalid operation: {op}")
            }
        };

        let test_div = test.trim_start_matches("Test: divisible by ").parse()?;

        let on_true = on_true
            .trim_start_matches("If true: throw to monkey ")
            .parse()?;

        let on_false = on_false
            .trim_start_matches("If false: throw to monkey ")
            .parse()?;

        monkeys.push(Monkey {
            items,
            operation,
            test_div,
            on_true,
            on_false,
        });
    }

    Ok(monkeys)
}

fn puzzle(mut input: Vec<Monkey>, rounds: usize, worry_fac: usize) -> usize {
    let mut totals = vec![0usize; input.len()];
    let f = input.iter().fold(1usize, |acc, m| acc * m.test_div);

    for _ in 0..rounds {
        for i in 0..input.len() {
            while let Some(worry) = input[i].items.pop_front() {
                totals[i] += 1;

                let worry = (match input[i].operation {
                    Operation::Add(n) => worry + n,
                    Operation::Mul(n) => worry * n,
                    Operation::Square => worry * worry,
                } / worry_fac)
                    % f;

                let target = if worry % input[i].test_div == 0 {
                    input[i].on_true
                } else {
                    input[i].on_false
                };

                input[target].items.push_back(worry);
            }
        }
    }

    totals.sort();
    totals.pop().unwrap() * totals.pop().unwrap()
}

fn part_1(input: Vec<Monkey>) -> usize {
    puzzle(input, 20, 3)
}

fn part_2(input: Vec<Monkey>) -> usize {
    puzzle(input, 10000, 1)
}

fn main() -> Result<(), Error> {
    let input = parse(&read_stdin()?)?;

    println!("Part 1: {}", part_1(input.clone()));
    println!("Part 2: {}", part_2(input));

    Ok(())
}

#[cfg(test)]
mod test {
    static INPUT: &str = r#"
        Monkey 0:
            Starting items: 79, 98
            Operation: new = old * 19
            Test: divisible by 23
                If true: throw to monkey 2
                If false: throw to monkey 3

        Monkey 1:
            Starting items: 54, 65, 75, 74
            Operation: new = old + 6
            Test: divisible by 19
                If true: throw to monkey 2
                If false: throw to monkey 0

        Monkey 2:
            Starting items: 79, 60, 97
            Operation: new = old * old
            Test: divisible by 13
                If true: throw to monkey 1
                If false: throw to monkey 3

        Monkey 3:
            Starting items: 74
            Operation: new = old + 3
            Test: divisible by 17
                If true: throw to monkey 0
                If false: throw to monkey 1
    "#;

    #[test]
    fn part_1() -> Result<(), super::Error> {
        let input = super::parse(INPUT)?;

        assert_eq!(super::part_1(input), 10605);

        Ok(())
    }

    #[test]
    fn part_2() -> Result<(), super::Error> {
        let input = super::parse(INPUT)?;

        assert_eq!(super::part_2(input), 2713310158);

        Ok(())
    }
}

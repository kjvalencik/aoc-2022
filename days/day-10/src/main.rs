use util::*;

#[derive(Debug)]
enum Op {
    Noop,
    AddrX(i64),
}

fn parse(input: &str) -> Result<Vec<Op>, Error> {
    input
        .trim()
        .lines()
        .map(|l| {
            Ok(match l.trim() {
                "noop" => Op::Noop,
                op => Op::AddrX(op.trim_start_matches("addx ").parse()?),
            })
        })
        .collect()
}

fn part_1(ops: &[Op]) -> i64 {
    let mut sum = 0;
    let mut x = 1;

    for (i, op) in ops
        .iter()
        .flat_map(|op| match op {
            Op::Noop => [Some(Op::Noop), None],
            Op::AddrX(n) => [Some(Op::Noop), Some(Op::AddrX(*n))],
        })
        .flatten()
        .enumerate()
        .map(|(i, op)| (i + 1, op))
    {
        match i {
            20 | 60 | 100 | 140 | 180 | 220 => sum += (i as i64) * x,
            _ => {}
        }

        if let Op::AddrX(n) = op {
            x += n;
        }
    }

    sum
}

fn part_2(ops: &[Op]) -> String {
    let mut buf = String::new();
    let mut x = 1;

    for (i, op) in ops
        .iter()
        .flat_map(|op| match op {
            Op::Noop => [Some(Op::Noop), None],
            Op::AddrX(n) => [Some(Op::Noop), Some(Op::AddrX(*n))],
        })
        .flatten()
        .enumerate()
    {
        if i % 40 == 0 && i != 0 {
            buf += "\n";
        }

        let y = (i % 40) as i64;

        if x >= y - 1 && x <= y + 1 {
            buf += "#";
        } else {
            buf += ".";
        }

        if let Op::AddrX(n) = op {
            x += n;
        }
    }

    buf
}

fn main() -> Result<(), Error> {
    let input = parse(&read_stdin()?)?;

    println!("Part 1: {}", part_1(&input));
    println!("Part 2:\n{}", part_2(&input));

    Ok(())
}

#[cfg(test)]
mod test {
    static INPUT: &str = include_str!("../test/sample.txt");
    static OUTPUT: &str = include_str!("../test/output.txt");

    #[test]
    fn part_1() -> Result<(), super::Error> {
        let input = super::parse(INPUT)?;

        assert_eq!(super::part_1(&input), 13140);

        Ok(())
    }

    #[test]
    fn part_2() -> Result<(), super::Error> {
        let input = super::parse(INPUT)?;

        assert_eq!(super::part_2(&input), OUTPUT.trim());

        Ok(())
    }
}

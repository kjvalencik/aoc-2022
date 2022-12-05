use util::*;

type Stack = Vec<u8>;

#[derive(Clone, Debug)]
struct Puzzle {
    stacks: Vec<Stack>,
    moves: Vec<Move>,
}

#[derive(Clone, Debug)]
struct Move {
    qty: usize,
    from: usize,
    to: usize,
}

fn parse_stacks(lines: &str) -> Vec<Stack> {
    let mut stacks = Vec::new();

    for line in lines.trim_matches('\n').lines() {
        for (x, c) in line.as_bytes().iter().enumerate() {
            stacks.resize_with(stacks.len().max(x + 1), Vec::new);
            stacks[x].push(*c);
        }
    }

    stacks
        .into_iter()
        .filter_map(|stack| {
            if stack.last().unwrap_or(&b' ') == &b' ' {
                return None;
            }

            let stack = stack
                .into_iter()
                .rev()
                .skip(1)
                .take_while(|&c| c != b' ')
                .collect();

            Some(stack)
        })
        .collect()
}

fn parse_moves(moves: &str) -> Result<Vec<Move>, Error> {
    moves
        .trim()
        .lines()
        .map(|line| {
            let mut parts = line.trim().split(' ').skip(1);
            if let (Some(qty), _, Some(from), _, Some(to)) = (
                parts.next(),
                parts.next(),
                parts.next(),
                parts.next(),
                parts.next(),
            ) {
                return Ok(Move {
                    qty: qty.parse()?,
                    from: from.parse()?,
                    to: to.parse()?,
                });
            }

            bail!("Invalid move")
        })
        .collect()
}

fn parse(input: &str) -> Result<Puzzle, Error> {
    let (stacks, moves) = input
        .trim_matches('\n')
        .split_once("\n\n")
        .context("Missing movies")?;

    Ok(Puzzle {
        stacks: parse_stacks(stacks),
        moves: parse_moves(moves)?,
    })
}

fn part_1(puzzle: Puzzle) -> String {
    let Puzzle { mut stacks, moves } = puzzle;

    for mv in moves {
        if stacks.len() >= mv.from && stacks.len() >= mv.to {
            for _ in 0..(mv.qty) {
                if let Some(c) = stacks[mv.from - 1].pop() {
                    stacks[mv.to - 1].push(c);
                }
            }
        }
    }

    stacks
        .into_iter()
        .filter_map(|mut stack| stack.pop().map(char::from))
        .collect()
}

fn part_2(puzzle: Puzzle) -> String {
    let Puzzle { mut stacks, moves } = puzzle;

    for mv in moves {
        if stacks.len() >= mv.from && stacks.len() >= mv.to {
            let qty = if stacks[mv.from - 1].len() >= mv.qty {
                mv.qty
            } else {
                stacks[mv.from - 1].len()
            };

            let pos = stacks[mv.from - 1].len() - qty;
            let blocks = stacks[mv.from - 1].split_off(pos);

            stacks[mv.to - 1].extend(blocks);
        }
    }

    stacks
        .into_iter()
        .filter_map(|mut stack| stack.pop().map(char::from))
        .collect()
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
            [D]
        [N] [C]
        [Z] [M] [P]
         1   2   3

        move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2
    "#;

    #[test]
    fn part_1() -> Result<(), super::Error> {
        let input = super::parse(INPUT)?;

        assert_eq!(super::part_1(input), "CMZ");

        Ok(())
    }

    #[test]
    fn part_2() -> Result<(), super::Error> {
        let input = super::parse(INPUT)?;

        assert_eq!(super::part_2(input), "MCD");

        Ok(())
    }
}

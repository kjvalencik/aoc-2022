use util::*;

#[derive(Debug, Eq, PartialEq)]
enum Jet {
    Left,
    Right,
}

fn next_shape(i: usize) -> Vec<(usize, usize)> {
    match i % 5 {
        0 => vec![(0, 0), (1, 0), (2, 0), (3, 0)],
        1 => vec![(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)],
        2 => vec![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
        3 => vec![(0, 0), (0, 1), (0, 2), (0, 3)],
        4 => vec![(0, 0), (1, 0), (0, 1), (1, 1)],
        _ => unreachable!(),
    }
}

fn simulate(
    jets: &[Jet],
    board: &mut HashSet<(usize, usize)>,
    mut shape: Vec<(usize, usize)>,
    j: &mut usize,
    top: &mut usize,
) {
    for (x, y) in shape.iter_mut() {
        *x += 3;
        *y = *y + *top + 4;
    }

    loop {
        let next: Vec<_> = match jets[*j % jets.len()] {
            Jet::Left => shape.iter().map(|&(x, y)| (x - 1, y)).collect(),
            Jet::Right => shape.iter().map(|&(x, y)| (x + 1, y)).collect(),
        };

        *j += 1;

        if next
            .iter()
            .all(|p| p.0 > 0 && p.0 < 8 && !board.contains(p))
        {
            shape = next;
        }

        let next = shape.iter().map(|&(x, y)| (x, y - 1)).collect::<Vec<_>>();

        if next.iter().any(|p| board.contains(p)) {
            break;
        }

        shape = next;
    }

    for p in &shape {
        if p.1 > *top {
            *top = p.1;
        }
    }

    board.extend(shape);
}

fn part_1(jets: &[Jet]) -> usize {
    let mut top = 0;
    let mut board = HashSet::new();
    let mut j = 0;

    for x in 1..=7 {
        board.insert((x, 0));
    }

    for i in 0..2022 {
        simulate(jets, &mut board, next_shape(i), &mut j, &mut top);
    }

    top
}

fn part_2(jets: &[Jet]) -> usize {
    let mut top = 0;
    let mut board = HashSet::new();
    let mut j = 0;
    let mut states = Vec::new();
    let r = 10000;

    for x in 1..=7 {
        board.insert((x, 0));
    }

    for i in 0..r {
        let prev_top = top;

        simulate(jets, &mut board, next_shape(i), &mut j, &mut top);
        states.push((top - prev_top, i % 5, j % jets.len()));
    }

    let mut repeat = 0;

    for i in 0usize..(states.len() / 2) {
        let start = states.len() - i - 1;
        let left = &states[start..states.len()];
        let right = &states[(start - i - 1)..start];

        if left == right {
            repeat = i;
        }
    }

    let section = &states[(states.len() - repeat - 1)..states.len()];
    let sum = section.iter().map(|(v, _, _)| *v).sum::<usize>();

    let y = (1_000_000_000_000 - r) / section.len();
    let remainder = &section[0..((1_000_000_000_000 - r) % section.len())]
        .iter()
        .map(|(v, _, _)| *v)
        .sum::<usize>();

    top + remainder + sum * y
}

fn parse(input: &str) -> Result<Vec<Jet>, Error> {
    input
        .trim()
        .chars()
        .map(|c| {
            Ok(match c {
                '<' => Jet::Left,
                '>' => Jet::Right,
                _ => bail!("Invalid char: {c}"),
            })
        })
        .collect()
}

fn main() -> Result<(), Error> {
    let input = parse(&read_stdin()?)?;

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));

    Ok(())
}

#[cfg(test)]
mod test {
    static INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn part_1() -> Result<(), super::Error> {
        let input = super::parse(INPUT)?;

        assert_eq!(super::part_1(&input), 3068);

        Ok(())
    }

    #[test]
    fn part_2() -> Result<(), super::Error> {
        let input = super::parse(INPUT)?;

        assert_eq!(super::part_2(&input), 1514285714288);

        Ok(())
    }
}

use util::*;

struct Puzzle {
    start: (usize, usize),
    end: (usize, usize),
    map: Vec<Vec<u8>>,
}

fn parse(input: &str) -> Result<Puzzle, Error> {
    let mut map = input
        .trim()
        .lines()
        .map(|l| l.trim().as_bytes().to_owned())
        .collect::<Vec<_>>();

    let find = |m| {
        map.iter()
            .enumerate()
            .flat_map(|(y, l)| l.iter().enumerate().map(move |(x, c)| (c, (x, y))))
            .find(move |(&c, _)| c == m)
            .map(|(_, pos)| pos)
    };

    let start = find(b'S').context("Missing start")?;
    let end = find(b'E').context("Missing end")?;

    map[start.1][start.0] = b'a';
    map[end.1][end.0] = b'z';

    Ok(Puzzle { start, end, map })
}

fn part_1(input: &Puzzle) -> Result<usize, Error> {
    #[derive(Copy, Clone, Eq, PartialEq)]
    struct State {
        cost: usize,
        pos: usize,
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            other
                .cost
                .cmp(&self.cost)
                .then_with(|| self.pos.cmp(&other.pos))
        }
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let width = input.map[0].len();
    let adj_list = input
        .map
        .iter()
        .enumerate()
        .flat_map(|(y, line)| line.iter().enumerate().map(move |(x, &c)| (x, y, c)))
        .map(|(x, y, c)| {
            // `x, y` are base `1` to avoid underflow
            let get_edge = |x: usize, y: usize| {
                // Off the map up or left
                if x == 0 || y == 0 {
                    return None;
                }

                // Adjust `x` and `y` backt to `0` base
                let x = x - 1;
                let y = y - 1;

                // If `None`, off the map right or down
                let t = *input.map.get(y)?.get(x)?;

                // Need to climb more than `1`
                if t > c && t - c > 1 {
                    return None;
                }

                Some(x + width * y)
            };

            [
                // Left
                get_edge(x, y + 1),
                // Right
                get_edge(x + 2, y + 1),
                // Up
                get_edge(x + 1, y),
                // Down
                get_edge(x + 1, y + 2),
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut dist = (0..adj_list.len()).map(|_| usize::MAX).collect::<Vec<_>>();
    let mut heap = BinaryHeap::new();
    let start = input.start.0 + input.start.1 * width;
    let end = input.end.0 + input.end.1 * width;

    dist[start] = 0;
    heap.push(State {
        cost: 0,
        pos: start,
    });

    while let Some(State { cost, pos }) = heap.pop() {
        if pos == end {
            return Ok(cost);
        }

        if cost > dist[pos] {
            continue;
        }

        for &pos in &adj_list[pos] {
            let next = State {
                cost: cost + 1,
                pos,
            };

            if next.cost < dist[next.pos] {
                heap.push(next);
                dist[next.pos] = next.cost;
            }
        }
    }

    bail!("Could not find a solution")
}

fn part_2(mut input: Puzzle) -> Result<usize, Error> {
    let mut min = usize::MAX;
    let starts = input
        .map
        .iter()
        .enumerate()
        .flat_map(|(y, line)| line.iter().enumerate().map(move |(x, c)| (x, y, c)))
        .filter(|(_, _, &c)| c == b'a')
        .map(|(x, y, _)| (x, y));

    for start in starts {
        input.start = start;

        if let Ok(n) = part_1(&input) {
            min = min.min(n);
        }
    }

    if min == usize::MAX {
        bail!("No solution found");
    }

    Ok(min)
}

fn main() -> Result<(), Error> {
    let input = parse(&read_stdin()?)?;

    println!("Part 1: {}", part_1(&input)?);
    println!("Part 2: {}", part_2(input)?);

    Ok(())
}

#[cfg(test)]
mod test {
    static INPUT: &str = r#"
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi
    "#;

    #[test]
    fn part_1() -> Result<(), super::Error> {
        let input = super::parse(INPUT)?;

        assert_eq!(super::part_1(&input)?, 31);

        Ok(())
    }

    #[test]
    fn part_2() -> Result<(), super::Error> {
        let input = super::parse(INPUT)?;

        assert_eq!(super::part_2(input)?, 29);

        Ok(())
    }
}

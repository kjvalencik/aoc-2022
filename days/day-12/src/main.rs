use util::*;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    pos: (usize, usize),
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

struct Puzzle {
    start: (usize, usize),
    end: (usize, usize),
    map: Vec<Vec<u8>>,
}

impl Puzzle {
    fn parse(input: &str) -> Result<Self, Error> {
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

        Ok(Self { start, end, map })
    }

    fn path_costs(&self, reverse: bool) -> HashMap<(usize, usize), usize> {
        let can_climb = &if reverse {
            |t, c| c <= t || c - t == 1
        } else {
            |t, c| t <= c || t - c == 1
        };

        let start = if reverse { self.end } else { self.start };

        let edges = self
            .map
            .iter()
            .enumerate()
            .map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .map(move |(x, &c)| {
                        let get_edge = |x: usize, y: usize| {
                            if x == 0 || y == 0 {
                                return None;
                            }

                            let x = x - 1;
                            let y = y - 1;
                            let t = *self.map.get(y)?.get(x)?;

                            if !can_climb(t, c) {
                                return None;
                            }

                            Some((x, y))
                        };

                        [
                            get_edge(x, y + 1),
                            get_edge(x + 2, y + 1),
                            get_edge(x + 1, y),
                            get_edge(x + 1, y + 2),
                        ]
                        .into_iter()
                        .flatten()
                        .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let mut dist = HashMap::new();
        let mut heap = BinaryHeap::new();

        dist.insert(start, 0);
        heap.push(State {
            cost: 0,
            pos: start,
        });

        while let Some(State { cost, pos }) = heap.pop() {
            match dist.get(&pos) {
                Some(&n) if cost > n => continue,
                _ => {}
            }

            for &pos in &edges[pos.1][pos.0] {
                let next = State {
                    cost: cost + 1,
                    pos,
                };

                match dist.entry(next.pos) {
                    Entry::Occupied(mut e) => {
                        if next.cost < *e.get() {
                            heap.push(next);
                            e.insert(next.cost);
                        }
                    }
                    Entry::Vacant(e) => {
                        heap.push(next);
                        e.insert(next.cost);
                    }
                }
            }
        }

        dist
    }

    fn part_1(&self) -> Result<usize, Error> {
        self.path_costs(false)
            .get(&self.end)
            .copied()
            .context("No solution")
    }

    fn part_2(&self) -> Result<usize, Error> {
        let costs = self.path_costs(true);

        self.map
            .iter()
            .enumerate()
            .flat_map(move |(y, line)| {
                line.iter()
                    .enumerate()
                    .filter(|(_, &c)| c == b'a')
                    .map(move |(x, _)| (x, y))
            })
            .flat_map(move |pos| costs.get(&pos).copied())
            .min()
            .context("No solution")
    }
}

fn main() -> Result<(), Error> {
    let puzzle = Puzzle::parse(&read_stdin()?)?;

    println!("Part 1: {}", puzzle.part_1()?);
    println!("Part 2: {}", puzzle.part_2()?);

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
        let puzzle = super::Puzzle::parse(INPUT)?;

        assert_eq!(puzzle.part_1()?, 31);

        Ok(())
    }

    #[test]
    fn part_2() -> Result<(), super::Error> {
        let puzzle = super::Puzzle::parse(INPUT)?;

        assert_eq!(puzzle.part_2()?, 29);

        Ok(())
    }
}

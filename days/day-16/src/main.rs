use util::*;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State<'a> {
    cost: usize,
    position: &'a str,
}

impl Ord for State<'_> {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(other.position))
    }
}

impl PartialOrd for State<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Valve<'a> {
    flow_rate: usize,
    tunnels: HashSet<&'a str>,
    dist: HashMap<&'a str, usize>,
}

#[derive(Debug)]
struct Puzzle<'a> {
    valves: HashMap<&'a str, Valve<'a>>,
}

fn calc_dist<'a>(start: &'a str, valves: &HashMap<&'a str, Valve<'a>>) -> HashMap<&'a str, usize> {
    let mut dist = HashMap::new();
    let mut heap = BinaryHeap::new();

    dist.insert(start, 0);
    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        if dist.get(position).map(|n| cost > *n).unwrap_or(false) {
            continue;
        }

        if let Some(valve) = valves.get(position) {
            for tunnel in &valve.tunnels {
                let next = State {
                    cost: cost + 1,
                    position: tunnel,
                };

                // If so, add it to the frontier and continue
                if dist.get(tunnel).map(|n| next.cost < *n).unwrap_or(true) {
                    heap.push(next);
                    dist.insert(tunnel, next.cost);
                }
            }
        }
    }

    dist.retain(|name, _| valves.get(name).map(|v| v.flow_rate > 0).unwrap_or(false));
    dist
}

fn parse(input: &str) -> Result<Puzzle, Error> {
    let mut valves = input
        .trim()
        .lines()
        .map(|line| {
            let dist = HashMap::with_capacity(0);
            let (l, r) = line.trim().split_once(';').context("Missing tunnels")?;
            let l = l.trim().trim_start_matches("Valve ");
            let name = l.split(' ').next().context("Missing name")?;
            let flow_rate = l.split('=').nth(1).context("Missing rate")?.parse()?;

            let tunnels = r
                .trim()
                .trim_start_matches("tunnels lead to valves ")
                .trim_start_matches("tunnel leads to valve ")
                .split(", ")
                .collect();

            Ok((
                name,
                Valve {
                    flow_rate,
                    tunnels,
                    dist,
                },
            ))
        })
        .collect::<Result<HashMap<_, _>, Error>>()?;

    let distances = valves
        .keys()
        .map(|name| (*name, calc_dist(name, &valves)))
        .collect::<HashMap<_, _>>();

    for (name, d) in distances {
        if let Some(valve) = valves.get_mut(name) {
            valve.dist = d;
        }
    }

    Ok(Puzzle { valves })
}

fn find_part_1<'a>(
    puzzle: &Puzzle<'a>,
    name: &'a str,
    valve: &Valve<'a>,
    mut remaining: usize,
    mut sum: usize,
    max: &mut usize,
    visited: &mut HashSet<&'a str>,
) {
    visited.insert(name);

    if valve.flow_rate > 0 {
        remaining -= 1;
        sum += remaining * valve.flow_rate;

        if sum > *max {
            *max = sum;
        }
    }

    for (tunnel, dist) in &valve.dist {
        if visited.contains(tunnel) || *dist >= remaining {
            continue;
        }

        if let Some(valve) = puzzle.valves.get(tunnel) {
            find_part_1(puzzle, tunnel, valve, remaining - dist, sum, max, visited);
        }
    }

    visited.remove(name);
}

fn part_1(puzzle: &Puzzle) -> usize {
    let start_pos = "AA";
    let mut max = 0;
    let start = if let Some(valve) = puzzle.valves.get(start_pos) {
        valve
    } else {
        return max;
    };

    let mut visited = HashSet::new();

    find_part_1(puzzle, start_pos, start, 30, 0, &mut max, &mut visited);

    max
}

#[allow(clippy::too_many_arguments)]
fn find_part_2<'a>(
    puzzle: &Puzzle<'a>,
    name: &'a str,
    valve: &Valve<'a>,
    mut remaining: usize,
    mut sum: usize,
    max: &mut usize,
    visited: &mut HashSet<&'a str>,
    is_elephant: bool,
) {
    visited.insert(name);

    if valve.flow_rate > 0 {
        remaining -= 1;
        sum += remaining * valve.flow_rate;

        if sum > *max {
            *max = sum;
        }
    }

    if !is_elephant {
        if let Some(valve) = puzzle.valves.get("AA") {
            find_part_2(puzzle, "AA", valve, 26, sum, max, visited, true);
        }
    }

    for (tunnel, dist) in &valve.dist {
        if visited.contains(tunnel) || *dist >= remaining {
            continue;
        }

        if let Some(valve) = puzzle.valves.get(tunnel) {
            find_part_2(
                puzzle,
                tunnel,
                valve,
                remaining - dist,
                sum,
                max,
                visited,
                is_elephant,
            );
        }
    }

    visited.remove(name);
}

fn part_2(puzzle: &Puzzle) -> usize {
    let start_pos = "AA";
    let mut max = 0;
    let start = if let Some(valve) = puzzle.valves.get(start_pos) {
        valve
    } else {
        return max;
    };

    let mut visited = HashSet::new();

    find_part_2(
        puzzle,
        start_pos,
        start,
        26,
        0,
        &mut max,
        &mut visited,
        false,
    );

    max
}

fn main() -> Result<(), Error> {
    let input = read_stdin()?;
    let input = parse(&input)?;

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));

    Ok(())
}

#[cfg(test)]
mod test {
    static INPUT: &str = r#"
        Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        Valve BB has flow rate=13; tunnels lead to valves CC, AA
        Valve CC has flow rate=2; tunnels lead to valves DD, BB
        Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
        Valve EE has flow rate=3; tunnels lead to valves FF, DD
        Valve FF has flow rate=0; tunnels lead to valves EE, GG
        Valve GG has flow rate=0; tunnels lead to valves FF, HH
        Valve HH has flow rate=22; tunnel leads to valve GG
        Valve II has flow rate=0; tunnels lead to valves AA, JJ
        Valve JJ has flow rate=21; tunnel leads to valve II
    "#;

    #[test]
    fn part_1() -> Result<(), super::Error> {
        let input = super::parse(INPUT)?;

        assert_eq!(super::part_1(&input), 1651);

        Ok(())
    }

    #[test]
    fn part_2() -> Result<(), super::Error> {
        let input = super::parse(INPUT)?;

        assert_eq!(super::part_2(&input), 1707);

        Ok(())
    }
}

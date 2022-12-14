use util::*;

#[derive(Clone, Debug)]
enum Item {
    Rock,
    Sand,
    Start,
}

#[derive(Clone, Debug)]
struct Puzzle {
    map: HashMap<(isize, isize), Item>,
}

impl Puzzle {
    fn parse(input: &str) -> Result<Self, Error> {
        let mut map = HashMap::new();

        map.insert((500, 0), Item::Start);

        for line in input.trim().lines() {
            let mut prev: Option<(isize, isize)> = None;

            for segment in line.trim().split(" -> ") {
                let (x, y) = segment.split_once(',').context("Missing y")?;
                let (x, y) = (x.parse::<isize>()?, y.parse::<isize>()?);

                if let Some(prev) = prev.take() {
                    for x in x.min(prev.0)..=x.max(prev.0) {
                        for y in y.min(prev.1)..=y.max(prev.1) {
                            map.insert((x, y), Item::Rock);
                        }
                    }
                }

                prev = Some((x, y));
            }
        }

        Ok(Puzzle { map })
    }

    fn range(&self) -> ((isize, isize), (isize, isize)) {
        let mut min = (isize::MAX, isize::MAX);
        let mut max = (isize::MIN, isize::MIN);

        for &(x, y) in self.map.keys() {
            min.0 = min.0.min(x);
            min.1 = min.1.min(y);
            max.0 = max.0.max(x);
            max.1 = max.1.max(y);
        }

        (min, max)
    }

    fn part_1(mut self) -> usize {
        let (_, (_, max_y)) = self.range();

        for i in 0.. {
            let mut p = (500, 0);

            loop {
                if p.1 > max_y {
                    return i;
                }

                let next = (p.0, p.1 + 1);
                if self.map.get(&next).is_none() {
                    p = next;
                    continue;
                }

                let next = (p.0 - 1, p.1 + 1);
                if self.map.get(&next).is_none() {
                    p = next;
                    continue;
                }

                let next = (p.0 + 1, p.1 + 1);
                if self.map.get(&next).is_none() {
                    p = next;
                    continue;
                }

                self.map.insert(p, Item::Sand);
                break;
            }
        }

        unreachable!()
    }

    fn part_2(mut self) -> usize {
        let (_, (_, max_y)) = self.range();

        for i in 1.. {
            let mut p = (500, 0);

            loop {
                if p.1 < max_y + 1 {
                    let next = (p.0, p.1 + 1);
                    if self.map.get(&next).is_none() {
                        p = next;
                        continue;
                    }

                    let next = (p.0 - 1, p.1 + 1);
                    if self.map.get(&next).is_none() {
                        p = next;
                        continue;
                    }

                    let next = (p.0 + 1, p.1 + 1);
                    if self.map.get(&next).is_none() {
                        p = next;
                        continue;
                    }
                }

                self.map.insert(p, Item::Sand);
                break;
            }

            if p == (500, 0) {
                return i;
            }
        }

        unreachable!()
    }
}

impl fmt::Display for Puzzle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let (min, max) = self.range();

        for y in min.1..=max.1 {
            writeln!(f)?;

            for x in min.0..=max.0 {
                match self.map.get(&(x, y)) {
                    Some(Item::Rock) => write!(f, "#")?,
                    Some(Item::Sand) => write!(f, "o")?,
                    Some(Item::Start) => write!(f, "+")?,
                    None => write!(f, ".")?,
                }
            }
        }

        Ok(())
    }
}

fn main() -> Result<(), Error> {
    let puzzle = Puzzle::parse(&read_stdin()?)?;

    println!("Part 1: {}", puzzle.clone().part_1());
    println!("Part 2: {}", puzzle.part_2());

    Ok(())
}

#[cfg(test)]
mod test {
    static INPUT: &str = r#"
        498,4 -> 498,6 -> 496,6
        503,4 -> 502,4 -> 502,9 -> 494,9
    "#;

    #[test]
    fn part_1() -> Result<(), super::Error> {
        let input = super::Puzzle::parse(INPUT)?;

        assert_eq!(input.part_1(), 24);

        Ok(())
    }

    #[test]
    fn part_2() -> Result<(), super::Error> {
        let input = super::Puzzle::parse(INPUT)?;

        assert_eq!(input.part_2(), 93);

        Ok(())
    }
}

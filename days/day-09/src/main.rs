use util::*;

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

fn parse(input: &str) -> Result<Vec<(Direction, usize)>, Error> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (l, r) = line.trim().split_once(' ').context("Missing number")?;
            let n = r.parse()?;
            let d = match l {
                "L" => Direction::Left,
                "R" => Direction::Right,
                "U" => Direction::Up,
                "D" => Direction::Down,
                _ => bail!("Invalid direction: {l}"),
            };

            Ok((d, n))
        })
        .collect()
}

fn rope_sim<const N: usize>(input: &[(Direction, usize)]) -> usize {
    let mut s = HashSet::new();
    let mut rope = [(0isize, 0isize); N];

    s.insert(*rope.last().unwrap());

    for (d, n) in input {
        for _ in 0..*n {
            match d {
                Direction::Left => {
                    rope[0].0 -= 1;
                }
                Direction::Right => {
                    rope[0].0 += 1;
                }
                Direction::Up => {
                    rope[0].1 -= 1;
                }
                Direction::Down => {
                    rope[0].1 += 1;
                }
            }

            for i in 1..rope.len() {
                let head = rope[i - 1];
                let mut tail = rope[i];

                let (dx, dy) = (head.0 - tail.0, head.1 - tail.1);

                if dx < -1 {
                    tail.0 -= 1;

                    match dy.cmp(&0) {
                        Ordering::Less => tail.1 -= 1,
                        Ordering::Greater => tail.1 += 1,
                        _ => {}
                    }
                } else if dx > 1 {
                    tail.0 += 1;

                    match dy.cmp(&0) {
                        Ordering::Less => tail.1 -= 1,
                        Ordering::Greater => tail.1 += 1,
                        _ => {}
                    }
                } else if dy < -1 {
                    tail.1 -= 1;

                    match dx.cmp(&0) {
                        Ordering::Less => tail.0 -= 1,
                        Ordering::Greater => tail.0 += 1,
                        _ => {}
                    }
                } else if dy > 1 {
                    tail.1 += 1;

                    match dx.cmp(&0) {
                        Ordering::Less => tail.0 -= 1,
                        Ordering::Greater => tail.0 += 1,
                        _ => {}
                    }
                }

                rope[i - 1] = head;
                rope[i] = tail;
            }

            s.insert(*rope.last().unwrap());
        }
    }

    s.len()
}

fn part_1(input: &[(Direction, usize)]) -> usize {
    rope_sim::<2>(input)
}

fn part_2(input: &[(Direction, usize)]) -> usize {
    rope_sim::<10>(input)
}

fn main() -> Result<(), Error> {
    let input = parse(&read_stdin()?)?;

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));

    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn part_1() -> Result<(), super::Error> {
        static INPUT: &str = r#"
             R 4
             U 4
             L 3
             D 1
             R 4
             D 1
             L 5
             R 2
         "#;

        let input = super::parse(INPUT)?;

        assert_eq!(super::part_1(&input), 13);

        Ok(())
    }

    #[test]
    fn part_2() -> Result<(), super::Error> {
        static INPUT: &str = r#"
            R 5
            U 8
            L 8
            D 3
            R 17
            D 10
            L 25
            U 20
        "#;

        let input = super::parse(INPUT)?;

        assert_eq!(super::part_2(&input), 36);

        Ok(())
    }
}

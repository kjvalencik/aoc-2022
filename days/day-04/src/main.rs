use util::*;

type Team = (ops::RangeInclusive<usize>, ops::RangeInclusive<usize>);

fn parse_range(input: &str) -> Result<ops::RangeInclusive<usize>, Error> {
    let (l, r) = input.trim().split_once('-').context("Missing end")?;

    Ok(l.parse()?..=r.parse()?)
}

fn parse_line(input: &str) -> Result<Team, Error> {
    let (l, r) = input.trim().split_once(',').context("Missing range")?;

    Ok((parse_range(l)?, parse_range(r)?))
}

fn parse(input: &str) -> Result<Vec<Team>, Error> {
    input.trim().lines().map(parse_line).collect()
}

fn part_1(input: &[Team]) -> usize {
    input
        .iter()
        .filter(|(l, r)| {
            (l.contains(r.start()) && l.contains(r.end()))
                || (r.contains(l.start()) && r.contains(l.end()))
        })
        .count()
}

fn part_2(input: &[Team]) -> usize {
    input
        .iter()
        .filter(|(l, r)| {
            (l.start() <= r.end() && l.end() >= r.start())
                || (r.start() <= l.end() && r.end() >= l.start())
        })
        .count()
}

fn main() -> Result<(), Error> {
    let input = parse(&read_stdin()?)?;

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input));

    Ok(())
}

#[cfg(test)]
mod test {
    static INPUT: &str = r#"
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
    "#;

    #[test]
    fn part_1() -> Result<(), super::Error> {
        let input = super::parse(INPUT)?;

        assert_eq!(super::part_1(&input), 2);

        Ok(())
    }

    #[test]
    fn part_2() -> Result<(), super::Error> {
        let input = super::parse(INPUT)?;

        assert_eq!(super::part_2(&input), 4);

        Ok(())
    }
}

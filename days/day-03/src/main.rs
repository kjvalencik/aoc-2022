use util::*;

type Compartment = [bool; 52];
type Rucksack = (Compartment, Compartment);

fn parse_compartment(items: &[u8]) -> Result<Compartment, Error> {
    let mut compartment = [false; 52];

    for item in items.iter().copied() {
        match item {
            i if (b'A'..=b'Z').contains(&i) => {
                compartment[usize::from(i - b'A' + 26)] = true;
            }
            i if (b'a'..=b'z').contains(&i) => {
                compartment[usize::from(i - b'a')] = true;
            }
            i => bail!("Invalid item: {i}"),
        }
    }

    Ok(compartment)
}

fn parse_line(line: &str) -> Result<Rucksack, Error> {
    let line = line.trim().as_bytes();
    let (left, right) = line.split_at(line.len() / 2);
    let left = parse_compartment(left)?;
    let right = parse_compartment(right)?;

    Ok((left, right))
}

fn parse(input: &str) -> Result<Vec<Rucksack>, Error> {
    input.trim().lines().map(parse_line).collect()
}

fn part_1(rucksacks: &[Rucksack]) -> usize {
    rucksacks
        .iter()
        .map(|(l, r)| {
            for (i, (l, r)) in l.iter().zip(r).enumerate() {
                if *l && *r {
                    return i + 1;
                }
            }

            0
        })
        .sum()
}

fn part_2(rucksacks: &[Rucksack]) -> usize {
    let mut sum = 0;
    let mut it = rucksacks
        .iter()
        .map(|(l, r)| l.iter().zip(r).map(|(l, r)| *l || *r));

    while let (Some(a), Some(b), Some(c)) = (it.next(), it.next(), it.next()) {
        for (i, ((a, b), c)) in a.zip(b).zip(c).enumerate() {
            if a && b && c {
                sum += i + 1;
                break;
            }
        }
    }

    sum
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
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
    "#;

    #[test]
    fn part_1() -> Result<(), super::Error> {
        let input = super::parse(INPUT)?;

        assert_eq!(super::part_1(&input), 157);

        Ok(())
    }

    #[test]
    fn part_2() -> Result<(), super::Error> {
        let input = super::parse(INPUT)?;

        assert_eq!(super::part_2(&input), 70);

        Ok(())
    }
}

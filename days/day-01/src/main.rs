use util::*;

fn parse(input: &str) -> Result<Vec<u64>, Error> {
    let mut nums = input
        .trim()
        .split("\n\n")
        .map(|elf| {
            elf.trim()
                .split('\n')
                .map(|line| line.trim().parse::<u64>())
                .try_fold(0, |acc, n| Ok(acc + n?))
        })
        .collect::<Result<Vec<_>, Error>>()?;

    nums.sort();
    nums.reverse();

    Ok(nums)
}

fn part_1(input: &[u64]) -> u64 {
    input.first().copied().unwrap_or(0)
}

fn part_2(input: &[u64]) -> u64 {
    input.iter().take(3).sum()
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
        1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000
    "#;

    #[test]
    fn part_1() -> Result<(), super::Error> {
        let input = super::parse(INPUT)?;

        assert_eq!(super::part_1(&input), 24000);

        Ok(())
    }

    #[test]
    fn part_2() -> Result<(), super::Error> {
        let input = super::parse(INPUT)?;

        assert_eq!(super::part_2(&input), 45000);

        Ok(())
    }
}

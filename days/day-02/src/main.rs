use util::*;

#[derive(Clone, Copy)]
#[repr(u64)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Clone, Copy)]
#[repr(u64)]
enum Outcome {
    Lose = 0,
    Draw = 1,
    Win = 2,
}

impl Move {
    fn round(self, opponent: Move) -> u64 {
        3 * (match (3 + (self as u64) - (opponent as u64)) % 3 {
            0 => Outcome::Draw,
            1 => Outcome::Win,
            _ => Outcome::Lose,
        }) as u64
    }

    fn score(self, opponent: Move) -> u64 {
        (self as u64) + self.round(opponent)
    }
}

impl FromStr for Move {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            _ => bail!("Unexpected move: {s}"),
        })
    }
}

impl From<Move> for Outcome {
    fn from(m: Move) -> Self {
        match m {
            Move::Rock => Outcome::Lose,
            Move::Paper => Outcome::Draw,
            Move::Scissors => Outcome::Win,
        }
    }
}

fn parse(input: &str) -> Result<Vec<(Move, Move)>, Error> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (l, r) = line.trim().split_once(' ').context("Missing space")?;

            Ok((Move::from_str(l)?, Move::from_str(r)?))
        })
        .collect()
}

fn part_1(games: &[(Move, Move)]) -> u64 {
    games.iter().map(|(l, r)| r.score(*l)).sum()
}

fn part_2(games: &[(Move, Move)]) -> u64 {
    games
        .iter()
        .map(|(l, r)| {
            let r = match (*l as u64 + Outcome::from(*r) as u64) % 3 {
                0 => Move::Paper,
                1 => Move::Scissors,
                _ => Move::Rock,
            };

            (l, r)
        })
        .map(|(l, r)| r.score(*l))
        .sum()
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
        A Y
        B X
        C Z
    "#;

    #[test]
    fn part_1() -> Result<(), super::Error> {
        let games = super::parse(INPUT)?;

        assert_eq!(super::part_1(&games), 15);

        Ok(())
    }

    #[test]
    fn part_2() -> Result<(), super::Error> {
        let games = super::parse(INPUT)?;

        assert_eq!(super::part_2(&games), 12);

        Ok(())
    }
}

use util::*;

fn parse(input: &str) -> Result<Vec<Vec<u8>>, Error> {
    input
        .trim()
        .lines()
        .map(|l| {
            l.trim()
                .chars()
                .map(|c| Ok(c.to_string().parse()?))
                .collect()
        })
        .collect()
}

fn part_1(trees: &[Vec<u8>]) -> usize {
    let width = trees[0].len();
    let height = trees.len();

    (0..width)
        .flat_map(|x| (0..height).map(move |y| (x, y)))
        .filter(|&(x, y)| {
            if (0..x).all(|x2| trees[y][x2] < trees[y][x]) {
                return true;
            }

            if ((x + 1)..width).all(|x2| trees[y][x2] < trees[y][x]) {
                return true;
            }

            if (0..y).all(|y2| trees[y2][x] < trees[y][x]) {
                return true;
            }

            if ((y + 1)..height).all(|y2| trees[y2][x] < trees[y][x]) {
                return true;
            }

            false
        })
        .count()
}

fn part_2(trees: &[Vec<u8>]) -> Result<usize, Error> {
    let width = trees[0].len();
    let height = trees.len();

    (0..width)
        .flat_map(|x| (0..height).map(move |y| (x, y)))
        .map(|(x, y)| {
            let l = (0..x)
                .rev()
                .take_while(|&x2| trees[y][x2] < trees[y][x])
                .count();

            let r = ((x + 1)..width)
                .take_while(|&x2| trees[y][x2] < trees[y][x])
                .count();

            let u = (0..y)
                .rev()
                .take_while(|&y2| trees[y2][x] < trees[y][x])
                .count();

            let d = ((y + 1)..height)
                .take_while(|&y2| trees[y2][x] < trees[y][x])
                .count();

            (l + 1).min(x)
                * (r + 1).min(width - x - 1)
                * (u + 1).min(y)
                * (d + 1).min(height - y - 1)
        })
        .max()
        .context("Empty grid")
}

fn main() -> Result<(), Error> {
    let input = parse(&read_stdin()?)?;

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(&input)?);

    Ok(())
}

#[cfg(test)]
mod test {
    static INPUT: &str = r#"
        30373
        25512
        65332
        33549
        35390
    "#;

    #[test]
    fn part_1() -> Result<(), super::Error> {
        let input = super::parse(INPUT)?;

        assert_eq!(super::part_1(&input), 21);

        Ok(())
    }

    #[test]
    fn part_2() -> Result<(), super::Error> {
        let input = super::parse(INPUT)?;

        assert_eq!(super::part_2(&input)?, 8);

        Ok(())
    }
}

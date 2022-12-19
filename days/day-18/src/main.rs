use std::ops::RangeInclusive;
use util::*;

type Point = (isize, isize, isize);

fn parse(input: &str) -> Result<Vec<Point>, Error> {
    input
        .trim()
        .lines()
        .map(|l| {
            let mut it = l.trim().split(',');

            Ok((
                it.next().context("Missing x")?.parse()?,
                it.next().context("Missing y")?.parse()?,
                it.next().context("Missing z")?.parse()?,
            ))
        })
        .collect()
}

fn part_1(cubes: &[Point]) -> usize {
    let cubes = cubes.iter().collect::<HashSet<_>>();

    cubes
        .iter()
        .flat_map(|&&(x, y, z)| {
            [
                (x - 1, y, z),
                (x + 1, y, z),
                (x, y - 1, z),
                (x, y + 1, z),
                (x, y, z - 1),
                (x, y, z + 1),
            ]
        })
        .filter(|p| !cubes.contains(p))
        .count()
}

fn fill(
    cubes: &HashSet<&Point>,
    point: (isize, isize, isize),
    x_limit: &RangeInclusive<isize>,
    y_limit: &RangeInclusive<isize>,
    z_limit: &RangeInclusive<isize>,
    filled: &mut HashSet<Point>,
    surfaces: &mut usize,
) {
    let (x, y, z) = point;

    if !x_limit.contains(&x) || !y_limit.contains(&y) || !z_limit.contains(&z) {
        return;
    }

    if filled.contains(&point) {
        return;
    }

    if cubes.contains(&point) {
        *surfaces += 1;
        return;
    }

    filled.insert(point);
    fill(
        cubes,
        (x - 1, y, z),
        x_limit,
        y_limit,
        z_limit,
        filled,
        surfaces,
    );
    fill(
        cubes,
        (x + 1, y, z),
        x_limit,
        y_limit,
        z_limit,
        filled,
        surfaces,
    );
    fill(
        cubes,
        (x, y - 1, z),
        x_limit,
        y_limit,
        z_limit,
        filled,
        surfaces,
    );
    fill(
        cubes,
        (x, y + 1, z),
        x_limit,
        y_limit,
        z_limit,
        filled,
        surfaces,
    );
    fill(
        cubes,
        (x, y, z - 1),
        x_limit,
        y_limit,
        z_limit,
        filled,
        surfaces,
    );
    fill(
        cubes,
        (x, y, z + 1),
        x_limit,
        y_limit,
        z_limit,
        filled,
        surfaces,
    );
}

fn part_2(cubes: &[Point]) -> usize {
    let min_x = cubes.iter().map(|(x, _, _)| *x).min().unwrap() - 2;
    let min_y = cubes.iter().map(|(_, y, _)| *y).min().unwrap() - 2;
    let min_z = cubes.iter().map(|(_, _, z)| *z).min().unwrap() - 2;
    let max_x = cubes.iter().map(|(x, _, _)| *x).max().unwrap() + 2;
    let max_y = cubes.iter().map(|(_, y, _)| *y).max().unwrap() + 2;
    let max_z = cubes.iter().map(|(_, _, z)| *z).max().unwrap() + 2;

    let mut filled = HashSet::new();
    let mut surfaces = 0;
    let cubes = cubes.iter().collect::<HashSet<_>>();

    fill(
        &cubes,
        (min_x, min_y, min_z),
        &(min_x..=max_x),
        &(min_y..=max_y),
        &(min_z..=max_z),
        &mut filled,
        &mut surfaces,
    );

    surfaces
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
        2,2,2
        1,2,2
        3,2,2
        2,1,2
        2,3,2
        2,2,1
        2,2,3
        2,2,4
        2,2,6
        1,2,5
        3,2,5
        2,1,5
        2,3,5
    "#;

    #[test]
    fn part_1() -> Result<(), super::Error> {
        let input = super::parse(INPUT)?;

        assert_eq!(super::part_1(&input), 64);

        Ok(())
    }

    #[test]
    fn part_2() -> Result<(), super::Error> {
        let input = super::parse(INPUT)?;

        assert_eq!(super::part_2(&input), 58);

        Ok(())
    }
}

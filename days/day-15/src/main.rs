use std::ops::RangeInclusive;

use util::*;

type Puzzle = HashMap<(isize, isize), (isize, isize)>;

fn parse_pair(input: &str) -> Result<(isize, isize), Error> {
    let (x, y) = input.split_once(", ").context("Missing Y")?;
    let x = x.trim_start_matches("x=").parse()?;
    let y = y.trim_start_matches("y=").parse()?;

    Ok((x, y))
}

fn parse(input: &str) -> Result<Puzzle, Error> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (sensor, beacon) = line.trim().split_once(':').context("Missing beacon")?;
            let sensor = parse_pair(sensor.trim_start_matches("Sensor at "))?;
            let beacon = parse_pair(beacon.trim_start_matches(" closest beacon is at "))?;

            Ok((sensor, beacon))
        })
        .collect()
}

fn intervals(y: isize, input: &Puzzle) -> Vec<RangeInclusive<isize>> {
    let mut intervals = input
        .iter()
        .flat_map(|(sensor, beacon)| {
            let dx = sensor.0.abs_diff(beacon.0);
            let dy = sensor.1.abs_diff(beacon.1);
            let d = dx + dy;
            let ds = y.abs_diff(sensor.1);

            if ds > d {
                None
            } else {
                let d = (d - ds) as isize;

                Some((sensor.0 - d)..=(sensor.0 + d))
            }
        })
        .collect::<Vec<_>>();

    intervals.sort_by(|a, b| a.start().cmp(b.start()));
    intervals
}

fn part_1(y: isize, input: &Puzzle) -> usize {
    let beacons = input
        .values()
        .filter(|(_, n)| y == *n)
        .collect::<HashSet<_>>()
        .len();

    let intervals = intervals(y, input);
    let mut sum = 0;
    let mut min_x = isize::MIN;

    for i in intervals {
        let start = *i.start().max(&min_x);
        let end = *i.end();

        if start <= end {
            sum += start.abs_diff(end) + 1;
            min_x = end + 1;
        }
    }

    sum - beacons
}

fn part_2(input: &Puzzle) -> isize {
    for y in 0..=4000000isize {
        let intervals = intervals(y, input);
        let mut prev_max = isize::MIN;

        for i in intervals {
            let start = *i.start();
            let end = *i.end();

            if start <= end {
                if start > prev_max && prev_max != isize::MIN {
                    return (start - 1) * 4000000 + y;
                }

                prev_max = (end + 1).max(prev_max);
            }
        }
    }

    panic!("No solution")
}

fn main() -> Result<(), Error> {
    let input = parse(&read_stdin()?)?;

    println!("Part 1: {}", part_1(2000000, &input));
    println!("Part 2: {}", part_2(&input));

    Ok(())
}

#[cfg(test)]
mod test {
    static INPUT: &str = r#"
        Sensor at x=2, y=18: closest beacon is at x=-2, y=15
        Sensor at x=9, y=16: closest beacon is at x=10, y=16
        Sensor at x=13, y=2: closest beacon is at x=15, y=3
        Sensor at x=12, y=14: closest beacon is at x=10, y=16
        Sensor at x=10, y=20: closest beacon is at x=10, y=16
        Sensor at x=14, y=17: closest beacon is at x=10, y=16
        Sensor at x=8, y=7: closest beacon is at x=2, y=10
        Sensor at x=2, y=0: closest beacon is at x=2, y=10
        Sensor at x=0, y=11: closest beacon is at x=2, y=10
        Sensor at x=20, y=14: closest beacon is at x=25, y=17
        Sensor at x=17, y=20: closest beacon is at x=21, y=22
        Sensor at x=16, y=7: closest beacon is at x=15, y=3
        Sensor at x=14, y=3: closest beacon is at x=15, y=3
        Sensor at x=20, y=1: closest beacon is at x=15, y=3
    "#;

    #[test]
    fn part_1() -> Result<(), super::Error> {
        let input = super::parse(INPUT)?;

        assert_eq!(super::part_1(10, &input), 26);

        Ok(())
    }

    #[test]
    fn part_2() -> Result<(), super::Error> {
        let input = super::parse(INPUT)?;

        assert_eq!(super::part_2(&input), 56000011);

        Ok(())
    }
}

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

fn contained_by(sensor: (isize, isize), d: usize, x: isize, y: isize) -> bool {
    let dx = x.abs_diff(sensor.0);
    let dy = y.abs_diff(sensor.1);

    if dx + dy <= d {
        return true;
    }

    false
}

fn contained(max: isize, dist: &[((isize, isize), usize)], x: isize, y: isize) -> bool {
    if x < 0 || y < 0 || x > max || y > max {
        return true;
    }

    for (sensor, d) in dist {
        if contained_by(*sensor, *d, x, y) {
            return true;
        }
    }

    false
}

fn part_2(max: isize, input: &Puzzle) -> isize {
    let dist = input
        .iter()
        .map(|(sensor, beacon)| {
            let dx = sensor.0.abs_diff(beacon.0);
            let dy = sensor.1.abs_diff(beacon.1);
            let d = dx + dy;

            (*sensor, d)
        })
        .collect::<Vec<_>>();

    for (sensor, beacon) in input {
        let dx = sensor.0.abs_diff(beacon.0);
        let dy = sensor.1.abs_diff(beacon.1);
        let d = (dx + dy) as isize;

        let left = sensor.0 - d - 1;
        let right = sensor.0 + d + 1;
        let top = sensor.1 - d - 1;
        let bottom = sensor.1 + d + 1;

        if dist.iter().any(move |&(sensor, d)| {
            contained_by(sensor, d, left, sensor.1)
                && contained_by(sensor, d, right, sensor.1)
                && contained_by(sensor, d, sensor.0, top)
                && contained_by(sensor, d, sensor.0, bottom)
        }) {
            continue;
        }

        for x in (sensor.0 - d - 1)..=(sensor.0 + d + 1) {
            let dx = sensor.0.abs_diff(x) as isize;
            let dy = d - dx;

            let y = sensor.1 - dy - 1;
            if !contained(max, &dist, x, y) {
                return x * 4000000 + y;
            }

            let y = sensor.1 + dy + 1;
            if !contained(max, &dist, x, y) {
                return x * 4000000 + y;
            }
        }
    }

    panic!("No solution")
}

fn main() -> Result<(), Error> {
    let input = parse(&read_stdin()?)?;

    println!("Part 1: {}", part_1(2000000, &input));
    println!("Part 2: {}", part_2(4000000, &input));

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

        assert_eq!(super::part_2(20, &input), 56000011);

        Ok(())
    }
}

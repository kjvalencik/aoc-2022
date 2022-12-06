use util::*;

fn find_marker(input: &str, size: usize) -> Result<usize, Error> {
    for (i, w) in input.trim().as_bytes().windows(size).enumerate() {
        if w.iter().collect::<HashSet<_>>().len() == size {
            return Ok(i + size);
        }
    }

    bail!("Could not find start sequence")
}

fn part_1(input: &str) -> Result<usize, Error> {
    find_marker(input, 4)
}

fn part_2(input: &str) -> Result<usize, Error> {
    find_marker(input, 14)
}

fn main() -> Result<(), Error> {
    let input = read_stdin()?;

    println!("Part 1: {}", part_1(&input)?);
    println!("Part 2: {}", part_2(&input)?);

    Ok(())
}

#[cfg(test)]
mod test {
    #[test]
    fn part_1() -> Result<(), super::Error> {
        assert_eq!(super::part_1("mjqjpqmgbljsphdztnvjfqwrcgsmlb")?, 7);
        assert_eq!(super::part_1("bvwbjplbgvbhsrlpgdmjqwftvncz")?, 5);
        assert_eq!(super::part_1("nppdvjthqldpwncqszvftbrmjlhg")?, 6);
        assert_eq!(super::part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")?, 10);
        assert_eq!(super::part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")?, 11);

        Ok(())
    }

    #[test]
    fn part_2() -> Result<(), super::Error> {
        assert_eq!(super::part_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb")?, 19);
        assert_eq!(super::part_2("bvwbjplbgvbhsrlpgdmjqwftvncz")?, 23);
        assert_eq!(super::part_2("nppdvjthqldpwncqszvftbrmjlhg")?, 23);
        assert_eq!(super::part_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")?, 29);
        assert_eq!(super::part_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")?, 26);

        Ok(())
    }
}

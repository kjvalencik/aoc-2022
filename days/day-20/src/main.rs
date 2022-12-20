use util::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Num {
    n: isize,
    tag: usize,
}

fn parse(input: &str) -> Result<Vec<Num>, Error> {
    input
        .trim()
        .lines()
        .enumerate()
        .map(|(tag, l)| {
            Ok(Num {
                n: l.trim().parse()?,
                tag,
            })
        })
        .collect()
}

fn mix(orig: &[Num], nums: &mut [Num]) {
    let len = nums.len() - 1;

    for n in orig {
        let i = nums.iter().position(|v| v == n).unwrap();

        match n.n {
            n if n > 0 => {
                let d = (n as usize) % len;
                let j = i + d;

                if j < len {
                    nums[i..=j].rotate_left(1);
                } else {
                    nums[(j % len)..=i].rotate_right(1);
                }
            }
            n if n < 0 => {
                let d = -n % (len as isize);
                let j = i as isize - d;

                if j < 0 {
                    nums[i..=((j + len as isize) as usize)].rotate_left(1);
                } else {
                    nums[(j as usize)..=i].rotate_right(1);
                }
            }
            _ => {}
        }
    }
}

fn part_1(orig: &[Num]) -> Result<isize, Error> {
    let mut nums = orig.to_vec();

    mix(orig, &mut nums);

    let zero = nums.iter().position(|n| n.n == 0).context("Missing zero")?;
    let get_n = |n: usize| nums[(zero + n) % nums.len()].n;

    Ok(get_n(1000) + get_n(2000) + get_n(3000))
}

fn part_2(mut orig: Vec<Num>) -> Result<isize, Error> {
    let key = 811589153;

    for n in orig.iter_mut() {
        n.n *= key;
    }

    let mut nums = orig.clone();

    for _ in 0..10 {
        mix(&orig, &mut nums);
    }

    let zero = nums.iter().position(|n| n.n == 0).context("Missing zero")?;
    let get_n = |n: usize| nums[(zero + n) % nums.len()].n;

    Ok(get_n(1000) + get_n(2000) + get_n(3000))
}

fn main() -> Result<(), Error> {
    let input = parse(&read_stdin()?)?;

    println!("Part 1: {}", part_1(&input)?);
    println!("Part 2: {}", part_2(input)?);

    Ok(())
}

#[cfg(test)]
mod test {
    static INPUT: &str = r#"
        1
        2
        -3
        3
        -2
        0
        4
    "#;

    #[test]
    fn part_1() -> Result<(), super::Error> {
        let input = super::parse(INPUT)?;

        assert_eq!(super::part_1(&input)?, 3);

        Ok(())
    }

    #[test]
    fn part_2() -> Result<(), super::Error> {
        let input = super::parse(INPUT)?;

        assert_eq!(super::part_2(input)?, 1623178306);

        Ok(())
    }
}

use serde::Deserialize;
use util::*;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
enum Item {
    List(Vec<Item>),
    Value(usize),
}

impl Item {
    fn as_slice(&self) -> &[Item] {
        match self {
            Item::List(v) => v,
            Item::Value(_) => slice::from_ref(self),
        }
    }
}

fn parse(input: &str) -> Result<Vec<Item>, Error> {
    input
        .trim()
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| Ok(serde_json::from_str(l)?))
        .collect()
}

fn compare_lists(l: &[Item], r: &[Item]) -> Ordering {
    for (l, r) in l.iter().zip(r) {
        match compare_pair(l, r) {
            Ordering::Equal => {}
            cmp => return cmp,
        }
    }

    l.len().cmp(&r.len())
}

fn compare_pair(l: &Item, r: &Item) -> Ordering {
    match (l, r) {
        (Item::Value(l), Item::Value(r)) => l.cmp(r),
        _ => compare_lists(l.as_slice(), r.as_slice()),
    }
}

fn part_1(input: &[Item]) -> usize {
    input
        .chunks_exact(2)
        .enumerate()
        .filter(|(_, pair)| compare_pair(&pair[0], &pair[1]) == Ordering::Less)
        .map(|(i, _)| i + 1)
        .sum()
}

fn part_2(mut input: Vec<Item>) -> usize {
    let a = Item::List(vec![Item::List(vec![Item::Value(2)])]);
    let b = Item::List(vec![Item::List(vec![Item::Value(6)])]);

    input.push(a.clone());
    input.push(b.clone());
    input.sort_by(compare_pair);

    let a = input.iter().position(|i| i == &a).unwrap();
    let b = input.iter().position(|i| i == &b).unwrap();

    (a + 1) * (b + 1)
}

fn main() -> Result<(), Error> {
    let input = parse(&read_stdin()?)?;

    println!("Part 1: {}", part_1(&input));
    println!("Part 2: {}", part_2(input));

    Ok(())
}

#[cfg(test)]
mod test {
    static INPUT: &str = r#"
        [1,1,3,1,1]
        [1,1,5,1,1]

        [[1],[2,3,4]]
        [[1],4]

        [9]
        [[8,7,6]]

        [[4,4],4,4]
        [[4,4],4,4,4]

        [7,7,7,7]
        [7,7,7]

        []
        [3]

        [[[]]]
        [[]]

        [1,[2,[3,[4,[5,6,7]]]],8,9]
        [1,[2,[3,[4,[5,6,0]]]],8,9]
    "#;

    #[test]
    fn part_1() -> Result<(), super::Error> {
        let input = super::parse(INPUT)?;

        assert_eq!(super::part_1(&input), 13);

        Ok(())
    }

    #[test]
    fn part_2() -> Result<(), super::Error> {
        let input = super::parse(INPUT)?;

        assert_eq!(super::part_2(input), 140);

        Ok(())
    }
}

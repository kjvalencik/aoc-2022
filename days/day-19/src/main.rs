use util::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
struct Ore(usize);

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
struct Clay(usize);

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
struct Obsidian(usize);

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
struct Geode(usize);

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Eq, Ord)]
struct Time(usize);

#[derive(Debug, Default)]
struct Blueprint {
    ore: Ore,
    clay: Ore,
    obsidian: (Ore, Clay),
    geode: (Ore, Obsidian),
    clay_robot_limit: Clay,
    ore_robot_limit: Ore,
}

#[derive(Clone, Debug, Default)]
struct State {
    time: Time,
    ore: Ore,
    clay: Clay,
    obsidian: Obsidian,
    geodes: Geode,
    ore_robots: Ore,
    clay_robots: Clay,
    obsidian_robots: Obsidian,
}

fn div_ceil(lhs: usize, rhs: usize) -> usize {
    let d = lhs / rhs;

    if lhs % rhs == 0 {
        d
    } else {
        d + 1
    }
}

impl State {
    fn build_geode_robot(&self, blueprint: &Blueprint, max_geodes: &mut Geode) -> bool {
        if self.obsidian_robots.0 == 0 {
            return false;
        }

        let is_building = self.ore >= blueprint.geode.0 && self.obsidian >= blueprint.geode.1;
        let time_delta = if is_building {
            1
        } else {
            let delta = div_ceil(
                if self.ore > blueprint.geode.0 {
                    0
                } else {
                    blueprint.geode.0 .0 - self.ore.0
                },
                self.ore_robots.0,
            )
            .max(div_ceil(
                if blueprint.geode.1 .0 < self.obsidian.0 {
                    0
                } else {
                    blueprint.geode.1 .0 - self.obsidian.0
                },
                self.obsidian_robots.0,
            ));

            1 + delta
        };

        if self.time.0 < time_delta {
            return false;
        }

        round(
            blueprint,
            State {
                time: Time(self.time.0 - time_delta),
                ore: Ore(self.ore.0 + time_delta * self.ore_robots.0 - blueprint.geode.0 .0),
                clay: Clay(self.clay.0 + time_delta * self.clay_robots.0),
                obsidian: Obsidian(
                    self.obsidian.0 + time_delta * self.obsidian_robots.0 - blueprint.geode.1 .0,
                ),
                geodes: Geode(self.geodes.0 + self.time.0 - time_delta),
                ..self.clone()
            },
            max_geodes,
        );

        is_building
    }

    fn build_obsidian_robot(&self, blueprint: &Blueprint, max_geodes: &mut Geode) {
        if self.clay_robots.0 == 0 {
            return;
        }

        let is_building = self.ore >= blueprint.obsidian.0 && self.clay >= blueprint.obsidian.1;

        let time_delta = if is_building {
            1
        } else {
            let delta = div_ceil(
                if blueprint.obsidian.1 .0 < self.clay.0 {
                    0
                } else {
                    blueprint.obsidian.1 .0 - self.clay.0
                },
                self.clay_robots.0,
            )
            .max(div_ceil(
                if blueprint.obsidian.0 .0 < self.ore.0 {
                    0
                } else {
                    blueprint.obsidian.0 .0 - self.ore.0
                },
                self.ore_robots.0,
            ));

            1 + delta
        };

        if self.time.0 <= time_delta + 2 {
            return;
        }

        round(
            blueprint,
            State {
                time: Time(self.time.0 - time_delta),
                obsidian_robots: Obsidian(self.obsidian_robots.0 + 1),
                ore: Ore(self.ore.0 + time_delta * self.ore_robots.0 - blueprint.obsidian.0 .0),
                clay: Clay(self.clay.0 + time_delta * self.clay_robots.0 - blueprint.obsidian.1 .0),
                obsidian: Obsidian(self.obsidian.0 + time_delta * self.obsidian_robots.0),
                ..self.clone()
            },
            max_geodes,
        );
    }

    fn build_clay_robot(&self, blueprint: &Blueprint, max_geodes: &mut Geode) {
        if self.clay_robots >= blueprint.clay_robot_limit {
            return;
        }

        let is_building = self.ore >= blueprint.clay;
        let time_delta = if is_building {
            1
        } else {
            1 + div_ceil(blueprint.clay.0 - self.ore.0, self.ore_robots.0)
        };

        if self.time.0 <= time_delta + 3 {
            return;
        }

        round(
            blueprint,
            State {
                time: Time(self.time.0 - time_delta),
                clay_robots: Clay(self.clay_robots.0 + 1),
                ore: Ore(self.ore.0 + time_delta * self.ore_robots.0 - blueprint.clay.0),
                clay: Clay(self.clay.0 + time_delta * self.clay_robots.0),
                obsidian: Obsidian(self.obsidian.0 + time_delta * self.obsidian_robots.0),
                ..self.clone()
            },
            max_geodes,
        );
    }

    fn build_ore_robot(&self, blueprint: &Blueprint, max_geodes: &mut Geode) {
        if self.ore_robots >= blueprint.ore_robot_limit {
            return;
        }

        let is_building = self.ore >= blueprint.ore;
        let time_delta = if is_building {
            1
        } else {
            1 + div_ceil(blueprint.ore.0 - self.ore.0, self.ore_robots.0)
        };

        if self.time.0 <= time_delta + 4 {
            return;
        }

        round(
            blueprint,
            State {
                time: Time(self.time.0 - time_delta),
                ore_robots: Ore(self.ore_robots.0 + 1),
                ore: Ore(self.ore.0 + time_delta * self.ore_robots.0 - blueprint.ore.0),
                clay: Clay(self.clay.0 + time_delta * self.clay_robots.0),
                obsidian: Obsidian(self.obsidian.0 + time_delta * self.obsidian_robots.0),
                ..self.clone()
            },
            max_geodes,
        );
    }
}

fn round(blueprint: &Blueprint, state: State, max_geodes: &mut Geode) {
    if state.geodes > *max_geodes {
        *max_geodes = state.geodes;
    }

    if state.time.0 == 0 {
        return;
    }

    if state.build_geode_robot(blueprint, max_geodes) {
        return;
    }

    state.build_obsidian_robot(blueprint, max_geodes);
    state.build_clay_robot(blueprint, max_geodes);
    state.build_ore_robot(blueprint, max_geodes);
}

fn simulate(blueprint: &Blueprint, time: usize) -> usize {
    let mut max_geodes = Geode(0);

    round(
        blueprint,
        State {
            time: Time(time),
            ore_robots: Ore(1),
            ..Default::default()
        },
        &mut max_geodes,
    );

    max_geodes.0
}

fn part_1(blueprints: &[Blueprint]) -> usize {
    blueprints
        .iter()
        .enumerate()
        .map(|(i, blueprint)| (i, simulate(blueprint, 24)))
        .map(|(i, max)| (i + 1) * max)
        .sum()
}

fn part_2(blueprints: &[Blueprint]) -> usize {
    blueprints
        .iter()
        .take(3)
        .fold(1, |acc, blueprint| acc * simulate(blueprint, 32))
}

fn parse(input: &str) -> Result<Vec<Blueprint>, Error> {
    input
        .trim()
        .split("Blueprint ")
        .skip(1)
        .map(|s| {
            let mut it = s.split_whitespace();
            let mut blueprint = Blueprint {
                ore: Ore(it.nth(5).context("Missing ore")?.parse()?),
                clay: Ore(it.nth(5).context("Missing clay")?.parse()?),
                obsidian: (
                    Ore(it.nth(5).context("Missing ore")?.parse()?),
                    Clay(it.nth(2).context("Missing clay")?.parse()?),
                ),
                geode: (
                    Ore(it.nth(5).context("Missing ore")?.parse()?),
                    Obsidian(it.nth(2).context("Missing obsidian")?.parse()?),
                ),
                ..Default::default()
            };

            blueprint.clay_robot_limit = blueprint.obsidian.1;
            blueprint.ore_robot_limit = blueprint
                .ore
                .max(blueprint.clay)
                .max(blueprint.obsidian.0)
                .max(blueprint.geode.0);

            Ok(blueprint)
        })
        .collect()
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
        Blueprint 1:
            Each ore robot costs 4 ore.
            Each clay robot costs 2 ore.
            Each obsidian robot costs 3 ore and 14 clay.
            Each geode robot costs 2 ore and 7 obsidian.

        Blueprint 2:
            Each ore robot costs 2 ore.
            Each clay robot costs 3 ore.
            Each obsidian robot costs 3 ore and 8 clay.
            Each geode robot costs 3 ore and 12 obsidian.
    "#;

    #[test]
    fn part_1() -> Result<(), super::Error> {
        let input = super::parse(INPUT)?;

        assert_eq!(super::part_1(&input), 33);

        Ok(())
    }

    #[test]
    fn part_2() -> Result<(), super::Error> {
        let input = super::parse(INPUT)?;

        assert_eq!(super::part_2(&input), 3472);

        Ok(())
    }
}

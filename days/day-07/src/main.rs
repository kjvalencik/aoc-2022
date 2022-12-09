use util::*;

enum Cd {
    Root,
    Up,
    None,
}

#[derive(Debug, Default)]
struct Directory<'a> {
    files: HashMap<&'a str, usize>,
    dirs: HashMap<&'a str, Directory<'a>>,
}

fn parse_cd<'a>(
    name: &'a str,
    cur: &mut Directory<'a>,
    commands: &mut impl Iterator<Item = &'a str>,
) -> Result<Cd, Error> {
    if name == ".." {
        return Ok(Cd::Up);
    }

    if name == "/" {
        return Ok(Cd::Root);
    }

    let cur = cur.dirs.entry(name).or_default();

    while let Some(cmd) = commands.next() {
        match parse_command(cmd, cur, commands)? {
            Cd::Up => return Ok(Cd::None),
            Cd::Root => return Ok(Cd::Root),
            Cd::None => {}
        }
    }

    Ok(Cd::None)
}

fn parse_ls<'a>(lines: &'a str, cur: &mut Directory<'a>) -> Result<(), Error> {
    for line in lines.lines() {
        let (l, r) = line.trim().split_once(' ').context("Invalid file")?;

        if l == "dir" {
            cur.dirs.entry(r.trim()).or_default();
        } else {
            cur.files.insert(r.trim(), l.parse()?);
        }
    }

    Ok(())
}

fn parse_command<'a>(
    cmd: &'a str,
    cur: &mut Directory<'a>,
    commands: &mut impl Iterator<Item = &'a str>,
) -> Result<Cd, Error> {
    let cd = cmd.trim_start_matches("cd ");
    let ls = cmd.trim_start_matches("ls\n");

    if cd.len() < cmd.len() {
        return parse_cd(cd.trim(), cur, commands);
    }

    if ls.len() < cmd.len() {
        parse_ls(ls.trim(), cur)?;

        return Ok(Cd::None);
    }

    bail!("Unexpected command: {cmd}")
}

fn parse(input: &str) -> Result<Directory, Error> {
    let mut root = Directory::default();
    let mut commands = input.trim().split('$').skip(1).map(|c| c.trim());

    while let Some(cmd) = commands.next() {
        parse_command(cmd, &mut root, &mut commands)?;
    }

    Ok(root)
}

fn part_1(root: &Directory) -> usize {
    fn dir_size(total: &mut usize, d: &Directory) -> usize {
        let files = d.files.values().sum::<usize>();
        let dirs = d.dirs.values().map(|d| dir_size(total, d)).sum::<usize>();
        let size = files + dirs;

        if size <= 100000 {
            *total += size;
        }

        size
    }

    let mut total = 0;

    dir_size(&mut total, root);

    total
}

fn part_2(root: &Directory) -> Result<usize, Error> {
    static TOTAL_DISK: usize = 70_000_000;
    static REQUIRED: usize = 30_000_000;

    fn dir_size(d: &Directory) -> usize {
        let files = d.files.values().sum::<usize>();
        let dirs = d.dirs.values().map(dir_size).sum::<usize>();

        files + dirs
    }

    fn find_dir(d: &Directory, min: &mut usize, needed: usize) -> usize {
        let files = d.files.values().sum::<usize>();
        let dirs = d
            .dirs
            .values()
            .map(|d| find_dir(d, min, needed))
            .sum::<usize>();

        let size = files + dirs;

        if size >= needed && size < *min {
            *min = size;
        }

        size
    }

    let mut size = usize::MAX;
    let free = TOTAL_DISK - dir_size(root);
    let needed = REQUIRED - free;

    find_dir(root, &mut size, needed);

    if size == usize::MAX {
        bail!("Could not find a directory to delete");
    }

    Ok(size)
}

fn main() -> Result<(), Error> {
    let input = read_stdin()?;
    let root = parse(&input)?;

    println!("Part 1: {}", part_1(&root));
    println!("Part 2: {}", part_2(&root)?);

    Ok(())
}

#[cfg(test)]
mod test {
    static INPUT: &str = r#"
        $ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k
    "#;

    #[test]
    fn part_1() -> Result<(), super::Error> {
        let input = super::parse(INPUT)?;

        assert_eq!(super::part_1(&input), 95437);

        Ok(())
    }

    #[test]
    fn part_2() -> Result<(), super::Error> {
        let input = super::parse(INPUT)?;

        assert_eq!(super::part_2(&input)?, 24933642);

        Ok(())
    }
}

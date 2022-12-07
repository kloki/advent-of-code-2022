use std::env;
use std::fs;
use unix::{build_file_system, parse_command, Folder, CMD};
mod unix;
fn main() {
    let args: Vec<String> = env::args().collect();
    let fallback = "./input/input.txt".to_owned();
    let path = &args.get(1).unwrap_or(&fallback);
    let contents = fs::read_to_string(path).expect("Can't read file");
    let answer = run_1(contents.clone());
    println!("{}", answer);
    let answer = run_2(contents);
    println!("{}", answer);
}

fn parse_file_system(contents: String) -> Folder {
    let mut commands: Vec<CMD> = contents
        .trim()
        .split("\n$ ")
        .map(parse_command)
        .collect::<Vec<CMD>>();
    commands.reverse();
    commands.pop(); // assume first command is always cd /
    build_file_system("/".to_string(), &mut commands)
}

fn run_1(contents: String) -> u32 {
    let system = parse_file_system(contents);
    system.collect_sizes().iter().filter(|s| **s < 100000).sum()
}
fn run_2(contents: String) -> u32 {
    let system = parse_file_system(contents);
    let needed_space = system.size() - 40000000;
    *system
        .collect_sizes()
        .iter()
        .filter(|s| **s > needed_space)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "$ cd /
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
7214296 k";

    use super::*;
    #[test]
    fn test_day7_1() {
        assert_eq!(run_1(TEST_INPUT.to_string()), 95437)
    }
    #[test]
    fn test_day7_2() {
        assert_eq!(run_2(TEST_INPUT.to_string()), 24933642)
    }
}

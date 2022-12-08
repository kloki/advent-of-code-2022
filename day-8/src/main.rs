use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    let fallback = "./input/input.txt".to_owned();
    let path = &args.get(1).unwrap_or(&fallback);
    let contents = fs::read_to_string(path).expect("Can't read file");
    let answer = run_1(contents.clone());
    println!("{}", answer);
}

#[derive(Debug)]
struct Tree {
    height: u32,
    visible: bool,
}
fn parse_forest(input: String) -> Vec<Vec<Tree>> {
    input
        .trim()
        .split('\n')
        .map(|l| {
            l.chars()
                .map(|n| n.to_digit(10).unwrap())
                .map(|n| Tree {
                    height: n,
                    visible: false,
                })
                .collect()
        })
        .collect()
}

fn get_visible(forest: Vec<Vec<Tree>>) -> usize {
    forest
        .into_iter()
        .map(|l| l.iter().filter(|t| t.visible).count())
        .sum()
}

fn run_1(contents: String) -> usize {
    let mut forest = parse_forest(contents);

    let max_y = forest.len() - 1;
    let max_x = forest[0].len() - 1;

    for y in 1..max_y {
        let mut max_height = 0;

        for x in 0..max_x {
            if forest[y][x].height + 1 > max_height {
                max_height = forest[y][x].height + 1;
                forest[y][x].visible = true;
            }
            if max_height == 10 {
                break;
            }
        }
    }
    for y in 1..max_y {
        let mut max_height = 0;

        for x in (1..max_x + 1).rev() {
            if forest[y][x].height + 1 > max_height {
                max_height = forest[y][x].height + 1;
                forest[y][x].visible = true;
            }
            if max_height == 10 {
                break;
            }
        }
    }

    for x in 1..max_x {
        let mut max_height = 0;

        for y in 0..max_y {
            if forest[y][x].height + 1 > max_height {
                max_height = forest[y][x].height + 1;
                forest[y][x].visible = true;
            }
            if max_height == 10 {
                break;
            }
        }
    }
    for x in 1..max_x {
        let mut max_height = 0;

        for y in (1..max_y + 1).rev() {
            if forest[y][x].height + 1 > max_height {
                max_height = forest[y][x].height + 1;
                forest[y][x].visible = true;
            }
            if max_height == 9 {
                break;
            }
        }
    }
    forest[0][0].visible = true;
    forest[0][max_x].visible = true;
    forest[max_y][0].visible = true;
    forest[max_y][max_x].visible = true;

    get_visible(forest)
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "30373
25512
65332
33549
35390";

    use super::*;
    #[test]
    fn test_day8_1() {
        assert_eq!(run_1(TEST_INPUT.to_string()), 21)
    }
    // #[test]
    // fn test_day8_2() {
    //     assert_eq!(run_1(TEST_INPUT.to_string()), 8)
    // }
}

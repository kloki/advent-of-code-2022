use pair::Pair;
use std::env;
use std::fs;
mod pair;
fn main() {
    let args: Vec<String> = env::args().collect();
    let fallback = "./input/input.txt".to_owned();
    let path = &args.get(1).unwrap_or(&fallback);
    let contents = fs::read_to_string(&path).expect("Can't read file");
    let answer = run_1(contents.clone());
    println!("{}", answer);
    let answer = run_2(contents.clone());
    println!("{}", answer);
}

fn run_1(contents: String) -> usize {
    contents
        .trim()
        .split("\n")
        .map(|l| l.parse::<Pair>().unwrap())
        .map(|p| p.contained())
        .sum()
}

fn run_2(contents: String) -> usize {
    contents
        .trim()
        .split("\n")
        .map(|l| l.parse::<Pair>().unwrap())
        .map(|p| p.overlapped())
        .sum()
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    use super::*;
    #[test]
    fn test_day4_1() {
        assert_eq!(run_1(TEST_INPUT.to_string()), 2)
    }
    #[test]
    fn test_day4_2() {
        assert_eq!(run_2(TEST_INPUT.to_string()), 4)
    }
}

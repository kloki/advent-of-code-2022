use std::env;
use std::fs;
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

fn run_1(contents: String) -> u32 {
    contents
        .trim()
        .split("\n\n")
        .map(|s| s.split('\n').map(|t| t.parse::<u32>().unwrap()).sum())
        .max()
        .unwrap()
}
fn run_2(contents: String) -> u32 {
    let mut scores: Vec<u32> = contents
        .trim()
        .split("\n\n")
        .map(|s| s.split('\n').map(|t| t.parse::<u32>().unwrap()).sum())
        .collect();
    scores.sort();
    scores[scores.len() - 3..scores.len()].iter().sum()
}
#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    use super::*;
    #[test]
    fn test_day1_1() {
        assert_eq!(run_1(TEST_INPUT.to_string()), 24000)
    }
    #[test]
    fn test_day1_2() {
        assert_eq!(run_2(TEST_INPUT.to_string()), 45000)
    }
}

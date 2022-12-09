use rope::Motion;
use std::env;
use std::fs;
mod rope;
fn main() {
    let args: Vec<String> = env::args().collect();
    let fallback = "./input/input.txt".to_owned();
    let path = &args.get(1).unwrap_or(&fallback);
    let contents = fs::read_to_string(path).expect("Can't read file");
    let answer = run_1(contents.clone());
    println!("{}", answer);
}

fn run_1(contents: String) -> usize {
    0
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    use super::*;
    #[test]
    fn test_day9_1() {
        assert_eq!(run_1(TEST_INPUT.to_string()), 13)
    }
}

use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    let fallback = "./input/input.txt".to_owned();
    let path = &args.get(1).unwrap_or(&fallback);
    let contents = fs::read_to_string(path).expect("Can't read file");
    let answer = run(contents.clone());
    println!("{}", answer);
}

fn run(contents: String) -> usize {
    0
}

#[cfg(test)]
mod tests {

    const TEST_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    use super::*;
    #[test]
    fn test_day12_1() {
        assert_eq!(run(TEST_INPUT.to_string()), 21);
    }
}

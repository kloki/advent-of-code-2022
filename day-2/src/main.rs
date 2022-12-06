use game::Game;
use std::env;
use std::fs;
mod game;
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
    let game = Game::new();
    contents.trim().split('\n').map(|g| game.score_1(g)).sum()
}
fn run_2(contents: String) -> u32 {
    let game = Game::new();
    contents.trim().split('\n').map(|g| game.score_2(g)).sum()
}
#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "A Y
B X
C Z";

    use super::*;
    #[test]
    fn test_day2_1() {
        assert_eq!(run_1(TEST_INPUT.to_string()), 15)
    }
    #[test]
    fn test_day2_2() {
        assert_eq!(run_2(TEST_INPUT.to_string()), 12)
    }
}

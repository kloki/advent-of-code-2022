use rope::{Motion, Rope};
use std::env;
use std::fs;
mod rope;
fn main() {
    let args: Vec<String> = env::args().collect();
    let fallback = "./input/input.txt".to_owned();
    let path = &args.get(1).unwrap_or(&fallback);
    let contents = fs::read_to_string(path).expect("Can't read file");
    let answer = run(contents.clone(), 2);
    println!("{}", answer);
    let answer = run(contents, 10);
    println!("{}", answer);
}

fn run(contents: String, size: usize) -> usize {
    let motions: Vec<Motion> = contents.trim().split('\n').map(Motion::new).collect();
    let mut rope = Rope::new(size);
    for m in motions.iter() {
        rope.process(m);
    }
    rope.count()
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
    const TEST_INPUT_2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    use super::*;
    #[test]
    fn test_day9_1() {
        assert_eq!(run(TEST_INPUT.to_string(), 2), 13)
    }
    #[test]
    fn test_day9_2_1() {
        assert_eq!(run(TEST_INPUT.to_string(), 10), 1)
    }
    #[test]
    fn test_day9_2_2() {
        assert_eq!(run(TEST_INPUT_2.to_string(), 10), 36)
    }
}

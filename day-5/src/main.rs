use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    let fallback = "./input/input.txt".to_owned();
    let path = &args.get(1).unwrap_or(&fallback);
    let contents = fs::read_to_string(&path).expect("Can't read file");
    let answer = run_1(contents.clone());
    println!("{}", answer);
}

fn run_1(contents: String) -> String {
    "ABC".to_string()
}
#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    use super::*;
    #[test]
    fn test_day5_1() {
        assert_eq!(run_1(TEST_INPUT.to_string()), "CMZ".to_string())
    }
}

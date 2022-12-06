use boxes::{BoxStack, Operation};
use std::env;
use std::fs;
mod boxes;
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

fn run_1(contents: String) -> String {
    let parts: Vec<&str> = contents.split("\n\n").collect();
    let mut stacks = BoxStack::new(parts[0]);
    let operations: Vec<Operation> = parts[1].trim().split('\n').map(Operation::new).collect();
    for o in operations.into_iter() {
        stacks.process_9000(o)
    }

    stacks.top()
}

fn run_2(contents: String) -> String {
    let parts: Vec<&str> = contents.split("\n\n").collect();
    let mut stacks = BoxStack::new(parts[0]);
    let operations: Vec<Operation> = parts[1].trim().split('\n').map(Operation::new).collect();
    for o in operations.into_iter() {
        stacks.process_9001(o)
    }

    stacks.top()
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
    #[test]
    fn test_day5_2() {
        assert_eq!(run_2(TEST_INPUT.to_string()), "MCD".to_string())
    }
}

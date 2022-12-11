use monkey::Monkey;
use std::env;
use std::fs;
mod monkey;
fn main() {
    let args: Vec<String> = env::args().collect();
    let fallback = "./input/input.txt".to_owned();
    let path = &args.get(1).unwrap_or(&fallback);
    let contents = fs::read_to_string(path).expect("Can't read file");
    let answer = run(contents);
    println!("{}", answer);
}

fn run(contents: String) -> usize {
    let monkeys: Vec<Monkey> = contents.trim().split("\n\n").map(Monkey::new).collect();
    let mut inspections: Vec<usize> = monkeys.iter().map(|m| m.inspections).collect();
    inspections.sort();
    inspections[inspections.len()] * inspections[inspections.len() - 1]
}

#[cfg(test)]
mod tests {

    const TEST_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    use super::*;
    #[test]
    fn test_day11_1() {
        assert_eq!(run(TEST_INPUT.to_string()), 10605);
    }
}

use std::env;
use std::fs;
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

fn get_score(letter: char) -> usize {
    "0abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .find(letter)
        .unwrap()
}

fn priority_bag(input: &str) -> usize {
    let mut duplicate = '0';
    for letter in input[0..input.len() / 2].chars() {
        if input[input.len() / 2..input.len()].contains(letter) {
            duplicate = letter;
            break;
        }
    }
    get_score(duplicate)
}

fn run_1(contents: String) -> usize {
    contents.trim().split("\n").map(|s| priority_bag(s)).sum()
}
fn run_2(contents: String) -> usize {
    let mut bags = contents.trim().split("\n").peekable();
    let mut sum = 0;
    while bags.peek().is_some() {
        let bag1 = bags.next().unwrap();
        let bag2 = bags.next().unwrap();
        let bag3 = bags.next().unwrap();
        for letter in bag1.chars() {
            if bag2.contains(letter) && bag3.contains(letter) {
                sum += get_score(letter);
                break;
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    use super::*;
    #[test]
    fn test_day3_1() {
        assert_eq!(run_1(TEST_INPUT.to_string()), 157)
    }
    #[test]
    fn test_day3_2() {
        assert_eq!(run_2(TEST_INPUT.to_string()), 70)
    }
}

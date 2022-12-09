use itertools::Itertools;
use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    let fallback = "./input/input.txt".to_owned();
    let path = &args.get(1).unwrap_or(&fallback);
    let contents = fs::read_to_string(path).expect("Can't read file");
    let answer = run(contents.clone(), 4);
    println!("{}", answer);
    let answer = run(contents, 14);
    println!("{}", answer);
}

fn run(contents: String, size: usize) -> usize {
    let letters: Vec<char> = contents.trim().chars().collect();
    for (i, window) in letters.windows(size).enumerate() {
        if window.iter().unique().count() == size {
            return i + size;
        }
    }
    0
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_day6_1_1() {
        assert_eq!(run("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(), 4), 5)
    }
    #[test]
    fn test_day6_1_2() {
        assert_eq!(run("nppdvjthqldpwncqszvftbrmjlhg".to_string(), 4), 6)
    }
    #[test]
    fn test_day6_1_3() {
        assert_eq!(run("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(), 4), 10)
    }
    #[test]
    fn test_day6_1_4() {
        assert_eq!(run("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(), 4), 11)
    }
    #[test]
    fn test_day6_2_1() {
        assert_eq!(run("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string(), 14), 19)
    }
    #[test]
    fn test_day6_2_2() {
        assert_eq!(run("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(), 14), 23)
    }
    #[test]
    fn test_day6_2_3() {
        assert_eq!(run("nppdvjthqldpwncqszvftbrmjlhg".to_string(), 14), 23)
    }
    #[test]
    fn test_day6_2_4() {
        assert_eq!(run("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(), 14), 29)
    }
    #[test]
    fn test_day6_2_5() {
        assert_eq!(run("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(), 14), 26)
    }
}

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
    let orderings: Vec<bool> = contents
        .trim()
        .split("\n\n")
        .map(|p| p.split_once('\n').unwrap())
        .map(|p| compare_order(p.0, p.1))
        .collect();
    let mut score = 0;
    for (i, ordering) in orderings.iter().enumerate() {
        if *ordering {
            score += i + 1;
        }
    }

    score
}

fn compare_order(left: &str, right: &str) -> bool {
    true
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_day13_1_1() {
        assert!(compare_order("[1,1,3,1,1]", "[1,1,5,1,1]"));
    }
    #[test]
    fn test_day13_1_2() {
        assert!(compare_order("[[1],[2,3,4]]", "[[1],4]"));
    }
    #[test]
    fn test_day13_1_3() {
        assert!(!compare_order("[9]", "[[8,7,6]]"));
    }
    #[test]
    fn test_day13_1_4() {
        assert!(compare_order("[[4,4],4,4]", "[[4,4],4,4,4]"));
    }
    #[test]
    fn test_day13_1_5() {
        assert!(!compare_order("[7,7,7,7]", "[7,7,7]"));
    }
    #[test]
    fn test_day13_1_6() {
        assert!(compare_order("[]", "[3]"));
    }
    #[test]
    fn test_day13_1_7() {
        assert!(!compare_order("[[[]]]", "[[]]"));
    }
    #[test]
    fn test_day13_1_8() {
        assert!(!compare_order(
            "[1,[2,[3,[4,[5,6,7]]]],8,9]",
            "1,[2,[3,[4,[5,6,0]]]],8,9]"
        ));
    }
}

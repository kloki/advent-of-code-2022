use std::env;
use std::fs;
fn main() {
    let args: Vec<String> = env::args().collect();
    let fallback = "./input/input.txt".to_owned();
    let path = &args.get(1).unwrap_or(&fallback);
    let contents = fs::read_to_string(&path).expect("Can't read file");
    println!("{}", contents);
}
#[cfg(test)]
mod tests {
    #[test]
    fn test_day0_1() {
        assert_eq!(1, 1)
    }
}

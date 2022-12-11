pub enum Operation {
    Add,
    Multiply,
}
pub enum Value {
    Old,
    Int(isize),
}
pub struct Monkey {
    items: Vec<isize>,
    operation: Operation,
    value: Value,
    test: isize,
    test_pass: usize,
    test_fail: usize,
}

impl Monkey {
    pub fn new(&str: input) -> Self {}
}

mod tests {

    const TEST_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3";

    use super::*;
    #[test]
    fn test_monkey_parsing() {
        let monkey = Monkey::new(TEST_INPUT.to_string());
        assert!(&monkey.items.contains(&79));
        assert!(&monkey.items.contains(&98));
        assert_eq!(&monkey.test, &23);
        assert_eq!(&monkey.test_pass, &2);
        assert_eq!(&monkey.test_pass, &3);
    }
}

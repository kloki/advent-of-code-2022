#[derive(Debug)]
pub enum Operation {
    Add,
    Multiply,
}

#[derive(Debug)]
pub enum Value {
    Old,
    Int(isize),
}

#[derive(Debug)]
pub struct Monkey {
    items: Vec<isize>,
    operation: Operation,
    value: Value,
    test: isize,
    test_pass: usize,
    test_fail: usize,
}

impl Monkey {
    pub fn new(input: &str) -> Self {
        let lines: Vec<&str> = input.split('\n').collect();
        let items = lines[1]
            .split_once("items: ")
            .unwrap()
            .1
            .split(", ")
            .map(|s| s.parse::<isize>().unwrap())
            .collect();
        let mut operation = Operation::Add;
        let mut value = Value::Old;
        let parts: Vec<&str> = lines[2].trim().split(' ').collect();
        if parts[4] == "*" {
            operation = Operation::Multiply
        }
        if parts[5] != "old" {
            value = Value::Int(parts[5].parse().unwrap())
        }
        let test = lines[3]
            .split_once("by ")
            .unwrap()
            .1
            .parse::<isize>()
            .unwrap();
        let test_pass = lines[4]
            .split_once("monkey ")
            .unwrap()
            .1
            .parse::<usize>()
            .unwrap();
        let test_fail = lines[5]
            .split_once("monkey ")
            .unwrap()
            .1
            .parse::<usize>()
            .unwrap();
        Monkey {
            items,
            operation,
            value,
            test,
            test_pass,
            test_fail,
        }
    }
}

#[cfg(test)]
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
        let monkey = Monkey::new(TEST_INPUT);
        assert!(&monkey.items.contains(&79));
        assert!(&monkey.items.contains(&98));
        assert_eq!(&monkey.operation, Operation::Multiply);
        assert_eq!(&monkey.value, Value::Int(19));
        assert_eq!(&monkey.test_pass, &2);
        assert_eq!(&monkey.test_fail, &3);
    }
}

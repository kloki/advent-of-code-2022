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
    pub test: isize,
    pub modulo: isize,
    test_pass: usize,
    test_fail: usize,
    pub inspections: usize,
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
            inspections: 0,
            modulo: 1,
        }
    }

    pub fn give(&mut self, item: isize) {
        self.items.push(item)
    }

    pub fn worry(&mut self) -> Vec<(usize, isize)> {
        let mut items: Vec<(usize, isize)> = Vec::new();
        for item in self.items.drain(..) {
            self.inspections += 1;
            let value = match self.value {
                Value::Old => item,
                Value::Int(s) => s,
            };
            let mut new_item = match self.operation {
                Operation::Add => item + value,
                Operation::Multiply => item * value,
            };

            if self.modulo == 1 {
                new_item /= 3;
            }
            let mut next_monkey = self.test_fail;
            if new_item % self.test == 0 {
                next_monkey = self.test_pass;
            }

            if self.modulo != 1 {
                new_item %= self.modulo;
            }
            items.push((next_monkey, new_item))
        }

        items
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
        assert!(matches!(&monkey.operation, Operation::Multiply));
        assert!(matches!(&monkey.value, Value::Int(19)));
        assert_eq!(&monkey.test_pass, &2);
        assert_eq!(&monkey.test_fail, &3);
    }
}

#[derive(Debug)]
pub struct BoxStack {
    stacks: Vec<Vec<char>>,
}
impl BoxStack {
    pub fn new(input: &str) -> Self {
        let mut lines: Vec<&str> = input.split('\n').collect();
        let length = lines
            .pop()
            .unwrap()
            .chars()
            .filter(|c| !c.is_whitespace())
            .last()
            .unwrap()
            .to_digit(10)
            .unwrap() as usize;

        let mut stacks = vec![Vec::<char>::new(); length];

        for line in lines.into_iter().rev() {
            let box_letters: Vec<char> = line.chars().collect();
            for x in 0..length {
                if box_letters[1 + x * 4] != ' ' {
                    stacks[x].push(box_letters[1 + x * 4])
                }
            }
        }
        BoxStack { stacks }
    }

    pub fn top(&self) -> String {
        let mut answer = "".to_string();
        for stack in self.stacks.iter() {
            answer.push(stack[stack.len() - 1])
        }
        answer
    }

    pub fn process_9000(&mut self, operation: Operation) {
        for _ in 0..operation.amount {
            let letter = self.stacks[operation.source - 1].pop().unwrap();
            self.stacks[operation.target - 1].push(letter)
        }
    }
    pub fn process_9001(&mut self, operation: Operation) {
        let l = self.stacks[operation.source - 1].len();
        let mut letters: Vec<char> = self.stacks[operation.source - 1]
            .drain(l - operation.amount..l)
            .collect();
        self.stacks[operation.target - 1].append(&mut letters);
    }
}

#[derive(Debug)]
pub struct Operation {
    source: usize,
    target: usize,
    amount: usize,
}
impl Operation {
    pub fn new(input: &str) -> Self {
        let parts: Vec<&str> = input.split(' ').collect();
        Operation {
            source: parts[3].parse().unwrap(),
            target: parts[5].parse().unwrap(),
            amount: parts[1].parse().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    const TEST_INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 ";

    use super::*;
    #[test]
    fn test_stacks() {
        let stacks = BoxStack::new(&TEST_INPUT.to_string());
        assert_eq!(stacks.top(), "NDP");
    }
}

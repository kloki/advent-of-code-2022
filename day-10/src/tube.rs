#[derive(Debug)]
pub enum Signal {
    Noop,
    Addx(isize),
}

impl Signal {
    pub fn new(input: &str) -> Self {
        if input == "noop" {
            return Signal::Noop;
        }
        let parts = input.split_once(' ').unwrap();
        Signal::Addx(parts.1.parse().unwrap())
    }
}

pub struct Tube {
    output: isize,
    size: usize,
    tube: Vec<isize>,
}

impl Tube {
    pub fn new() -> Self {
        Tube {
            output: 1,
            size: 2,
            tube: vec![0; 2],
        }
    }
    pub fn process(&mut self, signal: &Signal) -> isize {
        dbg!(self.tube[0]);
        self.output += self.tube[0];

        dbg!(self.output);
        for i in 1..self.size {
            self.tube[i - 1] = self.tube[i];
        }
        match signal {
            Signal::Addx(s) => self.tube[1] = *s,
            _ => self.tube[1] = 0,
        }
        self.output
    }
}

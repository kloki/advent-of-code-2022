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
}

impl Tube {
    pub fn new() -> Self {
        Tube { output: 1 }
    }
    pub fn process(&mut self, signal: &Signal) -> Vec<isize> {
        match signal {
            Signal::Addx(s) => {
                let old = self.output;
                self.output += *s;
                vec![old, self.output]
            }
            Signal::Noop => vec![self.output],
        }
    }
}

use std::collections::HashSet;
pub enum Motion {
    L(usize),
    R(usize),
    U(usize),
    D(usize),
}

impl Motion {
    pub fn new(input: &str) -> Self {
        let parts = input.split_once(" ").unwrap();
        match parts.0 {
            "L" => Motion::L(parts.1.parse().unwrap()),
            "R" => Motion::R(parts.1.parse().unwrap()),
            "U" => Motion::U(parts.1.parse().unwrap()),
            "D" => Motion::D(parts.1.parse().unwrap()),
            _ => panic!("oops"),
        }
    }
}

pub struct Rope {
    knots: Vec<(isize, isize)>,
    visited: HashSet<(isize, isize)>,
    size: usize,
}
impl Rope {
    pub fn new(size: usize) -> Self {
        Rope {
            knots: vec![(0, 0); 10],
            visited: HashSet::from([(0, 0)]),
            size,
        }
    }
    pub fn count(&self) -> usize {
        self.visited.len()
    }

    pub fn process(&mut self, motion: &Motion) {
        let mut delta_x = 0;
        let mut delta_y = 0;
        let steps;
        match motion {
            Motion::L(s) => {
                delta_x = -1;
                steps = *s
            }
            Motion::R(s) => {
                delta_x = 1;
                steps = *s
            }
            Motion::U(s) => {
                delta_y = 1;
                steps = *s
            }
            Motion::D(s) => {
                delta_y = -1;
                steps = *s
            }
        }
        for _ in 0..steps {
            self.knots[0].0 += delta_x;
            self.knots[0].1 += delta_y;
            for i in 1..self.size {
                let diff_x = self.knots[i - 1].0 - self.knots[i].0;
                let diff_y = self.knots[i - 1].1 - self.knots[i].1;
                if diff_x.abs() == 0 && diff_y.abs() == 2 {
                    self.knots[i].1 += diff_y / 2;
                } else if diff_x.abs() == 2 && diff_y.abs() == 0 {
                    self.knots[i].0 += diff_x / 2;
                } else if diff_x.abs() == 1 && diff_y.abs() == 2 {
                    self.knots[i].0 += diff_x;
                    self.knots[i].1 += diff_y / 2;
                } else if diff_x.abs() == 2 && diff_y.abs() == 1 {
                    self.knots[i].0 += diff_x / 2;
                    self.knots[i].1 += diff_y;
                }
            }

            self.visited
                .insert((self.knots[self.size - 1].0, self.knots[self.size - 1].1));
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Node {
    pub height: usize,
    pub distance: usize,
    pub letter: char,
}

impl Node {
    pub fn new(letter: char) -> Self {
        let mut height = "SabcdefghijklmnopqrstuvwxyzE".find(letter).unwrap();
        if letter == 'S' {
            height = 1;
        } else if letter == 'E' {
            height = 26;
        }

        Node {
            height,
            letter,
            distance: 0,
        }
    }
    pub fn is_goal(&self) -> bool {
        self.letter == 'E'
    }
    pub fn is_start(&self) -> bool {
        self.letter == 'S'
    }
}

pub fn parse_map(contents: String) -> Vec<Vec<Node>> {
    contents
        .trim()
        .split('\n')
        .map(|l| l.chars().map(Node::new).collect())
        .collect()
}

pub fn get_start(map: &Vec<Vec<Node>>) -> (usize, usize) {
    for (y, line) in map.iter().enumerate() {
        for (x, node) in line.iter().enumerate() {
            if node.is_start() {
                return (y, x);
            }
        }
    }
    (0, 0)
}
pub fn get_start_candidates(map: &Vec<Vec<Node>>) -> Vec<(usize, usize)> {
    let mut candidates: Vec<(usize, usize)> = Vec::new();
    for (y, line) in map.iter().enumerate() {
        for (x, node) in line.iter().enumerate() {
            if node.height == 1 {
                candidates.push((y, x));
            }
        }
    }
    candidates
}

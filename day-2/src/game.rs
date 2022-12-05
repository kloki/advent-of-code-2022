#[derive(Debug, Clone, Copy)]
pub struct Game {}

// A = Rock 1
// B = Paper 2
// C = Scissor 3
// X = Rock
// Y = Paper
// Z = Scissor

impl Game {
    pub fn new() -> Self {
        Game {}
    }
    pub fn score_1(self, input: &str) -> u32 {
        match input {
            "A X" => 3 + 1,
            "A Y" => 6 + 2,
            "A Z" => 0 + 3,
            "B X" => 0 + 1,
            "B Y" => 3 + 2,
            "B Z" => 6 + 3,
            "C X" => 6 + 1,
            "C Y" => 0 + 2,
            "C Z" => 3 + 3,
            _ => 0,
        }
    }
    pub fn score_2(self, input: &str) -> u32 {
        match input {
            "A X" => 3 + 0,
            "A Y" => 1 + 3,
            "A Z" => 2 + 6,
            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,
            "C X" => 2 + 0,
            "C Y" => 3 + 3,
            "C Z" => 1 + 6,
            _ => 0,
        }
    }
}

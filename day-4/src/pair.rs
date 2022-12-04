#[derive(Debug)]
pub struct Pair {
    ls: usize,
    le: usize,
    rs: usize,
    re: usize,
}
impl Pair {
    pub fn new(input: &str) -> Self {
        let parsed: Vec<Vec<usize>> = input
            .split(",")
            .map(|p| p.split("-").map(|c| c.parse::<usize>().unwrap()).collect())
            .collect();
        Pair {
            ls: parsed[0][0],
            le: parsed[0][1],
            rs: parsed[1][0],
            re: parsed[1][1],
        }
    }
    pub fn contained(&self) -> usize {
        if ((self.ls <= self.rs) && (self.re <= self.le))
            || ((self.rs <= self.ls) && (self.le <= self.re))
        {
            return 1;
        }
        0
    }
    pub fn overlapped(&self) -> usize {
        if ((self.ls <= self.rs) && (self.le >= self.rs))
            || ((self.rs <= self.ls) && (self.re >= self.ls))
            || ((self.le >= self.re) && (self.ls <= self.re))
            || ((self.re >= self.le) && (self.rs <= self.le))
        {
            return 1;
        }
        0
    }
}

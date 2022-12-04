use std::str::FromStr;

#[derive(Debug)]
pub struct Pair {
    ls: usize,
    le: usize,
    rs: usize,
    re: usize,
}
impl Pair {
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

#[derive(Debug)]
pub struct PairParseError;
impl FromStr for Pair {
    type Err = PairParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed: Vec<Vec<usize>> = s
            .split(",")
            .map(|p| {
                p.split("-")
                    .filter_map(|c| c.parse::<usize>().ok())
                    .collect()
            })
            .collect();
        if parsed.len() != 2 || parsed[0].len() != 2 || parsed[0].len() != 2 {
            return Err(PairParseError);
        }

        Ok(Pair {
            ls: parsed[0][0],
            le: parsed[0][1],
            rs: parsed[1][0],
            re: parsed[1][1],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_pair_success() {
        assert!("1-2,1-2".parse::<Pair>().is_ok())
    }
    #[test]
    fn test_pair_garbage() {
        assert!("wrong".parse::<Pair>().is_err())
    }
    #[test]
    fn test_pair_no_int() {
        assert!("a-5,w-2".parse::<Pair>().is_err())
    }
}

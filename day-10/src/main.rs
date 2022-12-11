use std::env;
use std::fs;
use tube::{Signal, Tube};
mod tube;
fn main() {
    let args: Vec<String> = env::args().collect();
    let fallback = "./input/input.txt".to_owned();
    let path = &args.get(1).unwrap_or(&fallback);
    let contents = fs::read_to_string(path).expect("Can't read file");
    let answer = run(contents);
    let score = answer[19] * 20
        + answer[59] * 60
        + answer[99] * 100
        + answer[139] * 140
        + answer[179] * 180
        + answer[219] * 220;
    println!("{}", score);
    print_screens(answer)
}

fn print_screens(outputs: Vec<isize>) {
    for i in 0..6 {
        let mut output = "".to_string();
        for j in 0..40 {
            if (outputs[i * 40 + j] - j as isize).abs() < 2 {
                output.push('⬜');
            } else {
                output.push('⬛')
            }
        }

        println!("{}", output);
    }
}

fn run(contents: String) -> Vec<isize> {
    let signals: Vec<Signal> = contents.trim().split('\n').map(Signal::new).collect();
    let mut tube = Tube::new();
    signals
        .iter()
        .map(|s| tube.process(s))
        .fold(vec![1], |mut acc, mut x| {
            acc.append(&mut x);
            acc
        })
}

#[cfg(test)]
mod tests {
    const TEST_INPUT_SHORT: &str = "noop
addx 3
addx -5";
    const TEST_INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    use super::*;
    #[test]
    fn test_day10_0() {
        let answer = run(TEST_INPUT_SHORT.to_string());
        dbg!(&answer);
        assert_eq!(answer[3], 4);
        assert_eq!(answer[5], -1);
    }
    #[test]
    fn test_day10() {
        let answer = run(TEST_INPUT.to_string());
        assert_eq!(answer[19], 21);
        assert_eq!(answer[59], 19);
        assert_eq!(answer[99], 18);
        assert_eq!(answer[139], 21);
        assert_eq!(answer[179], 16);
        assert_eq!(answer[219], 18);
        print_screens(answer);

        // assert_eq!(2, 1);
    }
}

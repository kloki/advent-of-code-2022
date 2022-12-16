use node::{get_end_score, get_start, get_start_candidates, parse_map, Node};
use std::env;
use std::fs;
mod node;
fn main() {
    let args: Vec<String> = env::args().collect();
    let fallback = "./input/input.txt".to_owned();
    let path = &args.get(1).unwrap_or(&fallback);
    let contents = fs::read_to_string(path).expect("Can't read file");
    let answer = run_1(contents.clone());
    println!("{}", answer);
    let answer = run_2(contents);
    println!("{}", answer);
}

fn update_node(
    map: &mut Vec<Vec<Node>>,
    y: usize,
    x: usize,
    height: usize,
    distance: usize,
) -> Option<(usize, usize)> {
    if map[y][x].height < height && (map[y][x].distance > distance || map[y][x].distance == 0) {
        map[y][x].distance = distance;
        return Some((y, x));
    }

    None
}

fn run_1(contents: String) -> usize {
    let mut map = parse_map(contents);
    let mut candidates = vec![get_start(&map)];
    while !candidates.is_empty() {
        let (y, x) = candidates.remove(0);

        let distance = &map[y][x].distance + 1;
        let height = &map[y][x].height + 2;
        if x != 0 {
            if let Some(c) = update_node(&mut map, y, x - 1, height, distance) {
                candidates.push(c);
            }
        }
        if y != 0 {
            if let Some(c) = update_node(&mut map, y - 1, x, height, distance) {
                candidates.push(c);
            }
        }
        if x != (map[0].len() - 1) {
            if let Some(c) = update_node(&mut map, y, x + 1, height, distance) {
                candidates.push(c);
            }
        }
        if y != (map.len() - 1) {
            if let Some(c) = update_node(&mut map, y + 1, x, height, distance) {
                candidates.push(c);
            }
        }
    }
    get_end_score(&map)
}

fn run_2(contents: String) -> usize {
    let mut map = parse_map(contents);
    let mut candidates = get_start_candidates(&map);
    while !candidates.is_empty() {
        let (y, x) = candidates.remove(0);
        let distance = &map[y][x].distance + 1;
        let height = &map[y][x].height + 2;
        if x != 0 {
            if let Some(c) = update_node(&mut map, y, x - 1, height, distance) {
                candidates.push(c);
            }
        }
        if y != 0 {
            if let Some(c) = update_node(&mut map, y - 1, x, height, distance) {
                candidates.push(c);
            }
        }
        if x != (map[0].len() - 1) {
            if let Some(c) = update_node(&mut map, y, x + 1, height, distance) {
                candidates.push(c);
            }
        }
        if y != (map.len() - 1) {
            if let Some(c) = update_node(&mut map, y + 1, x, height, distance) {
                candidates.push(c);
            }
        }
    }
    get_end_score(&map)
}

#[cfg(test)]
mod tests {

    const TEST_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    use super::*;
    #[test]
    fn test_day12_1() {
        assert_eq!(run_1(TEST_INPUT.to_string()), 31);
    }
    #[test]
    fn test_day12_2() {
        assert_eq!(run_2(TEST_INPUT.to_string()), 29);
    }
}

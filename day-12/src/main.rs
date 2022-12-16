use node::{get_start, get_start_candidates, parse_map, Node};
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

fn find_route(mut map: Vec<Vec<Node>>, sp: (usize, usize)) -> usize {
    let mut candidates = vec![sp];
    while !candidates.is_empty() {
        let (y, x) = candidates.remove(0);
        if map[y][x].is_goal() {
            return map[y][x].distance;
        }
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
    0
}

fn run_1(contents: String) -> usize {
    let map = parse_map(contents);
    let start = get_start(&map);
    find_route(map, start)
}

fn run_2(contents: String) -> usize {
    let map = parse_map(contents.clone());
    let start_points = get_start_candidates(&map);
    let mut score = 999999;
    for st in start_points {
        let new_score = find_route(parse_map(contents.clone()), st);
        if new_score < score && new_score != 0 {
            score = new_score
        }
    }
    score
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

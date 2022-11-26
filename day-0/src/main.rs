use std::fs;
fn main() {
    let path = "./input/hello.txt".to_owned();
    let contents = fs::read_to_string(&path).expect(&("Can't read filed:".to_owned() + &path));
    println!("{}", contents)
}

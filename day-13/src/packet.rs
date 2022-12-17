pub struct Packet {
    items: Vec<PacketItem>,
}

pub enum PacketItem {
    End,
    Value(usize),
    Done,
}

impl Packet {
    fn new(input: &str) -> Self {
        let mut items: Vec<PacketItem> = Vec::new();
        let mut working = "".to_string();
        for letter in input.chars() {
            match letter {
                ']' => {
                    if !working.is_empty() {
                        items.push(PacketItem::Value(working.parse().unwrap()));
                        working = "".to_string();
                    };
                    items.push(PacketItem::End)
                }
                ',' => {
                    if !working.is_empty() {
                        items.push(PacketItem::Value(working.parse().unwrap()));

                        working = "".to_string();
                    };
                }
                '[' => {}
                l => working.push(l),
            }
        }
        items.push(PacketItem::Done);

        items.reverse();
        Packet { items }
    }

    fn pop(&mut self) -> PacketItem {
        self.items.pop().unwrap()
    }
}

pub fn compare_order(left: &str, right: &str) -> bool {
    let mut left = Packet::new(left);
    let mut right = Packet::new(right);

    loop {
        match (left.pop(), right.pop()) {
            (PacketItem::Value(l), PacketItem::Value(r)) if l < r => return true,
            (PacketItem::Value(l), PacketItem::Value(r)) if l > r => return false,
            (PacketItem::End, PacketItem::Value(_)) => return true,
            (PacketItem::Value(_), PacketItem::End) => return false,
            (PacketItem::Done, _) => return true,
            (_, PacketItem::Done) => return false,
            _ => {}
        }
    }
}
#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_day13_1_1() {
        assert!(compare_order("[1,1,3,1,1]", "[1,1,5,1,1]"));
    }
    #[test]
    fn test_day13_1_2() {
        assert!(compare_order("[[1],[2,3,4]]", "[[1],4]"));
    }
    #[test]
    fn test_day13_1_3() {
        assert!(!compare_order("[9]", "[[8,7,6]]"));
    }
    #[test]
    fn test_day13_1_4() {
        assert!(compare_order("[[4,4],4,4]", "[[4,4],4,4,4]"));
    }
    #[test]
    fn test_day13_1_5() {
        assert!(!compare_order("[7,7,7,7]", "[7,7,7]"));
    }
    #[test]
    fn test_day13_1_6() {
        assert!(compare_order("[]", "[3]"));
    }
    #[test]
    fn test_day13_1_7() {
        assert!(!compare_order("[[[]]]", "[[]]"));
    }
    #[test]
    fn test_day13_1_8() {
        assert!(!compare_order(
            "[1,[2,[3,[4,[5,6,7]]]],8,9]",
            "1,[2,[3,[4,[5,6,0]]]],8,9]"
        ));
    }
}

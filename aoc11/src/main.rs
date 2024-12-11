fn main() {
    let contents = include_str!("../input");
    let mut stones: Vec<i64> = contents
        .trim()
        .split(" ")
        .map(|s| s.parse().expect("not a number"))
        .collect();
    println!("No blinks: {} stones", stones.len());
    for i in 0..25 {
        stones = stones.into_iter()
            .fold(vec![], |mut next, stone| {    
                if stone == 0 {
                    next.push(1);
                    return next;
                }
                let stone_str = stone.to_string();
                if stone_str.len() % 2 == 0 {
                    let mid = stone_str.len() / 2;
                    next.push(stone_str[..mid].parse().expect("not a number"));
                    next.push(stone_str[mid..].parse().expect("not a number"));
                } else {
                    next.push(stone * 2024);
                }
                next
            });
        println!("Blink {}, there are {} stones", i + 1, stones.len());
    }
}

use std::collections::HashMap;

fn main() {
    let contents = include_str!("../input");
    let mut stones: HashMap<_, _> = contents
        .trim()
        .split(" ")
        .map(|s| (s.parse::<i64>().expect("not a number"), 1_u128))
        .collect();
    println!("No blinks: {} stones", stones.values().sum::<u128>());
    for i in 0..75 {
        let mut clone = HashMap::new();
        for (stone, count) in &stones {
            if *stone == 0 {
                clone
                    .entry(1)
                    .and_modify(|e| *e += *count)
                    .or_insert(*count);
                continue;
            }
            let stone_str = stone.to_string();
            if stone_str.len() % 2 == 0 {
                let mid = stone_str.len() / 2;
                let left = stone_str[..mid].parse().expect("not a number");
                let right = stone_str[mid..].parse().expect("not a number");
                clone
                    .entry(left)
                    .and_modify(|e| *e += *count)
                    .or_insert(*count);
                clone
                    .entry(right)
                    .and_modify(|e| *e += *count)
                    .or_insert(*count);
            } else {
                clone
                    .entry(*stone * 2024)
                    .and_modify(|e| *e += *count)
                    .or_insert(*count);
            }
        }
        stones = clone;
        println!(
            "Blink {}, there are {} stones",
            i + 1,
            stones.values().sum::<u128>()
        );
    }
}

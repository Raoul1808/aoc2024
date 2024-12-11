fn main() {
    let contents = include_str!("../input");
    let stones: Vec<i32> = contents
        .trim()
        .split(" ")
        .map(|s| s.parse().expect("not a number"))
        .collect();
    println!("Stones: {:?}", stones);
}

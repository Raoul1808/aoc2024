use vec2::Vec2;

mod vec2;

#[derive(Debug)]
struct ClawMachine {
    button_a: Vec2<u32>,
    button_b: Vec2<u32>,
    prize: Vec2<u32>,
}

fn extract_numbers(line: &str, prefix: &str) -> (u32, u32) {
    let result = line.strip_prefix(prefix).expect("invalid input");
    let result: Vec<_> = result.split(", ").collect();
    assert_eq!(result.len(), 2, "found invalid line");
    let x = result[0][2..].parse().expect("invalid input");
    let y = result[1][2..].parse().expect("invalid input");
    (x, y)
}

fn process_claw(claw: ClawMachine) -> u32 {
    let mut press_a = 0;
    let mut press_b;
    'button_a: loop {
        let mut pos = claw.button_a * press_a;
        press_b = 0;
        loop {
            if pos == claw.prize {
                break 'button_a;
            }
            if pos > claw.prize {
                if press_b == 0 {
                    println!("Unsolvable");
                    return 0;
                }
                break;
            }
            pos += claw.button_b;
            press_b += 1;
            if pos == claw.prize {
                break 'button_a;
            }
        }
        press_a += 1;
    }
    let tokens = press_a * 3 + press_b;
    println!("Tokens needed: {}", tokens);
    tokens
}

fn main() {
    let input = include_str!("../input");
    let clean_input: Vec<_> = input.lines().filter(|s| !s.trim().is_empty()).collect();
    let mut claws = vec![];
    for chunk in clean_input.chunks(3) {
        assert_eq!(chunk.len(), 3, "found incomplete chunk");
        let button_a = extract_numbers(chunk[0], "Button A: ").into();
        let button_b = extract_numbers(chunk[1], "Button B: ").into();
        let prize = extract_numbers(chunk[2], "Prize: ").into();
        claws.push(ClawMachine {
            button_a,
            button_b,
            prize,
        });
    }
    let total = claws.len();
    let tokens: u32 = claws
        .into_iter()
        .enumerate()
        .map(|(i, claw)| {
            println!("Calculating claw machine {}/{}", i + 1, total);
            process_claw(claw)
        })
        .sum();
    println!("Total tokens needed: {}", tokens);
}

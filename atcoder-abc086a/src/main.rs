/*
 * https://atcoder.jp/contests/abs/tasks/abc086_a
 */
use std::io::Read;

fn main() {
    let mut buffer = String::new();
    match std::io::stdin().read_to_string(&mut buffer) {
        Ok(_) => {}
        Err(msg) => {
            eprintln!("{:?}", msg);
            std::process::exit(1);
        }
    }
    let mut lines = buffer.lines();
    let first_line = lines.next().unwrap();
    let values: Vec<&str> = first_line.split(' ').collect();
    /* 1.42 (current available version in AtCoder) does not have reduce()
    let is_even: bool = values
        .iter()
        .map(|s| s.parse::<i32>().unwrap() % 2 == 0)
        .reduce(|a, b| a || b)
        .unwrap();*/
    let mut is_even = false;
    for b in values.iter().map(|s| s.parse::<i32>().unwrap() % 2 == 0) {
        if b {
            is_even = true;
            break;
        }
    }
    print!(
        "{}",
        match is_even {
            true => "Even",
            _ => "Odd",
        }
    );
}

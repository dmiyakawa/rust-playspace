/*
 * https://atcoder.jp/contests/abs/tasks/abc081_a
 */

use std::io::Read;

fn read_from_stdin() -> String {
    let mut buffer = String::new();
    match std::io::stdin().read_to_string(&mut buffer) {
        Ok(_) => {}
        Err(msg) => {
            eprintln!("{:?}", msg);
            std::process::exit(1);
        }
    }
    buffer
}

fn main() {
    let buffer = read_from_stdin();
    let mut lines = buffer.lines();
    let first_line = lines.next().unwrap();
    let mut count = 0;
    for ch in first_line.chars() {
        if ch == '1' {
            count += 1;
        }
    }
    print!("{}", count);
}

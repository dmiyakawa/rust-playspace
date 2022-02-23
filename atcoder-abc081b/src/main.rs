/*
 * https://atcoder.jp/contests/abs/tasks/abc081_b
 */

use std::io::Read;

fn read_from_stdin() -> Vec<String> {
    let mut buffer = String::new();
    match std::io::stdin().read_to_string(&mut buffer) {
        Ok(_) => {}
        Err(msg) => {
            eprintln!("{:?}", msg);
            std::process::exit(1);
        }
    }
    let mut vec: Vec<String> = Vec::new();
    for s in buffer.lines() {
        vec.push(String::from(s));
    }
    vec
}

fn main() {
    let lines = read_from_stdin();
    let n = lines[0].parse::<i32>().unwrap();
    let a_s: Vec<i32> = lines[1]
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    assert_eq!(n as usize, a_s.len());
    let min_ops = a_s
        .iter()
        .map(|a| {
            let mut ops = 0;
            let mut current: i32 = *a;
            while current % 2 == 0 {
                current = current / 2;
                ops += 1;
            }
            ops
        })
        .min()
        .unwrap();
    println!("{:?}", min_ops);
}

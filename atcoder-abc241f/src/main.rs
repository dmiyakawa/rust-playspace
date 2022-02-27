/*
 * https://atcoder.jp/contests/abc241/tasks/abc241_f
 *
 * Note: this does NOT work.
 */

use std::{
    collections::{HashMap, HashSet},
    io::Read
};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

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

fn line_to_nums(line: &str) -> Vec<i32> {
    let nums: Vec<i32> = line
        .split(' ')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    nums
}

//
// pがstart地点もしくはその地点のy軸上の「上下」にある点に障害物があれば、開始点となりえる
//
fn consider_with_x_axis(p: &Point, start: &Point, obstacles: &HashSet<Point>, reachables: &mut HashMap<Point, i32>, places_to_consider: &mut Vec<Point>, num_moves: &i32) {
    if p == start || obstacles.contains(&Point {x: p.x, y: p.y + 1}) || obstacles.contains(&Point {x: p.x, y: p.y - 1}) {
        let current_num_moves = match reachables.get(&p) {
            Some(v) => *v,
            _ => 1000000000i32,
        };
        // println!("  ({}, {}) <- considering in x. num_moves: {}, current: {}", p.x, p.y, num_moves, current_num_moves);
        if num_moves + 1 < current_num_moves {
            // println!("    inserting ({}, {}) with num_moves: {}", p.x, p.y, num_moves + 1);
            reachables.insert(*p, num_moves + 1);
            places_to_consider.push(*p);
        }
    }
}

//
// pがstart地点もしくはその地点のx軸上の「左右」にある点に障害物があれば、開始点となりえる
//
fn consider_with_y_axis(p: &Point, start: &Point, obstacles: &HashSet<Point>, reachables: &mut HashMap<Point, i32>, places_to_consider: &mut Vec<Point>, num_moves: &i32) {
    if p == start || obstacles.contains(&Point {x: p.x + 1, y: p.y}) || obstacles.contains(&Point {x: p.x - 1, y: p.y}) {
        let current_num_moves = match reachables.get(&p) {
            Some(v) => *v,
            _ => 1000000000i32,
        };
        // println!("  ({}, {}) <- considering in y. num_moves: {}, current: {}", p.x, p.y, num_moves, current_num_moves);
        if num_moves + 1 < current_num_moves {
            // println!("    inserting ({}, {}) with num_moves: {}", p.x, p.y, num_moves + 1);
            reachables.insert(*p, num_moves + 1);
            places_to_consider.push(*p);
        }
    }
}

fn main() {
    let lines = read_from_stdin();
    let mut nums = line_to_nums(&lines[0]);
    let x_max = nums[0] as i32;
    let y_max = nums[1];
    let n = nums[2];

    nums = line_to_nums(&lines[1]);
    let start = Point {
        x: nums[0] - 1 as i32,
        y: nums[1] - 1 as i32,
    };

    nums = line_to_nums(&lines[2]);
    let g = Point {
        x: nums[0] - 1 as i32,
        y: nums[1] - 1 as i32,
    };

    let mut obstacles: HashSet<Point> = HashSet::new();
    let mut x_to_ys: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut y_to_xs: HashMap<i32, HashSet<i32>> = HashMap::new();
    for i in 1..n {
        nums = line_to_nums(&lines[2 + i as usize]);
        let x = nums[0] - 1;
        let y = nums[1] - 1;
        obstacles.insert(Point {x: x, y: y});
        x_to_ys.entry(x).or_insert(HashSet::new()).insert(y);
        y_to_xs.entry(y).or_insert(HashSet::new()).insert(x);
    }

    // println!("# H(x_max): {}, W(y_max): {}, N: {}", x_max, y_max, n);
    // println!("# S: ({}, {})", start.x, start.y);
    // println!("# G: ({}, {})", g.x, g.y);
    // println!("# x_to_ys: {:?}", x_to_ys);
    // println!("# y_to_xs: {:?}", y_to_xs);

    let mut reachables: HashMap<Point, i32> = HashMap::new();
    reachables.insert(g, 0);

    let mut places_to_consider: Vec<Point> = Vec::new();
    places_to_consider.push(g);
    loop {
        match places_to_consider.pop() {
            Some(p) => {
                let num_moves = *reachables.get(&p).unwrap();

                // println!("({}, {}) -> {}", p.x, p.y, num_moves);

                if obstacles.contains(&Point {x: p.x + 1, y: p.y}) {
                    for i in 1..=p.x {
                        let cp = Point {x: p.x - i, y: p.y};
                        if obstacles.contains(&cp) {
                            break;
                        }
                        // println!("  ({}, {})", cp.x, cp.y);
                        consider_with_x_axis(&cp, &start, &obstacles, &mut reachables, &mut places_to_consider, &num_moves);
                    }
                }
                if obstacles.contains(&Point {x: p.x - 1, y: p.y}) {
                    for i in  1..x_max-p.x  {
                        let cp = Point {x: p.x + i, y: p.y};
                        if obstacles.contains(&cp) {
                            break;
                        }
                        // println!("  ({}, {})!", cp.x, cp.y);
                        consider_with_x_axis(&cp, &start, &obstacles, &mut reachables, &mut places_to_consider, &num_moves);
                    }
                }
                if obstacles.contains(&Point {x: p.x, y: p.y + 1}) {
                    for i in 1..=p.y {
                        let cp = Point {x: p.x, y: p.y - i};
                        if obstacles.contains(&cp) {
                            break;
                        }
                        // println!("  ({}, {})", cp.x, cp.y);
                        consider_with_y_axis(&cp, &start, &obstacles, &mut reachables, &mut places_to_consider, &num_moves);
                    }

                }
                if obstacles.contains(&Point {x: p.x, y: p.y - 1}) {
                    for i in p.y + 1..y_max-p.y {
                        let cp = Point {x: p.x, y: p.y + i};
                        if obstacles.contains(&cp) {
                            break;
                        }
                        // println!("  ({}, {})", cp.x, cp.y);
                        consider_with_y_axis(&cp, &start, &obstacles, &mut reachables, &mut places_to_consider, &num_moves);
                    }
                }
            },
            _ => {
                break;
            }
        };
    }
    let val = match reachables.get(&start) {
        Some(v) => *v,
        _ => -1,
    };
    print!("{}", val);
}

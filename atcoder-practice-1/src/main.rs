use std::io::Read;
use std::process::exit;

fn main() {
    /*
    if args.len() < 2 {
        eprintln!("Supply an argument for a input file (e.g. inputs/1.txt)");
        std::process::exit(1);
    }
    let filename = &args[1];

    let f = File::open(filename).expect("file not found");

    let mut lines = io::BufReader::new(f).lines();
    let first_line = lines.next().unwrap().unwrap();
    let second_line = lines.next().unwrap().unwrap();
    let third_line = lines.next().unwrap().unwrap();
    */
    let mut buffer = String::new();
    match std::io::stdin().read_to_string(&mut buffer) {
        Ok(_) => {}
        Err(msg) => {
            eprintln!("{:?}", msg);
            exit(1);
        }
    }
    let mut lines = buffer.lines();
    let first_line = lines.next().unwrap();
    let second_line = lines.next().unwrap();
    let third_line = lines.next().unwrap();
    let a = first_line.parse::<i32>().unwrap();
    let vals: Vec<&str> = second_line.split(' ').collect();
    let nums: Vec<i32> = vals.iter().map(|s| s.parse::<i32>().unwrap()).collect();
    let (b, c) = (nums[0], nums[1]);
    let s = third_line;
    // println!("a: {:?}, b: {:?}, c: {:?}, s: {:?}", a, b, c, s);
    println!("{} {}", a + b + c, s);
}

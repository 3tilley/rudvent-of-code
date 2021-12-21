use crate::utils;

enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

fn split_str(str: &str) -> Direction {
    let s: Vec<_> = str.split_whitespace().collect();
    if s.len() != 2 {
        panic!("Not two values: {:?}", s)
    }
    let amount: i32 = s[1].parse::<i32>().unwrap();
    match s[0] {
        "forward" => Direction::Forward(amount),
        "down" => Direction::Down(amount),
        "up" => Direction::Up(amount),
        _ => panic!("Unrecognised direction"),
    }
}

pub fn a() -> i32 {
    let contents = utils::read_file("input.txt", file!());
    let str_lines = contents.lines();
    let directions : Vec<_> = str_lines.map(split_str).collect();
    let mut horizontal : i32 = 0;
    let mut vertical : i32 = 0;

    for d in directions {
        match d {
            Direction::Down(a) => vertical += a,
            Direction::Up(a) => vertical -= a,
            Direction::Forward(a) => horizontal += a,
        }
    }
    let ans = horizontal * vertical;

    ans
}

pub fn b() -> i32 {
    let contents = utils::read_file("input.txt", file!());
    let str_lines = contents.lines();
    let directions : Vec<_> = str_lines.map(split_str).collect();
    let mut horizontal : i32 = 0;
    let mut vertical : i32 = 0;
    let mut aim : i32 = 0;

    for d in directions {
        match d {
            Direction::Down(a) => aim += a,
            Direction::Up(a) => aim -= a,
            Direction::Forward(a) => {
                horizontal += a;
                vertical += aim * a;
            },
        }
    }
    let ans = horizontal * vertical;

    ans
}

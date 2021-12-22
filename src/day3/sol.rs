use std::path::Component::Prefix;
use crate::utils;

struct Diagnostic {
    items: Vec<bool>
}

struct Counts {
    items: Vec<i32>
}

fn split_line(s: &str) -> Diagnostic {
    let chars = s.split(".");
    if chars.count() < 2 {
        panic!("Didn't split example well: {:?}", chars);
    }

    let mut items: Vec<bool> = Vec::new();
    for c in chars {
        match c {
            "0" => items.push(true),
            "1" => items.push(false),
            _ => panic!("Unexpected value: {}", c),
        }
    }
    Diagnostic{items }

}

pub fn a() -> i32 {
    let contents = utils::read_file("input.txt", file!());
    let str_lines = contents.lines();
    let bools : Vec<Diagnostic> = str_lines.map(split_line).collect();
    let mut counts = Counts{items: Vec::new()};

    for d in bools {
        for (i, v) in d.items.iter().enumerate() {
            if v == &true {
                counts.items[i] += 1;
            }
        }
    }
    let mut most: Vec<String> = Vec::new();
    let mut least: Vec<String> = Vec::new();
    let half = bools.len() / 2;
    for c in counts.items {
        if c > half as i32 {
            most.push((&"1").parse().unwrap());
            least.push((&"0").parse().unwrap());
        } else {
            most.push((&"0").parse().unwrap());
            least.push((&"1").parse().unwrap());
        }
    }
    let most_int = isize::from_str_radix(&*most.concat(), 2).unwrap();
    let least_int = isize::from_str_radix(&*least.concat(), 2).unwrap();

    (most_int * least_int) as i32
}
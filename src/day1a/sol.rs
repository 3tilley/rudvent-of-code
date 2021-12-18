use std::path::Path;
use std::fs;

pub fn ans() {
    let this_file = Path::new(file!()).parent().unwrap().join("input.txt");
    println!("Trying to read from: {}", this_file.display());
    let contents = fs::read_to_string(&this_file).expect("Unable to load file");
    let str_lines = contents.lines();
    let ints = str_lines.map(|s| s.parse::<i32>().unwrap());
    let mut counter = 0;
    let mut last_depth = None;
    for depth in ints {
        if last_depth != None && depth > last_depth.unwrap() {
            counter += 1;
        }
        last_depth = Some(depth);
    }

    println!("{}", counter);
}
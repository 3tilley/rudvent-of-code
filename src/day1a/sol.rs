use crate::utils;

pub fn ans() {
    let contents = utils::read_file("input.txt", file!());
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
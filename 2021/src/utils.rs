use std::path::Path;
use std::{fs, io};
use std::convert::TryInto;

pub fn read_file(name: &str, relative_to: &str) -> String {
    let path = Path::new(relative_to);
    let mut relative = path;
    if path.is_file() {
        relative = path.parent().unwrap();
    }

    let this_file = relative.join(name);
    println!("Trying to read from: {}", this_file.display());
    let contents = fs::read_to_string(&this_file).expect("Unable to load file");
    contents
}

pub fn ask_bool_input() -> bool {
    let mut answer = String::new();
    let yeses = vec!["yes".to_string(), "y".to_string()];
    println!("Copy answer to clipboard? [yN]: ");
    io::stdin().read_line(&mut answer);
    println!("{}", answer);
    yeses.contains(&answer.trim().to_lowercase())
}

pub trait Solution<T, U, V> {
    fn a(example: bool) -> T {
        todo!()
    }
    fn b(example: bool) -> T {
        todo!()
    }
    fn prepare_a(example: bool) -> U {
        todo!()
    }
    fn prepare_b(example: bool) -> U {
        todo!()
    }
    fn inner_a(prep: U) -> V {
        todo!()
    }
    fn inner_b(prep: U) -> V {
        todo!()
    }
    fn output_a(answer: V) -> T {
        todo!()
    }
    fn output_b(answer: V) -> T {
        todo!()
    }
}
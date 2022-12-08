use std::path::{Path, PathBuf};
use std::{fs, io};
use std::convert::TryInto;
use std::fmt::Debug;

pub struct DayData {
    day: u8,
}

impl DayData {
    pub fn new(day: u8) -> Self {
        Self { day }
    }

    pub fn example_1_path(&self) -> PathBuf {
        let path = PathBuf::from(file!());
        let mut data_path = path.parent().unwrap().parent().unwrap().to_path_buf();
        data_path.push("data");
        data_path.push(format!("day{}_example_1.txt", self.day));
        data_path
    }


}

pub fn read_file_from_data(name: &str, relative_to: &str) -> String {
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

pub fn ask_bool_input(msg: &str, default: bool) -> bool {
    let mut answer = String::new();
    let yeses = vec!["yes".to_string(), "y".to_string()];
    let noes = vec!["no".to_string(), "n".to_string()];
    if default {
        println!("{} [Y/n]", msg);
    } else {
        println!("{} [y/N]", msg);
    }
    // io::stdin().read_line(&mut answer);
    println!("{}", answer);
    let answer = answer.trim().to_lowercase();
    if yeses.contains(&answer) {
        true
    } else if noes.contains(&answer) {
        false
    } else {
        default
    }
}

pub fn ask_index_input<T: Debug>(msg: &str, items: &Vec<T>, max_attempts: u32, current_attempt: u32) -> usize {

    let mut answer = String::new();
    println!("Choose from the following options:\n");
    for (i, item) in items.iter().enumerate() {
        println!("{}:\n {:?}\n", i, item);
    }
    io::stdin().read_line(&mut answer);
    usize::from_str_radix(&answer.trim(), 10).unwrap_or_else(|_| {
        if current_attempt < max_attempts {
            println!("Invalid input, please try again");
            ask_index_input(msg, items, max_attempts, current_attempt + 1)
        } else {
            println!("Too many attempts, exiting");
            std::process::exit(1);
        }
    })
}

// pub trait Solution<T, U, V> {
//     fn a(example: bool) -> T {
//         todo!()
//     }
//     fn b(example: bool) -> T {
//         todo!()
//     }
//     fn prepare_a(example: bool) -> U {
//         todo!()
//     }
//     fn prepare_b(example: bool) -> U {
//         todo!()
//     }
//     fn inner_a(prep: U) -> V {
//         todo!()
//     }
//     fn inner_b(prep: U) -> V {
//         todo!()
//     }
//     fn output_a(answer: V) -> T {
//         todo!()
//     }
//     fn output_b(answer: V) -> T {
//         todo!()
//     }
// }
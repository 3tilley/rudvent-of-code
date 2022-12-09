use std::path::{Path, PathBuf};
use std::{fs, io};
use std::convert::TryInto;
use std::fmt::Debug;
use cached_path::{Cache, CacheBuilder};
use color_eyre::eyre::{WrapErr, Result, eyre};
use scraper::{ElementRef, Html, Selector};

// const url template
const DAY_TEMPLATE: &str = "https://adventofcode.com/2022/day/{day}";

fn day_url(day: u8) -> String {
    DAY_TEMPLATE.replace("{day}", &day.to_string())
}

pub struct DayData {
    day: u8,
    data_dir: PathBuf,
    cache: Cache,
    dry_run: bool,
}

impl DayData {
    pub fn new(day: u8, dry_run: bool) -> Self {
        let path = PathBuf::from(file!());
        let mut data_path = path.parent().unwrap().parent().unwrap().to_path_buf();
        data_path.push("data");
        data_path.push("html");
        // It's probably a bit abusive, but I never want to hit the server if I can avoid it
        let ten_years_in_seconds = 10 * 365 * 24 * 60 * 60;
        let cache = CacheBuilder::new()
            .dir(data_path.clone())
            .freshness_lifetime(ten_years_in_seconds)
            .build()
            .unwrap();
        Self { day, data_dir: data_path, cache, dry_run }
    }

    pub fn example_1_path(&self) -> PathBuf {
        let path = PathBuf::from(file!());
        let mut data_path = path.parent().unwrap().parent().unwrap().to_path_buf();
        data_path.push("data");
        data_path.push(format!("day{}_example_1.txt", self.day));
        data_path
    }

    pub fn input_1_path(&self) -> PathBuf {
        let path = PathBuf::from(file!());
        let mut data_path = path.parent().unwrap().parent().unwrap().to_path_buf();
        data_path.push("data");
        data_path.push(format!("day{}_input_1.txt", self.day));
        data_path
    }

    pub fn html_for_day(&self) -> String {
        let path = self.cache.cached_path(&*day_url(self.day)).unwrap();
        let data = read_as_string(&path).unwrap();
        data
    }

    pub fn fetch_day_example(&self) {
        let html = self.html_for_day();
        let doc = Html::parse_document(&html);
        let pre_selector = Selector::parse("pre code").unwrap();
        let pres = doc.select(&pre_selector).collect::<Vec<_>>();
        println!("\n{} pre tags\n", pres.len());
        match pres.len() {
            0 => println!("No obvious example blocks found"),
            1 => {
                let pre = pres.get(0).unwrap();
                if quiz_to_save(pre) {
                    save_example(self.example_1_path(), &pre.inner_html(), self.dry_run);
                }
            }
            x => {
                println!("Found {} potential example blocks, please select one:", x);
                let index = ask_index_input("Enter a digit to choose", &pres, 3, 0);
                save_example(self.example_1_path(), &pres.get(index).unwrap().inner_html(), self.dry_run);
            }
        }
    }

    pub fn fetch_data(&self) -> Result<()> {
        if self.example_1_path().exists() {
            println!("Example file already exists, skipping");
        } else {
            println!("Fetching example data");
            self.fetch_day_example();
            println!("Saved");
        }

        if self.input_1_path().exists() {
            println!("Input file already exists, skipping");
        } else {
            println!("Fetching input data");
            let text = reqwest::blocking::get(format!("{}/input", day_url(self.day))).unwrap().text()?;
            if text.contains("Puzzle inputs differ by user") {
                Err(eyre!("Need to provide authentication to fetch puzzle data"))
            }
            save_example(self.input_1_path(), &text, self.dry_run)?;
            println!("Saved");
        }
         Ok(())
    }

    pub fn example_1(&self) -> String {
        let path = self.example_1_path();
        let data = read_as_string(&path).unwrap();
        data.trim().to_string()
    }

    pub fn input_1(&self) -> String {
        let path = self.input_1_path();
        let data = read_as_string(&path).unwrap();
        data.trim().to_string()
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
    let data = read_as_string(path).unwrap();
    data
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
    io::stdin().read_line(&mut answer);
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

pub fn read_as_string(path: &Path) -> Result<String> {
    fs::read_to_string(&path)
        .wrap_err_with(|| format!("Failed to read data from {}", &path.display()))
}

fn quiz_to_save(pre: &ElementRef) -> bool {
    let found_example = pre.inner_html();
    println!("\nFound example:\n{}", found_example);
    ask_bool_input("Save this example?", true)
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

fn save_example(path: PathBuf, content: &str, dry_run: bool) -> Result<()> {
    // Just printing for now
    let msg = format!("Saving example at {}:\n{}", &path.display(), content);
    if dry_run {
        println!("Dry-run enabled, but would be {}", msg);
        Ok(())
    } else {
        log::info!("{}", msg);
        fs::write(&path, content)
            .wrap_err_with(|| format!("Failed to write data to {}", &path.display()))
    }
}

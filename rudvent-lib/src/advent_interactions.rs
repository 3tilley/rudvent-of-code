use cached_path::{Cache, CacheBuilder};
use color_eyre::eyre::{eyre, Result, WrapErr};
use color_eyre::Report;
use dialoguer::theme::ColorfulTheme;
use dialoguer::Confirm;
use reqwest::blocking::Client;
use scraper::node::Element;
use scraper::{Element as OtherElement, ElementRef, Html, Selector};
use std::convert::TryInto;
use std::fmt::{format, Debug};
use std::hash::BuildHasherDefault;
use std::path::{Path, PathBuf};
use std::{env, fs, io};
use std::str::FromStr;

use tracing::{debug, info, trace, warn};
use crate::advent_interactions::PostError::UnknownError;

// const url template
const DAY_TEMPLATE: &str = "https://adventofcode.com/{year}/day/{day}";

fn day_url(year: u16, day: u8) -> String {
    DAY_TEMPLATE
        .replace("{day}", &day.to_string())
        .replace("{year}", &year.to_string())
}

#[derive(Debug, Clone)]
pub enum PostError {
    TooLow,
    TooHigh,
    // Wait time is in minutes
    TooManyAttempts(usize),
    UnknownError(String),
}

pub struct DayData {
    year: u16,
    day: u8,
    data_dir: PathBuf,
    cache: Cache,
    client: Client,
    dry_run: bool,
    auth_token: String,
}

impl DayData {
    pub fn new(
        year: u16,
        day: u8,
        dry_run: bool,
        data_directory: PathBuf,
        auth_token: String,
    ) -> Self {
        // It's probably a bit abusive, but I never want to hit the server if I can avoid it
        let ten_years_in_seconds = 10 * 365 * 24 * 60 * 60;
        let client = DayData::make_client_builder(&auth_token).build().unwrap();
        let cache = CacheBuilder::new()
            .client_builder(DayData::make_client_builder(&auth_token))
            .dir(data_directory.join("html"))
            .freshness_lifetime(ten_years_in_seconds)
            .build()
            .unwrap();
        Self {
            year,
            day,
            data_dir: data_directory,
            cache,
            dry_run,
            auth_token,
            client,
        }
    }

    fn make_client_builder(auth_token: &str) -> reqwest::blocking::ClientBuilder {
        reqwest::blocking::ClientBuilder::new().default_headers({
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert(
                reqwest::header::COOKIE,
                format!("session={}", auth_token.to_string())
                    .parse()
                    .unwrap(),
            );
            headers
        })
    }

    pub fn example_1_path(&self) -> PathBuf {
        self.data_dir.join(format!("day{}_example_1.txt", self.day))
    }

    pub fn example_2_path(&self) -> PathBuf {
        // let ex_1 = self.data_dir.join(format!("day{}_example_1.txt", self.day));
        let ex_2 = self.data_dir.join(format!("day{}_example_2.txt", self.day));
        // if ex_2.exists() {
        //     ex_2
        // } else {
        //     ex_1
        // }
        ex_2
    }

    pub fn input_1_path(&self) -> PathBuf {
        self.data_dir.join(format!("day{}_input_1.txt", self.day))
    }

    pub fn html(&self, part_1: bool, all_html: bool, force_refetch: bool) -> Result<String> {
        let suffix = if part_1 { "1" } else { "2" };
        let path = self
            .data_dir
            .join(format!("day{}_{}.html", self.day, suffix));
        let text = if path.exists() && !force_refetch {
            info!("Loading HTML from {}", path.to_string_lossy());
            read_as_string(&path).unwrap()
        } else {
            info!("HTML not in cache, fetching");
            let url = day_url(self.year, self.day);
            let resp = self.client.get(&url).send()?;
            let text = match resp.status() {
                reqwest::StatusCode::OK => resp.text()?,
                e => {
                    warn!("Error fetching HTML: {:?}", e);
                    return Err(eyre!(
                        "Error fetching HTML: {:?}. Is your token correct?",
                        e
                    ));
                }
            };
            if self.dry_run {
                info!(
                    "Dry-run enabled, but would be saving HTML to {}",
                    path.to_string_lossy()
                );
            } else {
                write_as_string(path, &text, false)?;
            }
            text
        };
        if all_html {
            return Ok(text);
        }
        let html = Html::parse_document(&text);
        let selector = Selector::parse("article.day-desc").unwrap();
        let mut matching = Vec::new();
        for element in html.select(&selector) {
            matching.push(element.inner_html());
        }
        info!("Found {} matching elements", matching.len());
        Ok(matching.join("\n"))
    }

    pub fn fetch_day_example(&self, part_1: bool) -> Result<()> {
        let html = self.html(part_1, false, false)?;
        let doc = Html::parse_document(&html);
        let pre_selector = Selector::parse("pre code").unwrap();
        let pres = doc.select(&pre_selector).collect::<Vec<_>>();
        println!("\n{} pre tags\n", pres.len());
        match pres.len() {
            0 => Err(eyre!("No obvious example blocks found")),
            1 => {
                let pre = pres.get(0).unwrap();
                if !self.dry_run && quiz_to_save(pre) {
                    write_as_string(self.example_1_path(), &pre.inner_html(), self.dry_run)?;
                }
                Ok(())
            }
            x => {
                println!("Found {} potential example blocks, please select one:", x);
                let options = pres.iter().map(|x| x.inner_html()).collect::<Vec<String>>();
                let index = ask_index_input("Enter a digit to choose", &options, 3, 0);
                write_as_string(
                    self.example_1_path(),
                    &pres.get(index).unwrap().inner_html(),
                    self.dry_run,
                )?;
                Ok(())
            }
        }
    }

    pub fn fetch_data(&self) -> Result<()> {
        if self.example_1_path().exists() {
            println!("Example file already exists, skipping");
        } else {
            println!("Fetching example data for part 1");
            self.fetch_day_example(true)?;
            println!("Saved");
        }

        if self.input_1_path().exists() {
            println!("Input file already exists, skipping");
        } else {
            println!("Fetching input data");
            let text = self
                .client
                .get(format!("{}/input", day_url(self.year, self.day)))
                .send()
                .unwrap()
                .text()?;
            if text.contains("Puzzle inputs differ by user") {
                return Err(eyre!("Need to provide authentication to fetch puzzle data"));
            }
            write_as_string(self.input_1_path(), &text, self.dry_run)?;
            println!("Saved");
        }
        Ok(())
    }

    pub fn example_1(&self) -> String {
        let path = self.example_1_path();
        let data = read_as_string(&path).unwrap();
        data.to_string()
    }

    pub fn example_2(&self) -> String {
        let path_2 = self.example_2_path();
        let path = if path_2.exists() {
            path_2
        } else {
            self.example_1_path()
        };
        let data = read_as_string(&path).unwrap();
        data.to_string()
    }

    pub fn input_1(&self) -> String {
        let path = self.input_1_path();
        let data = read_as_string(&path).unwrap();
        data.to_string()
    }

    pub fn input_2(&self) -> String {
        let path = self.input_1_path();
        let data = read_as_string(&path).unwrap();
        data.to_string()
    }

    pub fn check_for_posting(&self, part_1: bool) -> Result<Option<String>> {
        let document = &*self.html(part_1, true, false)?;
        let day = self.day;

        let result = Self::has_been_posted(part_1, document, day);
        match result {
            Ok(None) => {
                let fresh_document = &*self.html(part_1, true, true)?;
                Self::has_been_posted(part_1, fresh_document, day)
            }
            _ => result,
        }
    }

    fn has_been_posted(part_1: bool, document: &str, day: u8) -> Result<Option<String>, Report> {
        let selector = Selector::parse(&*format!(r#"form[action="{}/answer"]"#, day)).unwrap();
        let html = Html::parse_document(document);

        // We'll use the header to select the right article, and from there get to the <p> below that contains the previous answer
        let header_selector =
            Selector::parse(&*format!(r#"main article.day-desc"#)).unwrap();
        let articles = html.select(&header_selector).collect::<Vec<_>>();
        if articles.len() == 1 {
            info!("Only part 1 is available, that probably means the part 1 answer hasn't been posted");
            Ok(None)
        } else if articles.len() == 2 {
            let article_index = if part_1 {0} else {1};
            let puzzle_answer_p = articles[article_index].next_sibling_element().unwrap();
            if puzzle_answer_p
                .html()
                .contains("Your puzzle answer was")
            {
                let ans = puzzle_answer_p
                    .select(&Selector::parse("code").unwrap()).next()
                    .expect("This should be a <code> block, not found")
                    .inner_html();
                Ok(Some(ans))
            } else {
                Ok(None)
            }
        } else {
            Err(eyre!("Unable to find the answer on the page"))
        }
    }

    pub fn post_ans(&self, answer: &str, part_1: bool) -> Result<String> {
        let suffix = if part_1 { "1" } else { "2" };
        let url = format!("{}/answer", day_url(self.year, self.day));
        let resp = self
            .client
            .post(&url)
            .form(&[
                ("level", suffix.to_string()),
                ("answer", answer.to_string()),
            ])
            .send()?;
        let text = resp.text()?;
        println!("Posted answer to part {}: {:?}", suffix, answer);
        let html_file = self
            .data_dir
            .join(format!("day{}_{}_answer.html", self.day, suffix));
        write_as_string(html_file, &text, self.dry_run)?;
        let res = process_answer(text);
        match res {
            Ok(x) => Ok(x),
            Err(e) => Err(eyre!("Error processing answer: {:?}", e)),
        }
    }

    pub fn is_data_available(&self, part_1: bool) -> Result<()> {
        let is_available = {
            if part_1 {
                self.example_1_path().exists() && self.input_1_path().exists()
            } else {
                self.example_2_path().exists() && self.input_1_path().exists()
            }
        };
        if is_available {
            Ok(())
        } else {
            Err(eyre!("Data not available, please run 'fetch' first"))
        }
    }

    pub fn next_day(&self) -> DayData {
        DayData::new(
            self.year,
            self.day + 1,
            self.dry_run,
            self.data_dir.clone(),
            self.auth_token.clone(),
        )
    }
}

pub(crate) fn process_answer(post_result: String) -> std::result::Result<String, PostError> {
    let html = Html::parse_document(&post_result);
    println!("{}", html.html());
    let selector = Selector::parse("main article p").unwrap();
    let mut selection = html.select(&selector);
    let first_p = selection.next().unwrap();
    if first_p.inner_html().contains("That's the right answer!") {
        Ok("That's the right answer!".to_string())
    } else if first_p.inner_html().contains("your answer is too low") {
        Err(PostError::TooLow)
    } else if first_p.inner_html().contains("your answer is too high") {
        Err(PostError::TooHigh)
    } else if first_p.inner_html().contains("Because you have guess incorrectly"){
        // The wrong answer has been posted too many times.
        let wait_string = "on this puzzle, please wait ";
        let index_of_wait = first_p.inner_html().find(wait_string);
        let offset = index_of_wait.unwrap() + wait_string.len();
        let wait_block = &first_p.inner_html()[offset..offset+10];
        debug!("Wait_block: {}", wait_block);
        let mut words_iter = wait_block.split_whitespace();
        let wait_amount = words_iter.next().unwrap();
        info!("Wait_amount: {}", wait_amount);
        let wait_unit = words_iter.next().unwrap();
        info!("Wait_unit: {}", wait_amount);
        if wait_unit == "minutes" {
            Err(PostError::TooManyAttempts(usize::from_str(wait_amount).unwrap()))
        } else {
            Err(UnknownError(first_p.inner_html()))
        }
    } else {
        Err(UnknownError(first_p.inner_html()))
    }
}

pub(crate) fn read_file_from_data(name: &str, relative_to: &str) -> String {
    let path = Path::new(relative_to);
    let mut relative = path;
    if path.is_file() {
        relative = path.parent().unwrap();
    }

    let this_file = relative.join(name);
    println!("Trying to read from: {}", this_file.display());
    let data = read_as_string(&path.to_path_buf()).unwrap();
    data
}

pub(crate) fn ask_bool_input(msg: &str, default: bool) -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(msg)
        .report(true)
        .default(default)
        .interact()
        .unwrap()
}

pub(crate) fn ask_index_input<T: Debug>(
    msg: &str,
    items: &Vec<T>,
    max_attempts: u32,
    current_attempt: u32,
) -> usize {
    let mut answer = String::new();
    println!("Choose from the following options:\n");
    for (i, item) in items.into_iter().enumerate() {
        println!("{}:\n {:?}\n", i + 1, item);
    }
    io::stdin().read_line(&mut answer);
    match usize::from_str_radix(&answer.trim(), 10) {
        Ok(x) => {
            if x > 0 && x <= items.len() {
                x - 1
            } else {
                if current_attempt < max_attempts {
                    println!("Index out of range, try again");
                    ask_index_input(msg, items, max_attempts, current_attempt + 1)
                } else {
                    panic!("Too many invalid inputs");
                }
            }
        }
        Err(e) => {
            if current_attempt < max_attempts {
                println!("Invalid input, please try again");
                ask_index_input(msg, items, max_attempts, current_attempt + 1)
            } else {
                println!("Too many attempts, exiting");
                std::process::exit(1);
            }
        }
    }
}

pub(crate) fn read_as_string(path: &PathBuf) -> Result<String> {
    fs::read_to_string(&path)
        .wrap_err_with(|| format!("Failed to read data from {}", &path.display()))
}

fn quiz_to_save(pre: &ElementRef) -> bool {
    let found_example = pre.inner_html();
    println!("\nFound example:\n{}", found_example);
    ask_bool_input("Save this example?", true)
}

pub fn write_as_string(path: PathBuf, content: &str, dry_run: bool) -> Result<()> {
    let msg = format!("Saving data to {}:\n{}", &path.display(), content);
    if dry_run {
        println!("Dry-run enabled, but would be {}", msg);
        Ok(())
    } else {
        trace!("{}", msg);
        fs::write(&path, content)
            .wrap_err_with(|| format!("Failed to write data to {}", &path.display()))
    }
}

pub fn rename_file(path: PathBuf, new_name: PathBuf) -> Result<()> {
    fs::rename(&path, &new_name).wrap_err_with(|| {
        format!(
            "Failed to rename {} to {}",
            &path.display(),
            &new_name.display()
        )
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_answer() {
        let path = PathBuf::from(file!());
        let mut data_path = path.parent().unwrap().parent().unwrap().to_path_buf();
        data_path.push("examples");
        data_path.push("day2_2_answer.html");
        let test_data = read_as_string(&data_path).unwrap();
        process_answer(test_data);
    }

    #[test]
    fn test_check_forms() {
        let path = PathBuf::from(file!());
        let mut data_path = path.parent().unwrap().parent().unwrap().to_path_buf();
        data_path.push("examples");
        data_path.push("day4_1.html");
        let test_data = read_as_string(&data_path).unwrap();
        assert_eq!(
            DayData::has_been_posted(true, &*test_data, 4).unwrap(),
            false
        );
    }
}

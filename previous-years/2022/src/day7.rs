use crate::day7::Command::Cd;
use crate::solution::{Example, StructSolution};
use crate::DayData;
use reqwest::header::COOKIE;
use std::borrow::Borrow;
use std::cmp::min;
use std::collections::HashMap;
use std::iter::once;
use std::str::FromStr;

// type Input1<'a> = &'a DirectoryItem;
type Input1 = DirectoryItem;
type Output1 = usize;
// type Input2<'a> = &'a DirectoryItem;
type Input2 = DirectoryItem;
type Output2 = usize;

#[derive(Debug)]
pub enum DirectoryItem {
    File {
        name: String,
        size: usize,
    },
    Directory {
        name: String,
        items: Vec<DirectoryItem>,
    },
}

impl FromStr for DirectoryItem {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // println!("{}", s);
        match &s.chars().next().unwrap() {
            'd' => {
                let (_, name) = s.split_once(" ").unwrap();
                Ok(DirectoryItem::Directory {
                    name: name.to_string(),
                    items: Vec::new(),
                })
            }
            x if x.is_digit(10) => {
                let (size, name) = s.split_once(" ").unwrap();
                Ok(DirectoryItem::File {
                    name: name.to_string(),
                    size: size.parse().unwrap(),
                })
            }
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Command {
    CdDotDot,
    Cd { dir: String },
    Ls,
    LsResult { content: String },
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with('$') {
            Ok(Command::LsResult {
                content: s.to_string(),
            })
        } else if s == "$ cd .." {
            Ok(Command::CdDotDot)
        } else if s == "$ ls" {
            Ok(Command::Ls)
        } else if &s[..4] == "$ cd" {
            Ok(Cd {
                dir: s[5..].to_string(),
            })
        } else {
            Err(())
        }
    }
}

impl DirectoryItem {
    pub fn walk(&self, indent: &str) {
        match self {
            DirectoryItem::File { name, size } => {
                println!("{} File: {}, size: {}", indent, name, size)
            }
            DirectoryItem::Directory { name, items } => {
                println!("{} Dir: {}", indent, name);
                for d in items {
                    d.walk(&(indent.to_owned() + "  "));
                }
            }
        }
    }
}

pub fn prepare(input: String) -> Input1 {
    let mut output = prepare_inner(&mut input.lines(), "blah".to_string(), Vec::new());
    match output {
        DirectoryItem::File { .. } => output,
        DirectoryItem::Directory { name, mut items } => items.remove(0),
    }
}

pub fn ls<I, T>(input: &mut I, mut so_far: Vec<DirectoryItem>) -> Vec<DirectoryItem>
where
    I: Iterator<Item = T>,
    T: Borrow<str>,
{
    let binding = input.next();
    match binding {
        Some(b) => {
            let line = b.borrow();
            // println!("ls processing: '{}' from '{}'", line, &line[..1]);
            match &line[..1] {
                "$" => so_far,
                "d" => {
                    so_far.push(line.parse().unwrap());
                    ls(input, so_far)
                }
                x => {
                    so_far.push(line.parse().unwrap());
                    ls(input, so_far)
                }
            }
        }
        None => so_far,
    }
}

pub fn prepare_inner<I, T>(
    input: &mut I,
    current_name: String,
    mut current_vec: Vec<DirectoryItem>,
) -> DirectoryItem
where
    I: Iterator<Item = T>,
    T: Borrow<str>,
{
    let binding = input.next().map(|f| f.borrow().parse::<Command>().unwrap());
    // println!("Inner processing: {:?}", binding);
    match binding {
        Some(Command::Ls) => {
            // println!("ls running");
            prepare_inner(input, current_name, current_vec)
        }
        Some(Command::CdDotDot) => {
            // let items = ls(input, Vec::new());
            // current_vec.push(DirectoryItem::Directory {
            //     name: current_name.clone(),
            //     items: items,
            // });
            // let items = prepare_inner(input, current_name, current_vec);
            let dir = DirectoryItem::Directory {
                name: current_name.clone(),
                items: current_vec,
            };
            // println!("cd .. running, returning {:?}", dir);
            dir
        }
        Some(Command::Cd { dir }) => {
            // println!("Cd {:?} running", dir);
            let lower = prepare_inner(input, dir, Vec::new());
            // println!("Cd adding: {:?}", lower);
            current_vec.push(lower);
            prepare_inner(input, current_name, current_vec)
        }
        Some(Command::LsResult { content }) => {
            // This is processing the results of ls
            let cont = content.parse().unwrap();
            match cont {
                DirectoryItem::File { .. } => current_vec.push(cont),
                DirectoryItem::Directory { name, .. } => {
                    // println!("Not adding {}, will be added later", name)
                }
            }
            prepare_inner(input, current_name, current_vec)
        }
        None => {
            let dir = DirectoryItem::Directory {
                name: current_name.clone(),
                items: current_vec,
            };
            // println!("End of iterator, returning {:?}", dir);
            dir
        }
    }
}

fn size<'a>(tree: &'a DirectoryItem, hash_map: &mut HashMap<String, usize>, path: &str) -> usize {
    match tree {
        DirectoryItem::Directory { name, items } => {
            // Note that while this will work, it'll end up displaying something unusual because
            // of / being the first path "name":
            // //: 48381165
            // ///a: 94853
            // ///a/e: 584
            // ///d: 24933642
            let new_path = format!("{}/{}", path, name);
            let sum = items.iter().map(|d| size(d, hash_map, &new_path)).sum();
            // println!("Adding: {} {}", name, sum);
            match hash_map.insert(new_path.clone(), sum) {
                None => {}
                Some(x) => {
                    panic!("For some reason found {:?} already in the map", new_path);
                }
            }
            sum
        }
        DirectoryItem::File { name, size } => *size,
    }
}

pub fn part_1(input: Input1) -> Output1 {
    // input.walk("");
    let mut sizes = HashMap::<String, usize>::new();
    let total = size(&input, &mut sizes, "");
    // for (k, v) in sizes.iter() {
    //     println!("{}: {}", *k, *v);
    // }
    sizes
        .iter()
        .filter_map(|(k, v)| if *v <= 100000 { Some(*v) } else { None })
        .sum()
}

pub fn part_2(input: Input2) -> Output2 {
    // input.walk("");
    let required_space = 30000000usize;
    let file_system_size = 70000000usize;
    let mut sizes = HashMap::<String, usize>::new();
    let total = size(&input, &mut sizes, "");
    // for (k, v) in sizes.iter() {
    //     println!("{}: {}", *k, *v);
    // }
    let need_to_free = required_space - (file_system_size - total);
    // println!("Total space: {}", total);
    // println!("Need to free: {}", need_to_free);
    sizes.iter().fold(
        total,
        |acc, (k, v)| {
            if *v > need_to_free {
                min(acc, *v)
            } else {
                acc
            }
        },
    )
}

pub fn make_sol() -> StructSolution<Input1, Output1, Input2, Output2> {
    let struct_solution = StructSolution {
        prepare_part_1: prepare,
        calc_part_1: part_1,
        prepare_part_2: prepare,
        calc_part_2: part_2,
        example_part_1: Example::Value(95437),
        example_part_2: Example::Value(24933642),
        day_data: DayData::new(7, false),
    };
    struct_solution
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_dir() {
        let test_dir = DirectoryItem::Directory {
            name: "/".to_string(),
            items: vec![DirectoryItem::File {
                name: "a".to_string(),
                size: 0,
            }],
        };
        test_dir.walk("")
    }

    #[test]
    fn test_build_dir() {
        let dir_structure =
            "$ cd /\n$ ls\ndir a\n123 b.txt\ndir b\n$ cd b\n$ ls\n456 o.txt\n$ cd ..\n";
        let output = prepare(dir_structure.to_string());
        output.walk("");
        // println!("{:?}", output)
    }

    #[test]
    fn test_parser() {
        assert_eq!(
            "$ cd hello".parse::<Command>().unwrap(),
            Command::Cd {
                dir: "hello".to_string()
            }
        );
        assert_eq!("$ cd ..".parse::<Command>().unwrap(), Command::CdDotDot);
        assert_eq!("$ ls".parse::<Command>().unwrap(), Command::Ls);
        assert_eq!(
            "123 h".parse::<Command>().unwrap(),
            Command::LsResult {
                content: "123 h".to_string()
            }
        );
    }
}

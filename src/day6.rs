use crate::solution::{Example, StructSolution};
use crate::DayData;
use std::collections::{HashMap, HashSet};

type Input1 = String;
type Output1 = usize;
type Input2 = String;
type Output2 = usize;

pub struct MostRecentlySeen {
    pub window: u8,
    pub hash_map: HashMap<char, u8>,
    pub current_window: u8,
    // This is really lazy, a ring buffer would be a better way to do this
    pub added_chars: Vec<char>,
}

impl MostRecentlySeen {
    pub fn new(window: u8) -> MostRecentlySeen {
        MostRecentlySeen {
            window,
            hash_map: Default::default(),
            current_window: 0,
            added_chars: Vec::new(),
        }
    }
    pub fn add(&mut self, c: char) -> bool {
        // println!("Before {:?}", self.hash_map);
        let mut could_be_unique = true;
        self.current_window += 1;
        self.added_chars.push(c);
        let new_val = self.hash_map.get_mut(&c);
        match new_val {
            Some(v) => {
                *v += 1;
            }
            None => {
                self.hash_map.insert(c, 1);
            }
        }
        if self.current_window > self.window {
            let least_recently_seen =
                self.added_chars[self.added_chars.len() - self.window as usize - 1];
            let old_val = self.hash_map.get_mut(&least_recently_seen);
            match old_val {
                Some(v) => {
                    *v -= 1;
                    if *v >= 2 {
                        could_be_unique = false;
                    }
                }
                None => panic!(
                    "Expecting to find {} in HashMap but didn't",
                    least_recently_seen
                ),
            }
            self.current_window -= 1;
        }
        println!(
            "{}, {}, After {:?}",
            self.current_window,
            self.added_chars.len(),
            self.hash_map
        );
        if could_be_unique {
            self.is_unique()
        } else {
            false
        }
    }

    pub fn is_unique(&self) -> bool {
        self.hash_map.iter().filter(|(k, v)| **v == 1).count() >= self.window as usize
    }
}

pub fn prepare(input: String) -> Input1 {
    input
}

pub fn part_1(input: Input1) -> Output1 {
    let mut counter = MostRecentlySeen::new(4);
    for c in input.chars() {
        let res = counter.add(c);
        if res {
            break;
        }
    }
    counter.added_chars.len()
}

pub fn part_2(input: Input2) -> Output2 {
    let mut counter = MostRecentlySeen::new(14);
    for c in input.chars() {
        let res = counter.add(c);
        if res {
            break;
        }
    }
    counter.added_chars.len()
}

pub fn make_sol() -> StructSolution<Input1, Output1, Input2, Output2> {
    let struct_solution = StructSolution {
        prepare_part_1: prepare,
        calc_part_1: part_1,
        prepare_part_2: prepare,
        calc_part_2: part_2,
        example_part_1: Example::Value(7),
        example_part_2: Example::Value(19),
        day_data: DayData::new(6, false),
    };
    struct_solution
}

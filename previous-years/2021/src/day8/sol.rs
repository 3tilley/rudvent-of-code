use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, Not};
use std::process::Output;
use crate::utils;

use flagset::{FlagSet, flags};
use crate::utils::Solution;
use std::str::Chars;
use std::prelude::rust_2021::FromIterator;
use criterion::SamplingMode::Flat;

flags! {

    #[derive(Hash)]
    pub enum Segments: u8 {
        A,
        B,
        C,
        D,
        E,
        F,
        G,
    }

}

impl From<FlagSet<Segments>> for Segments {
    fn from(flagset: FlagSet<Segments>) -> Self {
        let vec : Vec<Segments> = flagset.into_iter().collect();
        assert_eq!(vec.len(),1);
        //Segments(flagset.bits())
        vec[0]
    }
}

flags! {
    #[derive(Hash)]
    pub enum Actuals: u8 {
        A,
        B,
        C,
        D,
        E,
        F,
        G,
    }
}

//fn actual_from_segments(segments: Segments)

pub fn make_grid() -> Vec<FlagSet<Actuals>> {
    let mut vec = Vec::new();
    vec.push(FlagSet::<Actuals>::full() ^ Actuals::D);
    vec.push(Actuals::C | Actuals::F);
    vec.push(FlagSet::<Actuals>::full() ^ (Actuals::B | Actuals::F));
    vec.push(FlagSet::<Actuals>::full() ^ (Actuals::B | Actuals::E));
    vec.push(Actuals::B | Actuals::C | Actuals::F | Actuals::D);
    vec.push(FlagSet::<Actuals>::full() ^ (Actuals::C | Actuals::E));
    vec.push(FlagSet::<Actuals>::full() ^ Actuals::C);
    vec.push(Actuals::A | Actuals::C | Actuals::F);
    vec.push(FlagSet::<Actuals>::full());
    vec.push(FlagSet::<Actuals>::full() ^ Actuals::E);
    vec
}


fn count_segments<T: flagset::Flags>(flags: FlagSet<T>) -> u32 where FlagSet<T>: From<T> {
    let mut count = 0;
    for x in FlagSet::<T>::full().into_iter() {
        if flags.contains(x) {
            count += 1;
        }
    }
    count
}

struct Possibilities {
    map: HashMap<Segments, FlagSet<Actuals>>,
    grid: Vec<FlagSet<Actuals>>,
}

impl Possibilities {
    fn new() -> Possibilities {
        let grid = make_grid();
        let iter = FlagSet::<Segments>::full().into_iter().map(|x| (x, FlagSet::full()));
        let map = HashMap::from_iter(iter);
        Possibilities{map, grid}
    }

    fn options(&self, segment: &Segments) -> u32 {
        count_segments::<Actuals>(self.map[segment])
    }

    fn all_resolved(&self) -> bool {
        self.map.iter().all(|(x, _) | self.options(x) == 1)
    }

    fn add_one_seven(&mut self, one: FlagSet<Segments>, seven: FlagSet<Segments>) {
        assert_eq!(count_segments(one), 2);
        assert_eq!(count_segments(seven), 3);

        for one_seg in one.into_iter() {
            //let mut original = self.map.get_mut(&one_seg).unwrap();
            //original = &mut self.grid[1];
            //self.map.entry(one_seg).and_modify(|mut e|) self.grid[1]);
            self.update_actuals(one_seg, self.grid[1], false);
            //self.map.entry(one_seg).and_modify(|mut e| e = self.grid[1].clone());
        }
        self.update_actuals(Segments::from(seven - one), Actuals::A | Actuals::A, false);

        for other in FlagSet::<Segments>::not(seven).into_iter() {
            self.update_actuals(other, self.grid[7], true);
        }
    }
    fn add_four(&mut self, four: FlagSet<Segments>) {
        self.add_num(four, 4);
    }

    fn add_zero_six_nine(&mut self, candidates: Vec<FlagSet<Segments>>) {
        let intersect = Actuals::A | Actuals::B | Actuals::F | Actuals::G;
        for c in candidates {
            for seg in c.into_iter() {
                self.update_actuals(seg, intersect, false)
            }
        }
    }

    fn add_two_three_five(&mut self, candidates: Vec<FlagSet<Segments>>) {
        let intersect = Actuals::A | Actuals::D | Actuals::G;
        for c in candidates {
            for seg in c.into_iter() {
                self.update_actuals(seg, intersect, false)
            }
        }
    }

    fn add_num(&mut self, letters: FlagSet<Segments>, num: usize) {
        let num_actuals = self.grid[num];
        for seg in FlagSet::<Segments>::full().into_iter() {
            println!("{:?}, {:?}, {}", seg, num_actuals, letters.contains(seg));
            self.update_actuals(seg, num_actuals, !letters.contains(seg))
        }
    }

    fn update_actuals(&mut self, segment: Segments, actual: FlagSet<Actuals>, remove: bool) {
        if remove {
            self.map.entry(segment).and_modify(|mut e| *e -= actual);
        } else {
            self.map.entry(segment).and_modify(|mut e| *e &= actual);
        }
    }

}

pub struct Line {
    signals: Vec<FlagSet<Segments>>,
    output: Vec<FlagSet<Segments>>
}

pub fn to_segment(chr: char) -> Segments {
    match chr {
        'a' => Segments::A,
        'b' => Segments::B,
        'c' => Segments::C,
        'd' => Segments::D,
        'e' => Segments::E,
        'f' => Segments::F,
        'g' => Segments::G,
        _ => panic!("Unrecognised char: {}", chr)
    }
}

fn from_word(chars: Chars) -> FlagSet<Segments> {
    let mut flagset = FlagSet::<Segments>::default();
    for c in chars {
        flagset = flagset | to_segment(c);
    }
    flagset
}
// pub fn from_flags(flags: &[F]) -> Self {
//     let mut flagset = Self(F::ZERO);
//     for f in flags {
//         flagset = flagset | *f;
//     }
//     flagset
// }

fn read_line(input_line: &str) -> Line {
    let (signals_str, output_str) = input_line.split_once("|").unwrap();
    let signals = signals_str.trim().split(" ").map(|s| from_word(s.trim().chars())).collect();
    let output = output_str.trim().split(" ").map(|s| from_word(s.trim().chars())).collect();
    Line{signals, output}
}

pub fn load_data(name: &str) -> Vec<Line> {
    let contents = utils::read_file(name, file!());
    let lines: Vec<Line> = contents.lines().map(read_line).collect();

    lines
}

pub fn ans(name: &str, linear: bool) -> u32 {
    let input = load_data(name);
    //log::debug!("{:?}", input);
    3
}

pub struct Sol {

}

impl Solution<u32, Vec<Line>, u32> for Sol {
    fn prepare_a(example: bool) -> Vec<Line> {
         if example {
             load_data("example.txt")
         } else {
             load_data("input.txt")
         }
     }
    fn inner_a(prep: Vec<Line>) -> u32 {
        // Todo: why doesn't map work here
        let mut count = 0;
        for line in prep {
            let line_sum = line.output.into_iter().map(|seg| {
                let segment_count = count_segments(seg);
                match segment_count {
                    2 | 3 | 4 | 7 => 1,
                    _ => 0,
                }
            }).sum::<u32>();
            //log::debug!("Uniques in line: {}", line_sum);
            count += line_sum
        }
        count
    }

    fn inner_b(prep: Vec<Line>) -> u32 {

        for mut line in prep {
            let mut poss = Possibilities::new();
            let mut together = line.output.clone();
            together.append(&mut line.signals);
            together.sort();
            together.dedup();

            let groupby = together.iter().group_by(|e| count_segments(e));
            for (key, group) in groupby {
                match key {
                    2 => poss.add_num(group, 1),
                    3 => poss.add_num(group, 7),
                    4 => poss.add_num(group, 4),
                    5 => poss.add_two_three_five(group),
                    6 => poss.add_zero_six_nine(group),
                    _ => (),
                }
            }
        }
        3
    }


    fn output_a(answer: u32) -> u32 {
        answer
    }
}


#[cfg(test)]
mod tests {
    use criterion::SamplingMode::Flat;
    use flagset::FlagSet;
    use crate::day8::sol::{Segments, to_segment, read_line, from_word, count_segments, Possibilities};

    #[test]
    fn test_read_flag() {
        assert_eq!(to_segment('a'), Segments::A);
    }

    #[test]
    fn test_read_multiple_flags() {
        let seg_a = to_segment('a');
        let seg_b = to_segment('b');
        let segs = vec![seg_a, seg_b];
        //let flagset: FlagSet<Segments> = FlagSet::new(seg_a).unwrap();
        let flagset_2: FlagSet<Segments> = seg_a | seg_b;
        //let flagset_3: FlagSet<Segments> = FlagSet::from(segs);
        flagset_2.bits();
        assert_eq!(to_segment('a'), Segments::A);
    }

    #[test]
    fn test_read_line() {
        let input_string = "be ab | gc abc";
        let line = read_line(input_string);
        let counts: Vec<_> = line.output.into_iter().map(count_segments).collect();
        assert_eq!(counts[0], 2);
        assert_eq!(counts[1], 3);
    }

    #[test]
    fn test_count() {
        let flagset = from_word("ab".chars());
        assert_eq!(count_segments(flagset), 2);
    }

    #[test]
    fn test_possibilities_start() {
        let poss = Possibilities::new();
        assert_eq!(poss.all_resolved(), false);
        for s in FlagSet::<Segments>::full().into_iter() {
            assert_eq!(poss.options(&s), 7);
        }
    }

    #[test]
    fn test_possibilities_add_seven() {
        let mut poss = Possibilities::new();
        poss.add_one_seven(from_word("ab".chars()), from_word("abc".chars()));
        assert_eq!(poss.options(&Segments::C), 1);
    }

    #[test]
    fn test_possibilities_add_four() {
        let mut poss = Possibilities::new();
        poss.add_four(from_word("bcdf".chars()));
        assert_eq!(poss.options(&Segments::A), 3);
        assert_eq!(poss.options(&Segments::B), 4);
    }

    #[test]
    fn test_possibilities_add_multiple() {
        let mut poss = Possibilities::new();
        println!("{:?}", poss.map);
        poss.add_one_seven(from_word("cf".chars()), from_word("acf".chars()));
        println!("{:?}", poss.map);
        poss.add_four(from_word("bcdf".chars()));
        println!("{:?}", poss.map);
    }
}
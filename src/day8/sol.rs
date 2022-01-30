use std::cmp::max;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Add;
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
}

impl Possibilities {
    fn new() -> Possibilities {
        let iter = FlagSet::<Segments>::full().into_iter().map(|x| (x, FlagSet::full()));
        let map = HashMap::from_iter(iter);
        Possibilities{map}
    }

    fn options(&self, segment: &Segments) -> u32 {
        count_segments::<Actuals>(self.map[segment])
    }

    fn all_resolved(&self) -> bool {
        self.map.iter().all(|(x, _) | self.options(x) == 1)
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

        for lines in prep {
            poss = Possibilities::new();
        }
    }


    fn output_a(answer: u32) -> u32 {
        answer
    }
}


#[cfg(test)]
mod tests {
    use flagset::FlagSet;
    use crate::day8::sol::{Segments, to_segment, read_line, from_word, count_segments};

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
}
use std::cmp::max;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Add;
use std::process::Output;
use crate::utils;

use flagset::{FlagSet, flags};

flags! {
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

fn read_line(input_line: &str) {
    let (signals_str, output_str) = input_line.split_once("|").unwrap();
    let signals = signals_str.trim().split(" ").map(|s| s.chars().map(to_segment));
}

pub fn load_data(name: &str) -> Vec<u32> {
    let contents = utils::read_file(name, file!());
    //let (signals, displays) = contents;
    //contents.trim().split(",").map(|x| x.parse::<u32>().unwrap()).collect()
    vec![1,2]

}

pub fn ans(name: &str, linear: bool) -> u32 {
    let input = load_data(name);
    log::debug!("{:?}", input);
    3
}

pub fn a(name: &str) -> u32 {
    ans(name, true)
}

pub fn b(name: &str) -> u32 {
    ans(name, false)
}

#[cfg(test)]
mod tests {
    use flagset::FlagSet;
    use crate::day8::sol::{Segments, to_segment};

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
}
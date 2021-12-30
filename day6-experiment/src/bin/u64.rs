use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::Path;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Add;
use chrono::{DateTime, Utc};

struct Fish {
    map: HashMap<u32, u64>,
}

impl Fish {
    fn new(fish: Vec<u32>) -> Fish {
        let mut map: HashMap<u32, u64> = HashMap::new();
        for day in 0u32..9u32 {
            map.insert(day, 0);
        }
        for f in fish {
            let original = map[&f];
            let new_val = original + 1;
            map.insert(f, new_val);
        }
        Fish{map}
    }

    fn update(&mut self) {
        let new_fish = *self.map.get(&0u32).unwrap();
        for day in 1u32..9u32 {
            let old = self.map[&day];
            self.map.insert(day - 1, old);
        }
        self.map.insert(8,new_fish);
        let old_val = self.map[&6u32];
        let new_val = old_val.checked_add(new_fish).unwrap();
        //let new_val = old_val.add(new_fish);
        self.map.insert(6u32, new_val).unwrap();
    }

    fn num_fish(&self) -> u64 {
        let mut counter : u64 = 0;
        for v in self.map.values() {
            counter = counter.checked_add(*v).unwrap();
        }
        counter
    }

}

impl fmt::Display for Fish {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut out_vec: Vec<(u32, u64)> = Vec::new();
        for (&k, &v) in self.map.iter() {
            out_vec.push((k, v));
        }
        out_vec.sort_by_key(|x| x.0);
        let mut out_strings = Vec::new();
        for (_, v) in out_vec {
            out_strings.push(format!("{}", v));
        }
        let out_string : String = out_strings.join(" ");
        write!(f, "{}", out_string)
    }
}

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

pub fn load_data(name: &str) -> Vec<u32> {
    let contents = read_file(name, file!());
    contents.trim().split(",").map(|x| x.parse::<u32>().unwrap()).collect()
}

pub fn ans(name: &str, days: u32) -> u64 {
    let input = load_data(name);
    //log::debug!("{:?}", input);
    let mut fishes = Fish::new(input);
    let start : DateTime<Utc> = Utc::now();
    for _ in 0..days {
        fishes.update();
        //println!("{} | day {},  {:?}", fishes, d, fishes.num_fish());
    }
    let duration = Utc::now() - start;

    println!("{}us", duration.num_microseconds().unwrap());
    fishes.num_fish()
}

pub fn a(name: &str) -> u64 {
    ans(name, 18)
}

pub fn b(name: &str) -> u64 {
    ans(name, 256)
}

fn main() {

    let answer = b("input.txt");

    println!("Answer: {:?}", answer);
}

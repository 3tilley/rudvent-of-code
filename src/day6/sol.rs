use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Add;
use std::process::Output;
use chrono::{DateTime, Utc};
use fxhash::{FxHasher, FxHashMap};
use crate::utils;


pub trait Numeric : Sized + Add<Self, Output = Self> + Copy + Display + Debug{
    fn zero() -> Self;
    fn one() -> Self;
    fn checked_add(&self, other: Self) -> Option<Self>;
}

impl Numeric for u32 {
    fn zero() -> u32 {
        0u32
    }

    fn one() -> u32 {
        1u32
    }
    fn checked_add(&self, other: Self) -> Option<u32> {
        u32::checked_add(*self, other)
    }
}

impl Numeric for u64 {
    fn zero() -> u64 {
        0u64
    }
    fn one() -> u64 {
        1u64
    }
    fn checked_add(&self, other: Self) -> Option<u64> {
        u64::checked_add(*self, other)
    }
}

pub struct Fish<T>
    where
        T: Numeric {
    map: FxHashMap<u32, T>,
}

impl<T: Numeric> Fish<T> {
    fn new(fish: Vec<u32>) -> Fish<T> {
        //let mut map: HashMap<u32, T> = HashMap::new();
        let mut map = FxHashMap::default();
        for day in 0u32..9u32 {
            map.insert(day, Numeric::zero());
        }
        //log::debug!("{:?}", map);
        for f in fish {
            let original : T = map[&f];
            let new_val = original + T::one();
            map.insert(f, new_val);
        }
        //log::debug!("{:?}", map);
        Fish{map}
    }

    #[inline(never)]
    fn update(&mut self) {
        let new_fish = *self.map.get(&0u32).unwrap();
        //log::debug!("{:?}", new_fish);
        for day in 1u32..9u32 {
            let old = self.map[&day];
            self.map.insert(day - 1, old);
        }
        //log::debug!("{:?}", self.map);
        self.map.insert(8,new_fish);
        let old_val = self.map[&6u32];
        //let new_val = old_val.checked_add(new_fish).unwrap();
        let new_val = old_val.add(new_fish);
        self.map.insert(6u32, new_val).unwrap();
        //log::debug!("{:?}", self.map);
    }

    fn num_fish(&self) -> T {
        let mut counter = T::zero();
        for v in self.map.values() {
            counter = counter.checked_add(*v).unwrap();
        }
        counter
    }

}


pub struct VecFish<T> where T: Numeric {
    vec: Vec<T>,
}

impl<T: Numeric> VecFish<T> {
    fn new(fish: Vec<u32>) -> VecFish<T> {
        let mut vec: Vec<T> = vec![Numeric::zero(); 9];
        //log::debug!("{:?}", map);
        for f in fish {
            let mut original = vec.get_mut(f as usize).unwrap();
            *original = original.checked_add(T::one()).unwrap();
        }
        log::debug!("{:?}", vec);
        VecFish{vec}
    }

    #[inline(never)]
    fn update(&mut self) {
        let new_fish = self.vec[0];
        for day in 1..9 {
            self.vec[day-1] = self.vec[day];
        }
        //log::debug!("{:?}", self.vec);
        self.vec[8] = new_fish;
        let old_val = self.vec[6];
        //let new_val = old_val.checked_add(new_fish).unwrap();
        let new_val = old_val.checked_add(new_fish).unwrap();
        self.vec[6] = new_val;
        //log::debug!("{:?} - total: {}", self.vec, self.num_fish());
    }

    fn num_fish(&self) -> T {
        let mut counter = T::zero();
        for v in self.vec.iter() {
            counter = counter.checked_add(*v).unwrap();
        }
        counter
    }

}
impl<T: Numeric> fmt::Display for Fish<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut out_vec: Vec<(u32, T)> = Vec::new();
        for (&k, &v) in self.map.iter() {
            out_vec.push((k, v));
        }
        out_vec.sort_by_key(|x| x.0);
        let mut out_strings = Vec::new();
        for (k, v) in out_vec {
            out_strings.push(format!("{}", v));
        }
        let out_string : String = out_strings.join(" ");
        write!(f, "{}", out_string)
    }
}

pub fn load_data(name: &str) -> Vec<u32> {
    let contents = utils::read_file(name, file!());
    contents.trim().split(",").map(|x| x.parse::<u32>().unwrap()).collect()
}

pub fn ans<T: Numeric>(name: &str, days: u32) -> T {
    let input = load_data(name);
    //log::debug!("{:?}", input);
    let mut fishes = VecFish::new(input);
    let start : DateTime<Utc> = Utc::now();
    do_it(&mut fishes, days);
    let duration = Utc::now() - start;

    println!("{}ns", duration.num_nanoseconds().unwrap());
    fishes.num_fish()
}

pub fn do_it<T: Numeric>(fish: &mut VecFish<T>, days: u32) {
    for d in 0..days {
        fish.update();
        //println!("{} | day {},  {:?}", fishes, d, fishes.num_fish());
    }
}

pub fn a(name: &str) -> u32 {
    ans(name, 18)
}

pub fn b(name: &str) -> u64 {
    ans(name, 256)
}

use std::cmp::max;
use std::collections::HashMap;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Add;
use std::process::Output;
use crate::utils;

struct Crabs{
    map: HashMap<u32, u32>,
    max_pos: u32,
}

impl Crabs {
    fn new(crabs: Vec<u32>) -> Crabs {
        let mut map: HashMap<u32, u32> = HashMap::new();
        let mut max_pos = 0u32;
        for crab in crabs {
            match map.get_mut(&crab) {
                Some(count) => *count += 1,
                None => {
                    map.insert(crab, 1);
                }
            }
            max_pos = max(max_pos, crab);
        }
        log::debug!("{:?}", map);
        Crabs{map, max_pos}
    }

    fn cost_single(crabs: u32, pos: u32, target: u32) -> u32 {
        let ans =
            if pos > target {
                crabs * (pos - target)
            } else {
                crabs * (target - pos)
            };
        log::trace!("Pos: {}, crabs: {}, cost: {}", pos, crabs, ans);
        ans
    }

    fn cost_single_non_linear(crabs: u32, pos: u32, target: u32) -> u32 {
        let diff =
            if pos > target {
                pos - target
            } else {
                target - pos
            };
        let ans = (diff.checked_mul(diff.checked_add(1).unwrap()).unwrap()) / 2;
        log::trace!("Pos: {}, crabs: {}, cost: {}", pos, crabs, ans);
        ans.checked_mul(crabs).unwrap()
    }
    fn cost_all(&self, target: u32, linear:bool) -> u32 {
        let mut costs = Vec::new();
        for pos in 0..self.max_pos+1 {
            let crabs = self.map.get(&pos).unwrap_or(&0u32);
            if linear {
                costs.push(Crabs::cost_single(*crabs, pos, target));
            } else {
                costs.push(Crabs::cost_single_non_linear(*crabs, pos, target));
            }
        }
        let ans = costs.iter().sum();
        log::info!("Checking cost for: {}, cost: {}", target, ans);
        ans
    }

    fn cheapest(&self, linear: bool) -> u32 {
        (0..self.max_pos+1).map(|x| self.cost_all(x, linear)).min().unwrap()
    }
}

impl fmt::Display for Crabs {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let mut out_vec: Vec<(u32, u32)> = Vec::new();
        for (&k, &v) in self.map.iter() {
            out_vec.push((k, v));
        }
        out_vec.sort_by_key(|x| x.0);
        let mut headers = Vec::new();
        let mut out_strings = Vec::new();

        for (k, v) in out_vec {
            headers.push(format!("{:3}", k));
            out_strings.push(format!("{:3}", v));
        }
        let header_string : String = headers.join(" ");
        let out_string : String = out_strings.join(" ");
        write!(f, "\n{}\n{}", header_string, out_string)
    }
}

pub fn load_data(name: &str) -> Vec<u32> {
    let contents = utils::read_file(name, file!());
    contents.trim().split(",").map(|x| x.parse::<u32>().unwrap()).collect()
}

pub fn ans(name: &str, linear: bool) -> u32 {
    let input = load_data(name);
    log::debug!("{:?}", input);
    let mut crabs = Crabs::new(input);
    log::debug!("{}", crabs);
    if linear {
        crabs.cheapest(true)
    } else {
        crabs.cheapest(false)
    }
}

pub fn a(name: &str) -> u32 {
    ans(name, true)
}

pub fn b(name: &str) -> u32 {
    ans(name, false)
}

#[cfg(test)]
mod tests {
    use crate::day7::sol::a;
    use crate::day7::sol::b;
    use crate::day7::sol::Crabs;

    #[test]
    fn build_crabs() {
        let input: Vec<u32> = vec![0,1,3,3,5,5,5];
        let crabs = Crabs::new(input);
        assert_eq!(crabs.map[&0], 1);
        assert_eq!(crabs.map[&1], 1);
        assert_eq!(crabs.map[&3], 2);
        assert_eq!(crabs.map[&5], 3);

    }

    #[test]
    fn example_a() {
        assert_eq!(a("example.txt"), 37);
    }

    #[test]
    fn example_b() {
        assert_eq!(b("example.txt"), 168);
    }

    #[test]
    fn test_non_linear() {
        assert_eq!(Crabs::cost_single_non_linear(1, 1, 2), 1);
        assert_eq!(Crabs::cost_single_non_linear(1, 3, 1), 3);
        assert_eq!(Crabs::cost_single_non_linear(2, 1, 4), 12);
    }
}

use std::cmp::Ordering;
use crate::solution::{Example, StructSolution};
use crate::DayData;
use std::str::FromStr;

type Input1 = Vec<(Packet, Packet)>;
type Output1 = usize;
type Input2 = Vec<(Packet, Packet)>;
type Output2 = usize;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Packet {
    Value { val: u8 },
    List { packets: Vec<Packet> },
}

impl PartialOrd<Self> for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match is_pair_sorted(&self, other) {
            None => Ordering::Equal,
            Some(true) => Ordering::Less,
            Some(false) => Ordering::Greater,
        }
    }
}

impl FromStr for Packet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match (s.chars().nth(0), s.chars().nth_back(0)) {
            (None, None) => Ok(Packet::List {
                packets: Vec::new(),
            }),
            (Some('['), Some(']')) if s.len() == 2 => Ok(Packet::List {
                packets: Vec::new(),
            }),
            (Some('['), Some(']')) =>{
                let inner = &s[1..s.len() - 1];
                if inner.contains('[') {
                    let packets = tokenise(s).iter().map(|(start,end)| Packet::from_str(&s[*start..*end]).unwrap()).collect();
                    Ok( Packet::List {packets})
                } else {
                    Ok(Packet::List {
                        packets: inner
                            .split(",")
                            .map(|s_| Packet::from_str(s_).unwrap())
                            .collect(),
                    })
                }
            },
            (Some('['), Some(x)) | (Some(x), Some(']')) => Err(()),
            (Some(x), _) => Ok(Packet::Value {
                val: s.parse().unwrap(),
            }),
            _ => Err(()),
        }
    }
}

fn tokenise(s: &str) -> Vec<(usize, usize)> {
    let mut depth: usize = 0;
    let mut start: usize = 0;
    let mut end : usize = 0;
    let mut start_ends  =Vec::new();
    // println!("Start: {}, end: {}, depth: {}", start, end, depth);
    for (i,c) in s.chars().enumerate() {
        // println!("{}: {}. Start: {}, end: {}, depth: {}", i, c, start, end, depth);
        match c {
            '[' => {
                depth += 1;
                if depth == 1 {
                    start = i+1;
                }
            } ,
            ']' => {
                depth -= 1;
                if depth == 0 {end = i} ;
            },
            ',' if depth == 1 => {
                start_ends.push((start, i));
                end = i;
                start = i + 1;
            }
            x => ()
        }
    }
    start_ends.push((start, end));
    start_ends
}

pub fn is_pair_sorted(p1: &Packet, p2: &Packet) -> Option<bool> {
    // println!("p1: {:?}", &p1);
    // println!("p2: {:?}", &p2);
    match (p1, p2) {
        (Packet::Value { val }, Packet::Value { val: val2 }) => {
            if val < val2 {
                Some(true)
            } else if val > val2 {
                Some(false)
            } else {
                None
            }
        }
        (Packet::List { packets: list }, Packet::List { packets: list2 }) => {
            for i in 0..list.len() + 1 {
                match (list.get(i), list2.get(i)) {
                    (Some(v), Some(v2)) => match is_pair_sorted(v, v2) {
                        None => {
                            return is_pair_sorted(
                                &Packet::List {
                                    packets: list[1..].to_vec(),
                                },
                                &Packet::List {
                                    packets: list2[1..].to_vec(),
                                },
                            )
                        }
                        Some(x) => return Some(x),
                    },
                    (None, None) => return None,
                    (Some(v), None) => return Some(false),
                    (None, Some(v)) => return Some(true),
                }
            }
            panic!("Shouldn't ever get here");
        }
        (Packet::Value { .. }, Packet::List { .. }) => is_pair_sorted(
            &Packet::List {
                packets: vec![p1.clone(); 1],
            },
            p2,
        ),
        (Packet::List { .. }, Packet::Value { .. }) => is_pair_sorted(
            p1,
            &Packet::List {
                packets: vec![p2.clone(); 1],
            },
        ),
    }
}

pub fn prepare(input: String) -> Input1 {
    input
        .split("\n\n")
        .map(|chunk| {
            let (left, right) = chunk.trim().split_once("\n").unwrap();
            (left.parse().unwrap(), right.parse().unwrap())
        })
        .collect()
}

pub fn part_1(input: Input1) -> Output1 {
    input
        .iter()
        .enumerate()
        .filter_map(|(i, (p1, p2))|{
            // println!("p1: {:?}", p1);
            // println!("p2: {:?}", p2);
            match is_pair_sorted(p1, p2).unwrap() {
                true => Some(i + 1),
                false => {
                    // println!("Not sorted: {}", i);
                    None
                },
        }
        })
        .sum()
}

pub fn part_2(input: Input2) -> Output2 {
    let mut packets = Vec::new();
    let first = Packet::List{packets: vec![Packet::List {packets: vec![Packet::Value { val: 2}]}]};
    let second = Packet::List{packets: vec![Packet::List {packets: vec![Packet::Value { val: 6}]}]};
    packets.push(first.clone());
    packets.push(second.clone());
    for (p1, p2) in input {
        packets.push(p1);
        packets.push(p2);
    }
    packets.sort();
    let i_1 = packets.iter().position(|p| p == &first).unwrap() + 1;
    let i_2 = packets.iter().position(|p| p == &second).unwrap() + 1;
    i_1 * i_2
}

pub fn make_sol() -> StructSolution<Input1, Output1, Input2, Output2> {
    let struct_solution = StructSolution {
        prepare_part_1: prepare,
        calc_part_1: part_1,
        prepare_part_2: prepare,
        calc_part_2: part_2,
        example_part_1: Example::Value(13),
        example_part_2: Example::Value(140),
        day_data: DayData::new(13, false),
    };
    struct_solution
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_packet() {
        let p: Packet = "1".parse().unwrap();
        let p_list: Packet = "[1,2,3]".parse().unwrap();
        let p_list_empty: Packet = "[]".parse().unwrap();
        let p_list_lists: Packet = "[[1,2],[3]]".parse().unwrap();

        assert_eq!(p, Packet::Value { val: 1 });
        assert_eq!(
            p_list,
            Packet::List {
                packets: vec![
                    Packet::Value { val: 1 },
                    Packet::Value { val: 2 },
                    Packet::Value { val: 3 }
                ]
            }
        );
        assert_eq!(
            p_list_empty,
            Packet::List {
                packets: Vec::new()
            }
        );
    }

    #[test]
    fn token() {
        let t = tokenise("[1,2,13]");
        let t_double = tokenise("[[1,2,13]]");
        assert_eq!(t, vec![(1,2), (3,4), (5,7)]);
        assert_eq!(t_double, vec![(1,9)]);
    }

    #[test]
    fn eq_test() {
        let p : Packet = "[[2]]".parse().unwrap();
        let first = Packet::List{packets: vec![Packet::List {packets: vec![Packet::Value { val: 2}]}]};
        assert_eq!(p, first);
    }
}

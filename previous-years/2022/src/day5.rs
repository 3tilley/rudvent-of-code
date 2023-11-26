use crate::solution::{Example, StructSolution};
use crate::DayData;
use std::cmp::min;
use std::ops::DerefMut;
use std::str::FromStr;

type Input1 = CratesAndInstructions;
type Output1 = String;
type Input2 = CratesAndInstructions;
type Output2 = String;

#[derive(Copy, Clone, Debug)]
pub struct MoveInstruction {
    pub quantity: u8,
    pub from: u8,
    pub to: u8,
}

impl FromStr for MoveInstruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        println!("{}", s);
        let chunks: Vec<&str> = s.split_whitespace().collect();

        //  0    1  2    3 4  5
        // "move 10 from 2 to 3"
        let quantity_str: &str = chunks.get(1).unwrap();
        let from_str: &str = chunks.get(3).unwrap();
        let to_str: &str = chunks.get(5).unwrap();
        println!("'{}' - '{}' - '{}'", quantity_str, from_str, to_str);
        Ok(MoveInstruction {
            quantity: quantity_str.parse().unwrap(),
            from: from_str.parse().unwrap(),
            to: to_str.parse().unwrap(),
        })
    }
}

#[derive(Debug)]
pub struct CratesAndInstructions {
    pub crates: Vec<Vec<char>>,
    pub moves: Vec<MoveInstruction>,
}

impl CratesAndInstructions {
    pub fn top_of_stacks(&self) -> String {
        self.crates
            .iter()
            .filter_map(|c| c.last())
            .collect::<String>()
    }
}

pub fn prepare(input: String) -> Input1 {
    let (crates_str, moves_str) = input.split_once("\n\n").unwrap();
    println!("crate_str: {}", crates_str);
    let mut crate_vec: Vec<Vec<char>> = Vec::new();

    let split_lines = crates_str.split("\n");

    for line in split_lines {
        println!("Len: {} - {}", line.len(), line);
        let mut it = line.chars();
        println!("{:?}", line.chars());
        let stack_count = ((line.len() + 1) / 4);
        println!("Stack count: {:?}", stack_count);
        for i in 0..stack_count {
            let next = it.next();
            if next.is_some() & (crate_vec.len() <= i) {
                println!("Pushing: {:?}", next);
                crate_vec.push(Vec::new())
            };
            match next {
                Some(' ') => {
                    it.next();
                    it.next();
                    it.next();
                }
                Some('[') => {
                    let next_val = it.next().unwrap();
                    let mut val = crate_vec.get_mut(i).unwrap();
                    val.push(next_val);
                    it.next();
                    it.next();
                }
                Some(x) if x.is_digit(10) => (),
                Some(x) => panic!("Unknown character {}", x),
                None => (),
            }
        }
    }
    // Currently higher crates are first in the vectors, which isn't what we want
    crate_vec.iter_mut().for_each(|mut c| c.reverse());

    let moves = moves_str
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    println!("{:?}", crate_vec);

    CratesAndInstructions {
        crates: crate_vec,
        moves: moves,
    }
}

pub fn part_1(mut input: Input1) -> Output1 {
    for m in &input.moves {
        for _ in 0..m.quantity {
            let c = input.crates[m.from as usize - 1].pop().unwrap();
            input.crates[m.to as usize - 1].push(c);
        }
    }
    input.top_of_stacks()
    // .to_string()
}

fn move_between_vecs(from: &mut Vec<char>, to: &mut Vec<char>, start_index: usize) {
    to.extend(from.drain(start_index..))
}

pub fn part_2(mut input: Input2) -> Output2 {
    // This is pretty complex as it does a copy-free drain between the vectors. This is almost certainly
    // slower than doing a copy, but timings to be seen
    for m in &input.moves {
        let split_at_index = min(m.from, m.to);
        let right_index = ((m.from as i32 - m.to as i32).abs() - 1) as usize;
        let is_rightward = m.to > m.from;
        let start_index = input.crates[m.from as usize - 1].len() - (m.quantity as usize);
        let (mut left, mut right) = input.crates.split_at_mut(split_at_index as usize);
        if is_rightward {
            // move_between_vecs(left.l)
            let mut t = left.last_mut().unwrap().drain(start_index..);
            right[right_index].extend(t);
        } else {
            let mut t = &mut right[right_index].drain(start_index..);
            left.last_mut().unwrap().extend(t);
        }
    }
    input.top_of_stacks()
}

pub fn make_sol() -> StructSolution<Input1, Output1, Input2, Output2> {
    let struct_solution = StructSolution {
        prepare_part_1: prepare,
        calc_part_1: part_1,
        prepare_part_2: prepare,
        calc_part_2: part_2,
        example_part_1: Example::Value("CMZ".to_string()),
        example_part_2: Example::Value("MCD".to_string()),
        day_data: DayData::new(5, false),
    };
    struct_solution
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_move() {
        let test_ins: MoveInstruction = "move 10 from 2 to 3".parse().unwrap();
        assert_eq!(test_ins.quantity, 10);
        assert_eq!(test_ins.from, 2);
        assert_eq!(test_ins.to, 3);
    }

    #[test]
    fn test_part_2() {
        let input = "[A] [B]\n[C] [D]\n\nmove 1 from 1 to 2\nmove 1 from 2 to 1";
        println!("{}", part_2(prepare(input.to_string())));
    }
}

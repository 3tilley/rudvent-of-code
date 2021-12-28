use crate::utils;
use array2d::Array2D;
use std::fmt;

#[derive(Clone)]
pub struct Board {
    width: u32,
    height: u32,
    nums: array2d::Array2D<(u32, bool)>,
    is_complete: bool,
}
impl Board {
    fn new(input_str: &str) -> Self {
        let data: Vec<_> = input_str
            .trim()
            .lines()
            .map(|x| {
                x.split_ascii_whitespace()
                    .map(|s| (s.parse::<u32>().unwrap(), false))
                    .collect()
            })
            .collect();
        let array = array2d::Array2D::from_rows(&data);
        Board {
            width: array.column_len() as u32,
            height: array.row_len() as u32,
            nums: array,
            is_complete: false,
        }
    }
    fn check_line(&self, is_row: bool, i: u32) -> bool {
        let mut is_complete = true;
        if is_row {
            for (num, place) in self.nums.row_iter(i as usize) {
                if !place {
                    is_complete = false;
                }
            }
        } else {
            for (num, place) in self.nums.column_iter(i as usize) {
                if !place {
                    is_complete = false;
                }
            }
        }
        is_complete
    }

    fn cross_num(&mut self, num: u32) -> Option<(u32, u32)> {
        let mut row_col_i: Option<(u32, u32, (u32, bool))> = None;
        for (col_i, col) in self.nums.columns_iter().enumerate() {
            for (row_i, &element) in col.into_iter().enumerate() {
                if element.0 == num {
                    row_col_i = Some((row_i as u32, col_i as u32, element));
                    break;
                }
            }
        }
        match row_col_i {
            None => None,
            Some((row, col, (x, _))) => {
                self.nums
                    .set(row as usize, col as usize, (x, true))
                    .unwrap();
                Some((row, col))
            }
        }
    }
    fn update_and_cross(&mut self, num: u32) -> bool {
        let res = self.cross_num(num);
        let mut is_complete = false;
        match res {
            None => (),
            Some((row, col)) => {
                is_complete = self.check_line(true, row) || self.check_line(false, col);
            }
        }
        self.is_complete = is_complete;
        is_complete
    }
    fn unmarked_sum(&self) -> u32 {
        let mut sum = 0;
        for (el, marked) in self.nums.elements_column_major_iter() {
            if !marked {
                sum += el;
            }
        }
        sum
    }
}

impl fmt::Debug for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut ls = String::new();
        for r in self.nums.rows_iter() {
            for &e in r {
                if e.1 {
                    ls.push_str(" __");
                } else {
                    ls.push_str(&format!(" {:0>2}", &e.0.to_string()));
                }
            }
            ls.push_str("\n");
        }
        write!(f, "{}", ls)
    }
}

pub fn load_data(name: &str) -> (Vec<u32>, Vec<Board>) {
    let contents = utils::read_file(name, file!());
    match contents.split_once("\n") {
        None => panic!("Input file not in expected format"),
        Some((nums, rest)) => {
            let inputs = nums.split(",").map(|x| x.parse::<u32>().unwrap()).collect();
            (inputs, rest.split("\n\n").map(|x| Board::new(x)).collect())
        }
    }
}

pub fn a() -> u32 {
    let (inputs, mut boards) = load_data("input.txt");
    let mut result = None;
    for inp in inputs {
        for board in &mut boards {
            if board.update_and_cross(inp) {
                result = Some(board.unmarked_sum() * inp);
                break;
            }
        }
        match result {
            None => (),
            Some(_) => break,
        }
    }
    result.unwrap()
}

pub fn b(input_file: &str) -> u32 {
    let (inputs, mut boards) = load_data(input_file);
    let mut result = None;
    let mut last_score = None;
    for inp in inputs {
        for board in &mut boards {
            if board.update_and_cross(inp) {
                last_score = Some(board.unmarked_sum());
            }
        }
        boards.retain(|b| !b.is_complete);
        match boards.len() {
            1 => (),
            0 => {
                result = Some(last_score.unwrap() * inp);
                break;
            }
            _ => (),
        }
    }
    result.unwrap()
}

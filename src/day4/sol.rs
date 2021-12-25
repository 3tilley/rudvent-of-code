use array2d::Array2D;
//use std::fmt;
use crate::day3::sol::b;
use crate::utils;

#[derive(Debug)]
struct Board {
    width: u32,
    height: u32,
    nums: array2d::Array2D<(u32, bool)>
}
impl Board {
    fn new(input_str: &str) -> Self {
        let data : Vec<_> = input_str.trim().lines().map(|x| x.split_ascii_whitespace().map(|s| (s.parse::<u32>().unwrap(), false)).collect()).collect();
        let array = array2d::Array2D::from_rows(&data);
        Board { width: array.column_len() as u32, height: array.row_len() as u32, nums: array }
    }
    fn check_line(&self, is_row: bool, i: u32) -> bool {
        let mut is_complete = true;
        if is_row {
            for (num, place) in self.nums.column_iter(i as usize) {
                if !place {
                    is_complete = false;
                }
            }
        } else {
            for (_, place) in self.nums.row_iter(i as usize) {
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
                if element.0== num {
                    row_col_i = Some((row_i as u32, col_i as u32, element));
                    break;
                }
            }
        }
        match row_col_i {
            None => None,
            Some((row, col, (x, _))) => {
                self.nums.set(row as usize, col as usize, (x, true)).unwrap();
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
                 for is_row in vec![true, false] {
                     if is_row {
                         is_complete = self.check_line(true, row);
                     } else {
                         is_complete = self.check_line(false, col);
                     }
                 }
             }
         }
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

pub fn a() -> u32 {
    let contents = utils::read_file("example.txt", file!());
    match contents.split_once("\n") {
        None => panic!("Input file not in expected format"),
        Some((nums, rest)) => {
            let inputs = nums.split(",").map(|x| x.parse::<u32>().unwrap());
            let mut boards: Vec<Board> = rest.split("\n\n").map(|x| Board::new(x)).collect();
            let mut result = None;
            for inp in inputs {
                println!("{}", inp);
                for board in &mut boards {
                    println!("{:?}", &board);
                        if board.update_and_cross(inp) {
                            result = Some(board.unmarked_sum() * inp);
                            break;
                    }
                }
            }
            result.unwrap()
        }
    }
}

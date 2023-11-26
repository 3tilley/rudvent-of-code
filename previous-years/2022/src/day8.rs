use crate::solution::{Example, StructSolution};
use crate::DayData;
use fmt::Display;
use std::cmp::max_by;
use std::fmt;
use std::fmt::Formatter;
use std::iter::once;
use std::slice::Iter;

type Input1 = Forest;
type Output1 = usize;
type Input2 = Forest;
type Output2 = usize;

#[derive(Debug)]
pub struct Forest {
    n: usize,
    // This will be RC
    trees: Vec<Vec<u8>>,
}

impl Display for Forest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.trees {
            for col in row {
                write!(f, "{}", col)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn fmt(vis: &Vec<Vec<bool>>) {
    for row in vis {
        for col in row {
            print!("{}", if *col { '#' } else { '.' });
        }
        println!();
    }
}

impl Forest {
    pub fn iter_through_rows(&self, col: usize, rev: bool) -> Box<dyn Iterator<Item = &u8> + '_> {
        let mut col_iter = self.trees.iter().map(move |row| &row[col]);
        if rev {
            Box::new(col_iter.rev())
        } else {
            Box::new(col_iter)
        }
    }

    pub fn iter_through_cols(&self, row: usize, rev: bool) -> Box<dyn Iterator<Item = &u8> + '_> {
        let mut row_iter = self.trees[row].iter();
        if rev {
            Box::new(row_iter.rev())
        } else {
            Box::new(row_iter)
        }
    }

    fn get_indices(
        &self,
        fixed: usize,
        iter: usize,
        fixed_is_row: bool,
        rev: bool,
    ) -> (usize, usize) {
        let (fixed, iter) = if rev {
            (fixed, self.n - iter - 1)
        } else {
            (fixed, iter)
        };
        if fixed_is_row {
            (fixed, iter)
        } else {
            (iter, fixed)
        }
    }

    pub fn mark_inwards(
        &self,
        vis_map: &mut Vec<Vec<bool>>,
        index: usize,
        is_row: bool,
        rev: bool,
    ) {
        let iter = if is_row {
            self.iter_through_cols(index, rev)
        } else {
            self.iter_through_rows(index, rev)
        };
        let mut highest = None;
        for (i, val) in iter.enumerate() {
            let (first, second) = self.get_indices(index, i, is_row, rev);
            match highest {
                None => {
                    highest = Some(val);
                    vis_map[first][second] = true;
                }
                Some(h) => {
                    if val > h {
                        highest = Some(val);
                        vis_map[first][second] = true;
                    } else {
                        ()
                    }
                }
            }
        }
    }

    pub fn get_visible(&self) -> Vec<Vec<bool>> {
        let mut vis = vec![vec![false; self.n]; self.n];
        for rev in [false, true].iter() {
            for r in 0..self.n {
                self.mark_inwards(&mut vis, r, true, *rev);
            }
            for c in 0..self.n {
                self.mark_inwards(&mut vis, c, false, *rev);
            }
        }
        vis
    }

    pub fn visible_count(&self) -> usize {
        let vis = self.get_visible();
        println!("{:?}", fmt(&vis));
        vis.iter()
            .map(|row| row.iter().filter(|&&v| v).count())
            .sum()
    }

    pub fn scenic_score(&self, row: usize, col: usize) -> usize {
        let current = self.trees[row][col];
        println!("Checking ({} {}). Height: {}", row, col, current);
        let mut score = 0usize;
        let right: usize = (col + 1..)
            .find_map(|c| {
                println!("Right: {} {}", row, c);
                match self.trees[row].get(c) {
                    Some(&v) => {
                        println!("Right found height: {}", v);
                        if v >= current {
                            Some(c - col)
                        } else {
                            None
                        }
                    } // .unwrap_or(Some(self.n - col - 1))
                    None => Some(self.n - col - 1),
                }
            })
            .unwrap_or(0);
        let left = (0..col)
            .rev()
            .find_map(|c| {
                println!("Left: {} {}", row, c);
                match self.trees[row].get(c) {
                    Some(&v) => {
                        println!("Left found height: {}", v);
                        if v >= current {
                            Some(col - c)
                        } else if c == 0 {
                            Some(col)
                        } else {
                            None
                        }
                    }
                    None => Some(col),
                }
            })
            .unwrap_or(0);
        let down = (row + 1..)
            .find_map(|r| {
                println!("Down: {} {}", r, col);
                match self.trees.get(r) {
                    Some(found_row) => {
                        println!("Down found height: {}", found_row[col]);
                        if found_row[col] >= current {
                            Some(r - row)
                        } else {
                            None
                        }
                    }
                    None => Some(self.n - row - 1),
                }
            })
            .unwrap_or(0);
        let up = (0..row)
            .rev()
            .find_map(|r| {
                println!("Up: {} {}", r, col);
                match self.trees.get(r) {
                    Some(found_row) => {
                        println!("Up found height: {}", found_row[col]);
                        if found_row[col] >= current {
                            Some(row - r)
                        } else if r == 0 {
                            Some(row)
                        } else {
                            None
                        }
                    }
                    None => Some(row),
                }
            })
            .unwrap_or(0);
        let score = left * right * up * down;
        if score >= 0 {
            println!(
                "Candidate score {} at ({}, {}): {} {} {} {}",
                score, row, col, left, right, up, down
            );
        }
        score
    }
}

pub fn prepare(input: String) -> Input1 {
    let mut lines = input.lines();
    let mut trees = Vec::new();
    let first_line = lines.next().unwrap();
    let n = first_line.len();
    for r in 0..n {
        trees.push(vec![0u8; n]);
    }
    // for line in once(first_line).chain(input.lines() {
    for (row, line) in input.lines().enumerate() {
        for (col, c) in line.chars().enumerate() {
            trees[row][col] = (c as u8) - 48;
        }
    }
    let f = Forest { n, trees };
    println!("{}", f);
    f
}

pub fn part_1(input: Input1) -> Output1 {
    input.visible_count()
}

pub fn part_2(input: Input2) -> Output2 {
    let mut collec_vec = Vec::new();
    for r in 0..input.n {
        for c in 0..input.n {
            collec_vec.push((input.scenic_score(r, c), r, c));
        }
    }

    println!("{:?}", collec_vec);
    collec_vec
        .iter()
        .max_by(|(a, _, _), (b, _, _)| a.cmp(b))
        .map(|(s, r, c)| *s)
        .unwrap()
}

pub fn make_sol() -> StructSolution<Input1, Output1, Input2, Output2> {
    let struct_solution = StructSolution {
        prepare_part_1: prepare,
        calc_part_1: part_1,
        prepare_part_2: prepare,
        calc_part_2: part_2,
        example_part_1: Example::Value(21),
        example_part_2: Example::Value(8),
        day_data: DayData::new(8, false),
    };
    struct_solution
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iterators() {
        let f = Forest {
            n: 3,
            trees: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        };
        let vec = f.iter_through_rows(0, false).collect::<Vec<_>>();
        assert_eq!(vec, vec![&1, &4, &7]);
    }

    #[test]
    fn test_iterators_rev() {
        let f = Forest {
            n: 3,
            trees: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        };
        let vec = f.iter_through_rows(0, true).collect::<Vec<_>>();
        assert_eq!(vec, vec![&7, &4, &1]);
    }

    #[test]
    fn test_mark_inwards() {
        let f = Forest {
            n: 3,
            trees: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        };
        let mut vis = vec![vec![false; 3]; 3];
        f.mark_inwards(&mut vis, 0, true, true);
        assert_eq!(
            vis,
            vec![
                vec![false, false, true],
                vec![false, false, false],
                vec![false, false, false]
            ]
        );
    }
}

use array2d::Array2D;
use std::collections::HashMap;
use std::fmt::Debug;
use std::iter::once;
use std::marker::PhantomData;

trait Index<T, O>: PartialEq + Eq + Copy + Clone + std::hash::Hash {
    fn in_bounds(&self, lower: &T, higher: &T) -> bool;
    fn default_lower() -> T;
    fn offset(&self, offset: &O, lower: &T, higher: &T) -> Option<T>;
}

impl Index<(usize, usize), (isize, isize)> for (usize, usize) {
    fn in_bounds(&self, lower: &(usize, usize), higher: &(usize, usize)) -> bool {
        (self.0 >= lower.0) && (self.1 >= lower.1) && (self.0 <= higher.0) && (self.1 <= higher.1)
    }
    fn default_lower() -> (usize, usize) {
        (0,0)
    }
    fn offset(&self, offset: &(isize, isize), lower: &(usize, usize), higher: &(usize, usize)) -> Option<(usize, usize)> {
        let new_row =self.0.checked_add_signed(offset.0)?;
        let new_col = self.1.checked_add_signed(offset.1)?;
        if (new_row, new_col).in_bounds(lower, higher) {
            Some((new_row, new_col))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub(crate) struct SparseArray<I: Index<I, O>, T, O> {
    data: HashMap<I, T>,
    pub min_bounds: I,
    pub max_bounds: I,
    default: T,
    _offset: PhantomData<O>
}

impl<I: Index<I, O>, T, O> SparseArray<I, T, O> {
    pub fn new(data: Vec<(I, T)>, max_bounds: I, default: T) -> Result<SparseArray<I, T, O>, I> {
        let min_bounds = I::default_lower();
        for (i,v) in data.iter() {
            if !i.in_bounds(&min_bounds, &max_bounds) {
                return Err(*i)
            }
        }
        let sparse = SparseArray {
            data: HashMap::from_iter(data.into_iter()),
            min_bounds,
            max_bounds,
            _offset: PhantomData,
            default,
        };
        Ok(sparse)
    }

    pub fn offset_index(&self, origin: &I, offset: &O) -> Option<I> {
        origin.offset(&offset, &self.min_bounds, &self.max_bounds)
    }

    pub fn enumerate_iter(&self) -> impl Iterator<Item=(&I, &T)> {
        self.data.iter()
    }

    pub fn get_with_default(&self, index: &I) -> Option<&T> {
        if !index.in_bounds(&self.min_bounds, &self.max_bounds) {
            None
        } else {
            Some(self.data.get(index).unwrap_or(&self.default))
        }
    }

}

pub(crate) trait Array2DExt<T> {
    fn row(&self, index: usize) -> Vec<&T>;
    fn col(&self, index: usize) -> Vec<&T>;

    fn offset_row<V: TryInto<isize>>(&self, index: usize, offset: V) -> Option<usize>
    where
        <V as TryInto<isize>>::Error: Debug;
    fn offset_col<V: TryInto<isize>>(&self, index: usize, offset: V) -> Option<usize>
    where
        <V as TryInto<isize>>::Error: Debug;

    fn from_newline_delimited(s: &str, mapping_func: impl FnMut((usize, usize, char)) -> T + Copy) -> Array2D<T>;

    fn offset<V: TryInto<isize>>(
        &self,
        index: (usize, usize),
        offset: (V, V),
    ) -> Option<(usize, usize)>
    where
        <V as TryInto<isize>>::Error: Debug;

    fn direction_iter<'a, V: TryInto<isize> + Copy>(
        &'a self,
        index: (usize, usize),
        offset: (V, V),
    ) -> impl Iterator<Item = &'a T>
    where
        T: 'a,
        <V as TryInto<isize>>::Error: Debug;
    
    fn is_in_bounds<V: TryInto<isize>>(&self, index: (usize, usize), offset: (V, V)) -> bool
    where
        <V as TryInto<isize>>::Error: Debug;
    
    fn offset_iter(&self, start: (usize, usize), include_diagonal: bool) -> impl Iterator<Item = (i8, i8)>;

    fn to_str(&self, f: impl Fn((usize, usize, &T)) -> String) -> String;

    fn neighbours_iter<'a>(&'a self, start: (usize, usize), include_diagonal: bool) -> impl Iterator<Item = (usize, usize, &'a T)> where T: 'a;
}

impl<T> Array2DExt<T> for Array2D<T> {
    fn row(&self, row_index: usize) -> Vec<&T> {
        self.row_iter(row_index)
            .expect(&format!("Row {} out of bounds", row_index))
            .collect()
    }

    fn col(&self, col_index: usize) -> Vec<&T> {
        self.column_iter(col_index)
            .expect(&format!("Column {} out of bounds", col_index))
            .collect()
    }
    fn offset_row<V: TryInto<isize>>(&self, index: usize, offset: V) -> Option<usize>
    where
        <V as TryInto<isize>>::Error: Debug,
    {
        index
            .checked_add_signed(offset.try_into().unwrap())
            .filter(|row| row < &self.column_len())
    }
    fn offset_col<V: TryInto<isize>>(&self, index: usize, offset: V) -> Option<usize>
    where
        <V as TryInto<isize>>::Error: Debug,
    {
        index
            .checked_add_signed(offset.try_into().unwrap())
            .filter(|col| col < &self.row_len())
    }

    fn from_newline_delimited(s: &str, mut mapping_func: impl FnMut((usize, usize, char)) -> T + Copy) -> Array2D<T> {
        let mut iterator = s.lines().enumerate();
        // TODO: Consider whether this is accurate
        let (_, max_rows) = iterator.size_hint();
        let rows = match max_rows {
            None => {
                let c = iterator.count();
                iterator = s.lines().enumerate();
                c
            }
            Some(c) => c,
        };
        let first = iterator.next().unwrap();
        let cols = first.1.len();
        let mut mapped_iter = once(first)
            .chain(iterator)
            .flat_map(|(row, line)| line.chars().enumerate().map(move |(col, c)| mapping_func((row, col, c))));
        let res = Array2D::from_iter_row_major(mapped_iter, rows, cols).unwrap();
        // assert!(mapped_iter.next().is_none());
        res
    }

    fn offset<V: TryInto<isize>>(
        &self,
        index: (usize, usize),
        offset: (V, V),
    ) -> Option<(usize, usize)>
    where
        <V as TryInto<isize>>::Error: Debug,
    {
        self.offset_row(index.0, offset.0)
            .and_then(|row| self.offset_col(index.1, offset.1).map(|col| (row, col)))
    }

    fn is_in_bounds<V: TryInto<isize>>(&self, index: (usize, usize), offset: (V, V)) -> bool
    where
        <V as TryInto<isize>>::Error: Debug
    {
        let new_row = index.0.checked_add_signed(offset.0.try_into().unwrap());
        let new_col = index.1.checked_add_signed(offset.1.try_into().unwrap());
        new_row.zip(new_col).filter(|(row, col)| (row < &self.row_len()) && (col < &self.column_len())).is_some()
    }

    fn direction_iter<'a, V: TryInto<isize> + Copy>(
        &'a self,
        index: (usize, usize),
        direction: (V, V),
    ) -> impl Iterator<Item = &'a T>
    where
        T: 'a,
        <V as TryInto<isize>>::Error: Debug
    {
        DirectionIter {
            array2d: self,
            start: index,
            direction,
            current: Some(index)
        }
    }

    fn offset_iter(&self, start: (usize, usize), include_diagonal: bool) -> impl Iterator<Item = (i8, i8)>
    {
        OffsetIter::new(self, start, include_diagonal)
    }

    fn to_str(&self, f: impl Fn((usize, usize, &T)) -> String) -> String {
        let mut rows = Vec::new();
        for row in 0..self.row_len() {
            let mut cols = Vec::new();
            for col in 0.. self.column_len() {
                cols.push(f((row, col, self.get(row, col).unwrap())));
            }
            rows.push(cols.join(""));
        }
        rows.join("\n")
    }

    fn neighbours_iter<'a>(&'a self, start: (usize, usize), include_diagonal: bool) -> impl Iterator<Item=(usize, usize, &'a T)> where T: 'a {
        self.offset_iter(start, include_diagonal).map(move |offset| {
            let new_pos = (start.0.checked_add_signed(offset.0.into()).unwrap(), start.1.checked_add_signed(offset.1.into()).unwrap());
            (new_pos.0, new_pos.1, self.get(new_pos.0, new_pos.1).unwrap())
        })
    }
}

struct OffsetIter<'a, T> {
    array2d: &'a Array2D<T>,
    include_diagonal: bool,
    start: (usize, usize),
    current_offset_index: Option<u8>,
    max_index: u8,
}

impl<'a, T> OffsetIter<'a, T> {
    fn new(array2d: &'a Array2D<T>, start: (usize, usize), include_diagonal: bool) -> OffsetIter<'a, T> {
        OffsetIter {
            array2d,
            include_diagonal,
            start,
            current_offset_index: Some(0),
            max_index: if include_diagonal {7} else {3},
        }
    }

    fn next_offset(&mut self) -> Option<(i8, i8)> {
        match self.current_offset_index {
            Some(i) => {
                if i <= self.max_index {
                    let out = if self.include_diagonal {
                        DIAGONAL_OFFSETS[i as usize]
                    } else {
                        NON_DIAGONAL_OFFSETS[i as usize]
                    };
                    if i < self.max_index {
                        self.current_offset_index = Some(i + 1);
                    } else {
                        self.current_offset_index = None;
                    }
                    Some(out)
                } else {
                    None
                }
            },
            None => None,
        }
    }
}

pub static NON_DIAGONAL_OFFSETS: &[(i8, i8)] = &[(-1, 0), (0, 1), (1, 0), (0, -1)];
pub static DIAGONAL_OFFSETS: &[(i8, i8)] = &[(-1, -1), (-1, 0), (-1, 1), (0, 1), (1, 1), (1, 0), (1, -1), (0, -1)];

impl<'a, T> Iterator for OffsetIter<'a, T> {
    type Item = (i8, i8);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(offset) = self.next_offset() {
            if self.array2d.is_in_bounds(self.start, offset) {
                return Some(offset)
            }
        }
        None
    }
}

struct DirectionIter<'a, T, V: TryInto<isize> + Copy>
    where
    <V as TryInto<isize>>::Error: Debug,
{
    array2d: &'a Array2D<T>,
    start: (usize, usize),
    direction: (V, V),
    current: Option<(usize, usize)>,
}

// fn is_in_bounds(array_bounds: (usize, usize), index: (usize, usize)) -> bool {
//     (index.0 > 0) && (index.0 < array_bounds.0) && (index.1 > 0) && (index.1 < array_bounds.1)
// }

impl<'a, T, V: TryInto<isize> + Copy> Iterator for DirectionIter<'a, T, V>
    where
    <V as TryInto<isize>>::Error: Debug,
{
    type Item = &'a T;

    fn next(&mut self) -> Option<&'a T> {
        // println!("{:?}", self.current);
        // if is_in_bounds((self.array2d.row_len(), self.array2d.column_len()), (current.0, current.1)) {
        //     Some()
        // }
        match self.current {
            None => None,
            Some(current) => {
                match self.array2d.get(current.0, current.1) {
                    None => {
                        self.current = None;
                        None
                    },
                    Some(v) => {
                        let new_row = current.0.checked_add_signed(self.direction.0.try_into().unwrap());
                        let new_col = current.1.checked_add_signed(self.direction.1.try_into().unwrap());
                        match (new_row, new_col) {
                            (None, _) | (_, None) => {
                                self.current = None;
                                // (Some(v))
                            }
                            (Some(r), Some(c)) => {
                                self.current = Some((r, c));
                                // Some(v)
                            }
                        }
                        Some(v)
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{convert::identity, ops::Range};

    #[test]
    fn test_array2d_row_offset() {
        let arr = Array2D::filled_with(0, 4, 1);
        assert_eq!(arr.offset_row(3, -1), Some(2));
        assert_eq!(arr.offset_row(0, -1), None);
        assert_eq!(arr.offset_row(3, 1), None);
        assert_eq!(arr.offset_row(2, 1), Some(3));
    }
    #[test]
    fn test_array2d_col_offset() {
        let arr = Array2D::filled_with(0, 1, 4);
        assert_eq!(arr.offset_col(3, -1), Some(2));
        assert_eq!(arr.offset_col(0, -1), None);
        assert_eq!(arr.offset_col(3, 1), None);
        assert_eq!(arr.offset_col(2, 1), Some(3));
    }

    #[test]
    fn test_from_string() {
        let arr = Array2D::from_newline_delimited("abc\ndef", |c| c.2);
        println!("{:?}", arr);
        assert_eq!(arr.num_rows(), 2);
        assert_eq!(arr.num_columns(), 3);
        assert_eq!(arr.get(1, 2), Some('f').as_ref());
    }

    #[test]
    fn test_offset() {
        let arr = Array2D::filled_with(0, 4, 2);
        assert_eq!(arr.offset((0, 0), (1, 1)), Some((1, 1)));
        assert_eq!(arr.offset((0, 0), (0, 1)), Some((0, 1)));
        assert_eq!(arr.offset((0, 0), (1, 0)), Some((1, 0)));
        assert_eq!(arr.offset((1, 1), (-1, -1)), Some((0, 0)));
    }

    #[test]
    fn test_direction_iter() {
        let arr = Array2D::from_row_major(&(0..8).collect::<Vec<_>>(), 4, 2).unwrap();
        let mut it = arr.direction_iter((0,0), (1,1));
        assert_eq!(it.next(), Some(0).as_ref());
        assert_eq!(it.next(), Some(3).as_ref());
        assert_eq!(it.next(), None)
    }

    #[test]
    fn test_offset_iter() {
        let arr = Array2D::filled_with(0.0, 3, 3);
        let mut it = arr.offset_iter((2, 2), false);
        assert_eq!(it.next(), Some((-1, 0)));
        assert_eq!(it.next(), Some((0, -1)));
        assert_eq!(it.next(), None);
    }

    #[test]
    fn test_offset_iter_diag() {
        let arr = Array2D::filled_with(0.0, 3, 3);
        let mut it = arr.offset_iter((0, 0), true);
        assert_eq!(it.next(), Some((0, 1)));
        assert_eq!(it.next(), Some((1, 1)));
        assert_eq!(it.next(), Some((1, 0)));
        assert_eq!(it.next(), None);
    }
}

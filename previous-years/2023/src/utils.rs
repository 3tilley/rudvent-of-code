use array2d::Array2D;
use std::fmt::Debug;
use std::iter::once;

pub(crate) trait Array2DExt<T> {
    fn row(&self, index: usize) -> Vec<&T>;
    fn col(&self, index: usize) -> Vec<&T>;

    fn offset_row<V: TryInto<isize>>(&self, index: usize, offset: V) -> Option<usize>
    where
        <V as TryInto<isize>>::Error: Debug;
    fn offset_col<V: TryInto<isize>>(&self, index: usize, offset: V) -> Option<usize>
    where
        <V as TryInto<isize>>::Error: Debug;

    fn from_newline_delimited(s: &str, mapping_func: fn(char) -> T) -> Array2D<T>;

    fn offset<V: TryInto<isize>>(
        &self,
        index: (usize, usize),
        offset: (V, V),
    ) -> Option<(usize, usize)>
    where
        <V as TryInto<isize>>::Error: Debug;
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

    fn from_newline_delimited(s: &str, mapping_func: fn(char) -> T) -> Array2D<T> {
        let mut iterator = s.lines();
        // TODO: Consider whether this is accurate
        let (_, max_rows) = iterator.size_hint();
        let rows = match max_rows {
            None => {
                let c = iterator.count();
                iterator = s.lines();
                c
            }
            Some(c) => c,
        };
        let first = iterator.next().unwrap();
        let cols = first.len();
        let mut mapped_iter = once(first)
            .chain(iterator)
            .flat_map(|line| line.chars().map(mapping_func));
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::identity;

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
        let arr = Array2D::from_newline_delimited("abc\ndef", identity);
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
}

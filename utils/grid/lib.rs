use std::{
    fmt::{Debug, Display, Formatter, Write},
    io::{BufRead, Cursor},
    iter::repeat_n,
    mem::swap,
    ops::{Add, AddAssign, Sub},
    str::FromStr,
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Coord2d {
    pub x: i64,
    pub y: i64,
}

pub trait AsCoord2d {
    fn to_coord(self) -> Coord2d;
    fn x(&self) -> i64;
    fn y(&self) -> i64;
}

impl<T: AsCoord2d> Sub<T> for &Coord2d {
    type Output = Coord2d;
    fn sub(self, rhs: T) -> Self::Output {
        Coord2d {
            x: self.x() - rhs.x(),
            y: self.y() - rhs.y(),
        }
    }
}

impl<T: AsCoord2d> Add<T> for &Coord2d {
    type Output = Coord2d;
    fn add(self, rhs: T) -> Self::Output {
        Coord2d {
            x: self.x() + rhs.x(),
            y: self.y() + rhs.y(),
        }
    }
}

impl AsCoord2d for Coord2d {
    fn to_coord(self) -> Coord2d {
        self
    }
    fn x(&self) -> i64 {
        self.x
    }
    fn y(&self) -> i64 {
        self.y
    }
}

impl AsCoord2d for &Coord2d {
    fn to_coord(self) -> Coord2d {
        self.to_owned()
    }
    fn x(&self) -> i64 {
        self.x
    }
    fn y(&self) -> i64 {
        self.y
    }
}

impl<T> AsCoord2d for (T, T)
where
    T: Copy + TryInto<i64>,
    <T as TryInto<i64>>::Error: Debug,
{
    fn to_coord(self) -> Coord2d {
        Coord2d {
            x: self.0.try_into().unwrap(),
            y: self.1.try_into().unwrap(),
        }
    }
    fn x(&self) -> i64 {
        self.0.try_into().unwrap()
    }
    fn y(&self) -> i64 {
        self.1.try_into().unwrap()
    }
}

#[derive(Debug)]
pub struct GridRowIter<'a, T> {
    iter: std::slice::Iter<'a, T>,
}

impl<'a, T: Clone + Eq + PartialEq + Debug> GridRowIter<'a, T> {
    fn new(grid: &'a Grid<T>, y: i64) -> Self {
        let iter = grid.data[y as usize * grid.width()..(y as usize + 1) * grid.width()].iter();
        Self { iter }
    }
}

impl<'a, T> Iterator for GridRowIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

#[derive(Debug)]
pub struct GridColIter<'a, T> {
    grid: &'a Grid<T>,
    stride: usize,
    cur: usize,
}

impl<'a, T: Clone + Eq + PartialEq + Debug> GridColIter<'a, T> {
    fn new(grid: &'a Grid<T>, x: i64) -> Self {
        Self {
            grid,
            stride: grid.width(),
            cur: x as usize,
        }
    }
}

impl<'a, T: Clone + Eq + PartialEq + Debug> Iterator for GridColIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        let cur = self.cur;
        self.cur += self.stride;
        if cur < self.grid.data.len() {
            Some(&self.grid.data[cur])
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.grid.height() - 1, Some(self.grid.height() - 1))
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct Grid<T> {
    pub data: Vec<T>,
    width: i64,
}

impl<T: Clone + Eq + PartialEq + Debug> Grid<T> {
    /// Returns a new [Grid] with the same shape (width x height) as `self`, filled with `fill`
    pub fn same_shape<NT: Clone + Eq + PartialEq + Debug>(&self, fill: NT) -> Grid<NT> {
        Grid::with_shape(self.width(), self.height(), fill)
    }
    /// Returns a new [Grid] with the given shape (width x height), filled with `fill`
    pub fn with_shape(width: usize, height: usize, fill: T) -> Self {
        Self {
            data: Vec::from_iter(repeat_n(fill, width * height)),
            width: width as i64,
        }
    }
    pub fn width(&self) -> usize {
        self.width as usize
    }
    pub fn height(&self) -> usize {
        self.data.len() / self.width()
    }
    pub fn pos<C: AsCoord2d>(&self, c: &C) -> i64 {
        c.y() * self.width + c.x()
    }
    pub fn coord(&self, pos: i64) -> Option<(i64, i64)> {
        if pos < 0 || pos >= self.data.len() as i64 {
            None
        } else {
            Some((pos % self.width, pos / self.width))
        }
    }
    fn valid_pos<C: AsCoord2d>(&self, c: &C) -> Option<usize> {
        if c.x() < 0 || c.x() >= self.width {
            return None;
        }
        if c.y() < 0 || c.y() as usize >= self.height() {
            return None;
        }
        let pos = self.pos(c);
        if pos < 0 || pos as usize >= self.data.len() {
            return None;
        }
        self.pos(c).try_into().ok()
    }
    pub fn get<C: AsCoord2d>(&self, c: &C) -> Option<&T> {
        match self.valid_pos(c) {
            Some(pos) => Some(&self.data[pos]),
            None => None,
        }
    }
    pub fn get_mut<C: AsCoord2d>(&mut self, c: &C) -> Option<&mut T> {
        match self.valid_pos(c) {
            Some(pos) => Some(self.data.get_mut(pos).unwrap()),
            None => None,
        }
    }
    pub fn set<C: AsCoord2d>(&mut self, c: &C, mut val: T) -> Option<T> {
        match self.valid_pos(c) {
            Some(pos) => {
                swap(&mut self.data[pos], &mut val);
                Some(val)
            }
            None => None,
        }
    }
    pub fn increment<'a, A, C: AsCoord2d>(&'a mut self, c: &C, i: A) -> Option<&'a T>
    where
        T: AddAssign<A>,
    {
        match self.valid_pos(c) {
            Some(pos) => {
                self.data[pos] += i;
                Some(&self.data[pos])
            }
            None => None,
        }
    }
    pub fn row(&self, y: i64) -> Option<&[T]> {
        if y < self.height() as i64 && y >= 0 {
            Some(&self.data[self.pos(&(0, y)) as usize..self.pos(&(self.width, y)) as usize])
        } else {
            None
        }
    }

    pub fn row_iter(&self, y: i64) -> Option<GridRowIter<T>> {
        if (y as usize) < self.height() {
            Some(GridRowIter::new(self, y))
        } else {
            None
        }
    }

    pub fn col(&self, x: i64) -> Option<Vec<&T>> {
        if let Some(iter) = self.col_iter(x) {
            Some(iter.collect())
        } else {
            None
        }
    }

    pub fn col_iter(&self, x: i64) -> Option<GridColIter<T>> {
        if (x as usize) < self.width() {
            Some(GridColIter::new(self, x))
        } else {
            None
        }
    }

    pub fn find(&self, haystack: &T) -> Option<(i64, i64)> {
        self.coord(
            self.data
                .iter()
                .enumerate()
                .find_map(|(pos, val)| if val == haystack { Some(pos as i64) } else { None })
                .unwrap_or(-1),
        )
    }
    pub fn count(&self, haystack: &T) -> usize {
        self.data.iter().filter(|item| *item == haystack).count()
    }

    pub fn forward_slice<C: AsCoord2d>(&self, start: &C, len: i64) -> Option<&[T]> {
        let pos = (self.valid_pos(start), self.valid_pos(&(start.x() + len - 1, start.y())));
        match pos {
            (Some(pos1), Some(pos2)) => Some(&self.data[pos1..pos2 + 1]),
            _ => None,
        }
    }

    pub fn swap<A: AsCoord2d, B: AsCoord2d>(&mut self, a: A, b: B) {
        if let (Some(a), Some(b)) = (self.valid_pos(&a), self.valid_pos(&b)) {
            self.data.swap(a, b)
        }
    }

    // fn window_compare_impl<const REV: bool>(&self, needle: &[T]) -> Vec<(i64, i64)> {
    //     if (self.width as usize) < needle.len() {
    //         return Vec::new();
    //     }
    //     let mut res = Vec::new();
    //     for y in 0..self.height() as i64 {
    //         let mut windows_tmp = self.row(y).unwrap().windows(needle.len());
    //         let windows = if REV {
    //             windows_tmp.rev()
    //         } else {
    //             windows_tmp
    //         };

    //         res.extend(
    //             windows
    //                 .enumerate()
    //                 .filter_map(|(x, w)| if w == needle { Some((x as i64, y)) } else { None }),
    //         );
    //     }
    //     res
    // }
}

impl<T: BufRead> From<T> for Grid<u8> {
    fn from(input: T) -> Grid<u8> {
        let mut data = Vec::new();
        let mut width = 0;
        for line in input.split(b'\n').map(|i| i.unwrap()) {
            if width == 0 {
                width = line.len() as i64
            } else if line.len() as i64 != width {
                panic!("Grids must have fixed length rows")
            }
            data.extend_from_slice(&line);
        }
        Grid { data, width }
    }
}

// Should be Grid<char>?
impl FromStr for Grid<u8> {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Cursor::new(s).into())
    }
}

// impl<T: Copy + Eq + PartialEq + Display + Debug + Into<char>> Display for Grid<T> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         for y in 0..self.height() {
//             for x in 0..self.width() {
//                 f.write_fmt(format_args!("{}",self.get(x as i64, y as i64).unwrap() as char))?;
//             }
//             f.write_char('\n')?;
//         }
//         f.write_char('\n')
//     }
// }

impl Display for Grid<u8> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width() {
                f.write_fmt(format_args!("{}", *self.get(&(x as i64, y as i64)).unwrap() as char))?;
            }
            f.write_char('\n')?;
        }
        f.write_char('\n')
    }
}

impl Display for Grid<bool> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width() {
                f.write_fmt(format_args!(
                    "{}",
                    match *self.get(&(x as i64, y as i64)).unwrap() {
                        true => '.',
                        false => '#',
                    }
                ))?;
            }
            f.write_char('\n')?;
        }
        f.write_char('\n')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_VECTOR: &[u8] = b"ABCD
EFGH
IJKL
FBCG";

    static TEST_VECTOR_S: &str = "ABCD
EFGH
IJKL
FBCG";

    fn unchecked_load() -> Grid<u8> {
        Grid::from(TEST_VECTOR)
    }

    #[test]
    fn from_string() {
        let grid = unchecked_load();
        assert_eq!(grid.data, "ABCDEFGHIJKLFBCG".as_bytes());
        assert_eq!(
            TEST_VECTOR_S.parse::<Grid<u8>>().unwrap().data,
            "ABCDEFGHIJKLFBCG".as_bytes()
        );
    }

    #[test]
    fn indexing() {
        let grid = unchecked_load();
        assert_eq!(grid.get(&(0, 0)), Some(b'A').as_ref());
        assert_eq!(grid.get(&(3, 3)), Some(b'G').as_ref());
        assert_eq!(grid.get(&(-1, 0)), None);
        assert_eq!(grid.get(&(0, -1)), None);
        assert_eq!(grid.get(&(5, 0)), None);
        assert_eq!(grid.get(&(0, 5)), None);
    }

    #[test]
    fn forward_slice() {
        let grid = unchecked_load();
        assert_eq!(grid.forward_slice(&(0, 0), 2), Some(b"AB".as_slice()));
        assert_eq!(grid.forward_slice(&(2, 0), 2), Some(b"CD".as_slice()));
        assert_eq!(grid.forward_slice(&(2, 0), 3), None);
        assert_eq!(grid.forward_slice(&(0, 2), 4), Some(b"IJKL".as_slice()));
    }

    #[test]
    fn row_iter() {
        let grid = unchecked_load();
        assert_eq!(
            grid.row_iter(2).unwrap().collect::<Vec<_>>(),
            [&b'I', &b'J', &b'K', &b'L']
        );
        assert!(grid.row_iter(-1).is_none());
        assert!(grid.row_iter(4).is_none());
    }

    #[test]
    fn col_iter() {
        let grid = unchecked_load();
        assert_eq!(
            grid.col_iter(2).unwrap().collect::<Vec<_>>(),
            [&b'C', &b'G', &b'K', &b'C']
        );
        assert!(grid.col_iter(-1).is_none());
        assert!(grid.col_iter(4).is_none());
    }

    // #[test]
    // fn window_compare() {
    //     let grid = unchecked_load();
    //     assert_eq!(grid.window_compare(b"IJKL"), &[(0, 2)]);
    //     assert_eq!(grid.window_compare(b"BC"), &[(1, 0), (1, 3)]);
    //     assert_eq!(grid.window_compare(b"LF").len(), 0);
    // }
}

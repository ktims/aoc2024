use std::{
    fmt::{self, Debug, Display, Formatter, Write},
    io::{BufRead, Lines},
};

#[derive(Clone)]
pub struct Grid<T> {
    pub data: Vec<T>,
    width: i64,
}

impl<T: Copy + Eq + PartialEq + Display + Debug> Grid<T> {
    pub fn new(width: i64) -> Self {
        Self {
            data: Vec::new(),
            width,
        }
    }
    pub fn width(&self) -> usize {
        return self.width as usize;
    }
    pub fn height(&self) -> usize {
        return self.data.len() / self.width();
    }
    fn pos(&self, x: i64, y: i64) -> i64 {
        y * self.width + x
    }
    fn coord(&self, pos: i64) -> Option<(i64, i64)> {
        if pos < 0 || pos >= self.data.len() as i64 {
            None
        } else {
            Some((pos % self.width, pos / self.width))
        }
    }
    fn valid_pos(&self, x: i64, y: i64) -> Option<usize> {
        if x < 0 || x >= self.width {
            return None;
        }
        if y < 0 || y >= self.data.len() as i64 / self.width {
            return None;
        }
        let pos = self.pos(x, y);
        if pos < 0 || pos as usize >= self.data.len() {
            return None;
        }
        self.pos(x, y).try_into().ok()
    }
    pub fn get(&self, x: i64, y: i64) -> Option<T> {
        match self.valid_pos(x, y) {
            Some(pos) => Some(self.data[pos]),
            None => None,
        }
    }
    pub fn set(&mut self, x: i64, y: i64, val: T) -> bool {
        match self.valid_pos(x, y) {
            Some(pos) => {
                self.data[pos] = val;
                true
            }
            None => false,
        }
    }
    pub fn row(&self, y: i64) -> Option<&[T]> {
        if y < self.height() as i64 {
            Some(&self.data[self.pos(0, y) as usize..self.pos(self.width, y) as usize])
        } else {
            None
        }
    }

    pub fn find(&self, haystack: T) -> Option<(i64, i64)> {
        self.coord(
            self.data
                .iter()
                .enumerate()
                .find_map(|(pos, val)| if *val == haystack { Some(pos as i64) } else { None })
                .unwrap_or(-1),
        )
    }
    pub fn count(&self, haystack: T) -> usize {
        self.data.iter().filter(|item| **item == haystack).count()
    }

    pub fn forward_slice(&self, x: i64, y: i64, len: i64) -> Option<&[T]> {
        let pos = (self.valid_pos(x, y), self.valid_pos(x + len - 1, y));
        match pos {
            (Some(pos1), Some(pos2)) => Some(&self.data[pos1..pos2 + 1]),
            _ => None,
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

impl<T: BufRead> From<Lines<T>> for Grid<u8> {
    fn from(input: Lines<T>) -> Grid<u8> {
        let mut data = Vec::new();
        let mut width = 0;
        for line in input.map(|i| i.unwrap()) {
            if width == 0 {
                width = line.len() as i64
            } else if line.len() as i64 != width {
                panic!("Grids must have fixed length rows")
            }
            data.extend_from_slice(line.as_bytes());
        }
        Grid { data, width }
    }
}

impl<T: Copy + Eq + PartialEq + Display + Debug> Display for Grid<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width() {
                self.get(x as i64, y as i64).fmt(f)?;
            }
        }
        f.write_char('\n')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_VECTOR: &str = &"ABCD
EFGH
IJKL
FBCG";

    fn unchecked_load() -> Grid<u8> {
        Grid::from(Cursor::new(TEST_VECTOR).lines())
    }

    #[test]
    fn from_string() {
        let grid = unchecked_load();
        assert_eq!(grid.data, "ABCDEFGHIJKLFBCG".as_bytes());
    }

    #[test]
    fn indexing() {
        let grid = unchecked_load();
        assert_eq!(grid.get(0, 0), Some(b'A'));
        assert_eq!(grid.get(3, 3), Some(b'G'));
        assert_eq!(grid.get(-1, 0), None);
        assert_eq!(grid.get(0, -1), None);
        assert_eq!(grid.get(5, 0), None);
        assert_eq!(grid.get(0, 5), None);
    }

    #[test]
    fn forward_slice() {
        let grid = unchecked_load();
        assert_eq!(grid.forward_slice(0, 0, 2), Some(b"AB".as_slice()));
        assert_eq!(grid.forward_slice(2, 0, 2), Some(b"CD".as_slice()));
        assert_eq!(grid.forward_slice(2, 0, 3), None);
        assert_eq!(grid.forward_slice(0, 2, 4), Some(b"IJKL".as_slice()));
    }

    #[test]
    fn window_compare() {
        let grid = unchecked_load();
        assert_eq!(grid.window_compare(b"IJKL"), &[(0, 2)]);
        assert_eq!(grid.window_compare(b"BC"), &[(1, 0), (1, 3)]);
        assert_eq!(grid.window_compare(b"LF").len(), 0);
    }
}

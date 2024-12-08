fn main() {
    println!("Hello, world!");
}

fn read_to_str(mut r: impl std::io::Read) -> std::io::Result<String> {
    let mut buffer = String::new();
    r.read_to_string(&mut buffer)?;
    Ok(buffer)
}

struct Grid {
    buf: String,
    width: usize,
}

impl Grid {
    fn new(s: String) -> Self {
        let width = match s.as_str().find('\n') {
            Some(w) => w,
            None => s.len(),
        };
        Grid {
            buf: s,
            width: width,
        }
    }

    fn index_to_row_col(&self, ind: usize) -> (isize, isize) {
        ((ind / self.width) as isize, (ind % self.width) as isize)
    }

    fn char_at_row_col(&self, row: isize, col: isize) -> Option<u8> {
        let ind = row * (self.width as isize) + col;
        if ind < 0 {
            return None;
        }
        self.buf.as_bytes().get(ind as usize).copied()
    }

    fn iters_from_index(&self, ind: usize) -> Vec<GridIter> {
        let (row, col) = self.index_to_row_col(ind);
        self.iters_from_row_col(row, col)
    }

    fn iters_from_row_col(&self, row: isize, col: isize) -> Vec<GridIter> {
        vec![
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
            (-1, -1),
            (0, -1),
            (1, -1),
        ]
        .iter()
        .map(|dir| GridIter {
            grid: self,
            curr_row: row,
            curr_col: col,
            direction: *dir,
        })
        .collect()
    }

    fn count_occurrences(&self, string: &str) -> usize {
        let Some(first) = string.bytes().next() else {
            return 0;
        };
        self.buf
            .bytes()
            .enumerate()
            .filter(|(_, b)| *b == first) // only include grid indices which match the first char
            .map(|(i, _)| self.iters_from_index(i)) // create vec of iters from each matching index
            .flatten() // flatten that iter of vecs of iters into an iter of iters
            .map(|iter| iter.take(string.len()).collect::<Vec<u8>>()) // only look at the first N chars
            .filter(|v| v == string.as_bytes()) // only include those which match
            .count() // count them
    }
}

struct GridIter<'a> {
    grid: &'a Grid,
    curr_row: isize,
    curr_col: isize,
    direction: (isize, isize),
}

impl<'a> Iterator for GridIter<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        let char_here = self.grid.char_at_row_col(self.curr_row, self.curr_col);
        self.curr_row += self.direction.0;
        self.curr_row += self.direction.1;
        char_here
    }
}

#[cfg(test)]
mod tests {
    const EXAMPLE_INPUT: &str = "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn test_count_occurrences_xmas() {
        let grid = super::Grid::new(EXAMPLE_INPUT.into());
        let result = grid.count_occurrences("XMAS");
        assert_eq!(result, 18);
    }
}

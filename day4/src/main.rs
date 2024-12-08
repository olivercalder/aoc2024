fn main() {
    println!("Hello, world!");
}

fn read_to_str(mut r: impl std::io::Read) -> std::io::Result<String> {
    let mut buffer = String::new();
    r.read_to_string(&mut buffer)?;
    Ok(buffer)
}

struct Grid {
    lines: Vec<Vec<u8>>,
    width: usize,
}

impl Grid {
    fn new(s: String) -> Self {
        let lines = string_to_lines(s);
        let width = lines[0].len();
        Grid {
            lines: lines,
            width: width,
        }
    }

    fn char_at_row_col(&self, row: isize, col: isize) -> Option<&u8> {
        if row < 0 || col < 0 {
            return None;
        }
        let Some(r) = self.lines.get(row as usize) else {
            return None;
        };
        r.get(col as usize)
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
        self.lines.iter().enumerate() // get row
            .map(|(row, line)| indices_of_match_starts(line, first).into_iter().map(move |col| (row.clone() as isize, col)))
            .flatten() // flatten iter of iter of (row, col) into iter of (row, col)
            .map(|(row, col)| self.iters_from_row_col(row, col)) // iter of vecs of iters
            .flatten() // flatten that iter of vecs of iters into an iter of iters
            .map(|iter| iter.take(string.len()).collect::<Vec<u8>>())
            .filter(|v| v == string.as_bytes())
            .count()
    }
}

fn string_to_lines(s: String) -> Vec<Vec<u8>> {
    s.split('\n').map(|l| l.as_bytes().into()).collect()
}

fn indices_of_match_starts(line: &[u8], first: u8) -> Vec<isize> {
    line.iter().enumerate().filter(|(_, x)| **x == first).map(|(i, _)| i as isize).collect()
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
        char_here.copied()
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

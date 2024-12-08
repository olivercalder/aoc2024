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
}

impl Grid {
    fn new(s: String) -> Self {
        let lines = string_to_lines(s);
        Grid {
            lines,
        }
    }

    fn char_at_row_col(&self, row: isize, col: isize) -> Option<u8> {
        if row < 0 || col < 0 {
            return None;
        }
        let Some(r) = self.lines.get(row as usize) else {
            return None;
        };
        r.get(col as usize).copied()
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
    s.trim().split('\n').map(|l| l.as_bytes().into()).collect()
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

impl Iterator for GridIter<'_> {
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
    fn test_string_to_lines() {
        let lines = super::string_to_lines(EXAMPLE_INPUT.into());
        let expected: Vec<Vec<u8>> = vec![
            "MMMSXXMASM".as_bytes().to_vec(),
            "MSAMXMSMSA".as_bytes().to_vec(),
            "AMXSXMAAMM".as_bytes().to_vec(),
            "MSAMASMSMX".as_bytes().to_vec(),
            "XMASAMXAMM".as_bytes().to_vec(),
            "XXAMMXXAMA".as_bytes().to_vec(),
            "SMSMSASXSS".as_bytes().to_vec(),
            "SAXAMASAAA".as_bytes().to_vec(),
            "MAMMMXMMMM".as_bytes().to_vec(),
            "MXMXAXMASX".as_bytes().to_vec(),
        ];
        assert_eq!(lines, expected);
    }

    #[test]
    fn test_indices_of_match_starts() {
        assert_eq!(super::indices_of_match_starts("MMMSXXMASM".as_bytes(), b'X'), vec![4, 5]);
        assert_eq!(super::indices_of_match_starts("MSAMXMSMSA".as_bytes(), b'X'), vec![4]);
        assert_eq!(super::indices_of_match_starts("AMXSXMAAMM".as_bytes(), b'X'), vec![2, 4]);
        assert_eq!(super::indices_of_match_starts("MSAMASMSMX".as_bytes(), b'X'), vec![9]);
        assert_eq!(super::indices_of_match_starts("XMASAMXAMM".as_bytes(), b'X'), vec![0, 6]);
        assert_eq!(super::indices_of_match_starts("XXAMMXXAMA".as_bytes(), b'X'), vec![0, 1, 5, 6]);
        assert_eq!(super::indices_of_match_starts("SMSMSASXSS".as_bytes(), b'X'), vec![7]);
        assert_eq!(super::indices_of_match_starts("SAXAMASAAA".as_bytes(), b'X'), vec![2]);
        assert_eq!(super::indices_of_match_starts("MAMMMXMMMM".as_bytes(), b'X'), vec![5]);
        assert_eq!(super::indices_of_match_starts("MXMXAXMASX".as_bytes(), b'X'), vec![1, 3, 5, 9]);
    }

    #[test]
    fn test_count_occurrences_xmas() {
        let grid = super::Grid::new(EXAMPLE_INPUT.into());
        let result = grid.count_occurrences("XMAS");
        assert_eq!(result, 18);
    }
}

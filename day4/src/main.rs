fn main() -> std::io::Result<()> {
    let input = read_to_str(std::io::stdin().lock())?;
    let grid = Grid::new(input);
    let count_xmases = grid.count_occurrences("XMAS");
    let count_x_mases = grid.count_x_mas_occurrences();
    println!("XMASes: {}", count_xmases);
    println!("X-MASes: {}", count_x_mases);
    Ok(())
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
        Grid { lines }
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

    fn find_coords_of(&self, first: u8) -> impl Iterator<Item = (isize, isize)> + use<'_> {
        self.lines
            .iter()
            .enumerate() // get row index
            .map(move |(row, line)| {
                find_indices_of(line, first)
                    .into_iter()
                    .map(move |col| (row.clone() as isize, col))
            })
            .flatten() // flatten iter of iter of (row, col) into iter of (row, col)
    }

    fn iters_from_row_col(&self, row: isize, col: isize) -> Vec<GridIter> {
        vec![
            (0, 1),   // right
            (1, 1),   // right-down
            (1, 0),   // down
            (1, -1),  // down-left
            (0, -1),  // left
            (-1, -1), // left-up
            (-1, 0),  // up
            (-1, 1),  // up-right
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
        self.find_coords_of(first)
            .map(|(row, col)| self.iters_from_row_col(row, col)) // iter of vecs of iters
            .flatten() // flatten that iter of vecs of iters into an iter of iters
            .map(|iter| iter.bytes(string.len()))
            .filter(|v| v == string.as_bytes())
            .count()
    }

    fn count_x_mas_occurrences(&self) -> usize {
        self.find_coords_of(b'A')
            .map(|(row, col)| {
                vec![
                    GridIter {
                        grid: self,
                        curr_row: row - 1,
                        curr_col: col - 1,
                        direction: (1, 1),
                    }
                    .bytes(3),
                    GridIter {
                        grid: self,
                        curr_row: row + 1,
                        curr_col: col - 1,
                        direction: (-1, 1),
                    }
                    .bytes(3),
                ]
            })
            .filter(|x_bytes| {
                x_bytes
                    .iter()
                    .all(|bytes| bytes == "MAS".as_bytes() || bytes == "SAM".as_bytes())
            })
            .count()
    }
}

fn string_to_lines(s: String) -> Vec<Vec<u8>> {
    s.trim().split('\n').map(|l| l.as_bytes().into()).collect()
}

fn find_indices_of(line: &[u8], first: u8) -> Vec<isize> {
    line.iter()
        .enumerate() // get col index
        .filter(|(_, x)| **x == first)
        .map(|(i, _)| i as isize)
        .collect()
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
        self.curr_col += self.direction.1;
        char_here
    }
}

impl GridIter<'_> {
    fn bytes(self, len: usize) -> Vec<u8> {
        self.take(len).collect::<Vec<u8>>()
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
    fn test_find_indices_of() {
        assert_eq!(
            super::find_indices_of("MMMSXXMASM".as_bytes(), b'X'),
            vec![4, 5]
        );
        assert_eq!(
            super::find_indices_of("MSAMXMSMSA".as_bytes(), b'X'),
            vec![4]
        );
        assert_eq!(
            super::find_indices_of("AMXSXMAAMM".as_bytes(), b'X'),
            vec![2, 4]
        );
        assert_eq!(
            super::find_indices_of("MSAMASMSMX".as_bytes(), b'X'),
            vec![9]
        );
        assert_eq!(
            super::find_indices_of("XMASAMXAMM".as_bytes(), b'X'),
            vec![0, 6]
        );
        assert_eq!(
            super::find_indices_of("XXAMMXXAMA".as_bytes(), b'X'),
            vec![0, 1, 5, 6]
        );
        assert_eq!(
            super::find_indices_of("SMSMSASXSS".as_bytes(), b'X'),
            vec![7]
        );
        assert_eq!(
            super::find_indices_of("SAXAMASAAA".as_bytes(), b'X'),
            vec![2]
        );
        assert_eq!(
            super::find_indices_of("MAMMMXMMMM".as_bytes(), b'X'),
            vec![5]
        );
        assert_eq!(
            super::find_indices_of("MXMXAXMASX".as_bytes(), b'X'),
            vec![1, 3, 5, 9]
        );
    }

    #[test]
    fn test_find_coords_of() {
        let grid = super::Grid::new(EXAMPLE_INPUT.into());
        let coords: Vec<(isize, isize)> = grid.find_coords_of(b'X').collect();
        assert_eq!(
            coords,
            vec![
                (0, 4),
                (0, 5),
                (1, 4),
                (2, 2),
                (2, 4),
                (3, 9),
                (4, 0),
                (4, 6),
                (5, 0),
                (5, 1),
                (5, 5),
                (5, 6),
                (6, 7),
                (7, 2),
                (8, 5),
                (9, 1),
                (9, 3),
                (9, 5),
                (9, 9),
            ]
        )
    }

    #[test]
    fn test_iters_from_row_col() {
        let grid = super::Grid::new(EXAMPLE_INPUT.into());
        let strs: Vec<String> = grid
            .iters_from_row_col(1, 2)
            .into_iter()
            .map(|iter| String::from_utf8(iter.bytes(4)).unwrap())
            .collect();
        assert_eq!(
            strs,
            vec!["AMXM", "ASAM", "AXAA", "AMM", "ASM", "AM", "AM", "AS",],
        );
    }

    #[test]
    fn test_count_occurrences_xmas() {
        let grid = super::Grid::new(EXAMPLE_INPUT.into());
        let result = grid.count_occurrences("XMAS");
        assert_eq!(result, 18);
    }

    #[test]
    fn test_count_x_mas_occurrences() {
        let grid = super::Grid::new(EXAMPLE_INPUT.into());
        let result = grid.count_x_mas_occurrences();
        assert_eq!(result, 9);
    }
}

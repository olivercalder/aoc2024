use std::collections::BTreeSet;

fn main() -> std::io::Result<()> {
    let input = read_to_str(std::io::stdin().lock())?;
    let grid = Grid::new(input);
    let count = grid.count_positions();
    println!("{}", count);
    Ok(())
}

fn read_to_str(mut r: impl std::io::Read) -> std::io::Result<String> {
    let mut buffer = String::new();
    r.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn string_to_lines(s: String) -> Vec<Vec<u8>> {
    s.trim().split('\n').map(|l| l.as_bytes().into()).collect()
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

    /// find_start returns the (row, col, direction) of the start.
    fn find_start(&self) -> (isize, isize, (isize, isize)) {
        for (row, r_contents) in self.lines.iter().enumerate() {
            for (col, x) in r_contents.iter().enumerate() {
                match x {
                    b'^' => return (row as isize, col as isize, (-1, 0)),
                    b'>' => return (row as isize, col as isize, (0, 1)),
                    b'v' => return (row as isize, col as isize, (1, 0)),
                    b'<' => return (row as isize, col as isize, (0, -1)),
                    _ => continue,
                }
            }
        }
        panic!("cannot find start");
    }

    fn count_positions(&self) -> usize {
        let (row, col, direction) = self.find_start();
        let mut positions = GridIter::new(&self, row, col, direction).collect::<BTreeSet<(isize, isize)>>();
        positions.insert((row, col));
        positions.len()
    }
}

struct GridIter<'a> {
    grid: &'a Grid,
    curr_row: isize,
    curr_col: isize,
    curr_dir: (isize, isize),
}

impl<'a> GridIter<'a> {
    fn new(grid: &Grid, row: isize, col: isize, direction: (isize, isize)) -> GridIter {
        GridIter {
            grid,
            curr_row: row,
            curr_col: col,
            curr_dir: direction,
        }
    }
}

impl Iterator for GridIter<'_> {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<(isize, isize)> {
        loop {
            let next_row = self.curr_row + self.curr_dir.0;
            let next_col = self.curr_col + self.curr_dir.1;
            let next_char = self.grid.char_at_row_col(next_row, next_col);
            match next_char {
                Some(b'#') => self.curr_dir = next_direction(self.curr_dir),
                Some(_) => {
                    self.curr_row = next_row;
                    self.curr_col = next_col;
                    return Some((self.curr_row, self.curr_col));
                }
                None => return None,
            }
        }
    }
}

fn next_direction(direction: (isize, isize)) -> (isize, isize) {
    (direction.1, -direction.0)
}

#[cfg(test)]
mod tests {
    const EXAMPLE_INPUT: &str = "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_string_to_lines() {
        let lines = crate::string_to_lines(EXAMPLE_INPUT.into());
        let expected: Vec<Vec<u8>> = vec![
            "....#.....".as_bytes().to_vec(),
            ".........#".as_bytes().to_vec(),
            "..........".as_bytes().to_vec(),
            "..#.......".as_bytes().to_vec(),
            ".......#..".as_bytes().to_vec(),
            "..........".as_bytes().to_vec(),
            ".#..^.....".as_bytes().to_vec(),
            "........#.".as_bytes().to_vec(),
            "#.........".as_bytes().to_vec(),
            "......#...".as_bytes().to_vec(),
        ];
        assert_eq!(lines, expected);
    }

    #[test]
    fn test_find_start() {
        let grid = crate::Grid::new(EXAMPLE_INPUT.into());
        let start = grid.find_start();
        assert_eq!(start, (6, 4, (-1, 0)));
    }

    #[test]
    fn test_next_direction() {
        assert_eq!(crate::next_direction((0, 1)), (1, 0));
        assert_eq!(crate::next_direction((1, 0)), (0, -1));
        assert_eq!(crate::next_direction((0, -1)), (-1, 0));
        assert_eq!(crate::next_direction((-1, 0)), (0, 1));
    }

    #[test]
    fn count_positions() {
        let grid = crate::Grid::new(EXAMPLE_INPUT.into());
        assert_eq!(grid.count_positions(), 41)
    }
}

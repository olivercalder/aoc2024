use std::collections::BTreeSet;

fn main() -> std::io::Result<()> {
    let input = read_to_str(std::io::stdin().lock())?;
    let grid = Grid::new(input);
    let positions = grid.count_positions();
    let obstacle_placements = grid.count_obstacle_placements();
    println!("positions: {}", positions);
    println!("obstacle placements: {}", obstacle_placements);
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
        self.get_positions(true).len()
    }

    fn get_positions(&self, include_start: bool) -> BTreeSet<(isize, isize)> {
        let (row, col, direction) = self.find_start();
        let mut positions = GridIter::new(&self, row, col, direction)
            .map(|(pos, _)| pos)
            .collect::<BTreeSet<(isize, isize)>>();
        if include_start {
            positions.insert((row, col));
        }
        positions
    }

    fn count_obstacle_placements(&self) -> usize {
        let (start_row, start_col, start_dir) = self.find_start();
        self.get_positions(false).into_iter().filter(|pos| GridIter::new_with_obstacle(self, start_row, start_col, start_dir, *pos).has_cycle()).count()
    }
}

struct GridIter<'a> {
    grid: &'a Grid,
    curr_row: isize,
    curr_col: isize,
    curr_dir: (isize, isize),
    obstacle: Option<(isize, isize)>,
}

impl<'a> GridIter<'a> {
    fn new(grid: &Grid, row: isize, col: isize, direction: (isize, isize)) -> GridIter {
        GridIter {
            grid,
            curr_row: row,
            curr_col: col,
            curr_dir: direction,
            obstacle: None,
        }
    }

    fn new_with_obstacle(grid: &Grid, row: isize, col: isize, direction: (isize, isize), obstacle: (isize, isize)) -> GridIter {
        GridIter {
            grid,
            curr_row: row,
            curr_col: col,
            curr_dir: direction,
            obstacle: Some(obstacle),
        }
    }

    fn has_cycle(mut self) -> bool {
        let mut position_directions: BTreeSet<((isize, isize), (isize, isize))> = BTreeSet::new();
        while let Some(pos_dir) = self.next() {
            if !position_directions.insert(pos_dir) {
                return true;
            }
        }
        false
    }
}

impl Iterator for GridIter<'_> {
    type Item = ((isize, isize), (isize, isize));

    fn next(&mut self) -> Option<((isize, isize), (isize, isize))> {
        loop {
            let next_row = self.curr_row + self.curr_dir.0;
            let next_col = self.curr_col + self.curr_dir.1;
            if Some((next_row, next_col)) == self.obstacle {
                self.curr_dir = next_direction(self.curr_dir);
                continue;
            }
            let next_char = self.grid.char_at_row_col(next_row, next_col);
            match next_char {
                Some(b'#') => self.curr_dir = next_direction(self.curr_dir),
                Some(_) => {
                    self.curr_row = next_row;
                    self.curr_col = next_col;
                    return Some(((self.curr_row, self.curr_col), self.curr_dir));
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
    fn test_count_positions() {
        let grid = crate::Grid::new(EXAMPLE_INPUT.into());
        assert_eq!(grid.count_positions(), 41)
    }

    #[test]
    fn test_count_obstacle_placements() {
        let grid = crate::Grid::new(EXAMPLE_INPUT.into());
        assert_eq!(grid.count_obstacle_placements(), 6)
    }
}

fn main() {
    let (left, right) = sorted_cols(std::io::stdin().lock());
    let distance = total_distance(&left, &right);
    println!("total distance: {}", distance);
}

fn sorted_cols(r: impl std::io::BufRead) -> (Vec<isize>, Vec<isize>) {
    let (mut left, mut right): (Vec<isize>, Vec<isize>) = r.lines()
                        .map_while(Result::ok)
                        .filter(|line| !line.is_empty())
                        .map(|line| line_to_tuple(line.as_str()))
                        .unzip();
    left.sort_unstable();
    right.sort_unstable();
    (left, right)
}

fn line_to_tuple(line: &str) -> (isize, isize) {
    let mut nums = line.split_whitespace().map(|s| s.parse::<isize>().expect("failed to parse number"));
    let pair: (isize, isize) = (nums.next().unwrap(), nums.next().unwrap());
    pair
}

fn total_distance(left: &[isize], right: &[isize]) -> isize {
    left.iter()
        .zip(right.iter())
        .map(|(x, y)| (x - y).abs())
        .sum()
}

#[cfg(test)]
mod tests {
    const EXAMPLE_INPUT: &str = "
3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_sorted_cols() {
        let test_input = std::io::BufReader::new(EXAMPLE_INPUT.as_bytes());
        let (left, right) = super::sorted_cols(test_input);
        assert_eq!(left, vec![1, 2, 3, 3, 3, 4]);
        assert_eq!(right, vec![3, 3, 3, 4, 5, 9]);
    }

    #[test]
    fn test_line_to_tuple() {
        assert_eq!(super::line_to_tuple("3   4"), (3, 4));
        assert_eq!(super::line_to_tuple("4   3"), (4, 3));
        assert_eq!(super::line_to_tuple("2   5"), (2, 5));
        assert_eq!(super::line_to_tuple("1   3"), (1, 3));
        assert_eq!(super::line_to_tuple("3   9"), (3, 9));
        assert_eq!(super::line_to_tuple("3   3"), (3, 3));
    }

    #[test]
    fn test_example_distance() {
        let test_input = std::io::BufReader::new(EXAMPLE_INPUT.as_bytes());
        let (left, right) = super::sorted_cols(test_input);
        let result = super::total_distance(&left, &right);
        let expected = 11; // given expected answer
        assert_eq!(result, expected)
    }
}

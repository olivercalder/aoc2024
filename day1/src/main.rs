fn main() {
    let (left, right) = sorted_cols(std::io::stdin().lock());
    let distance = total_distance(&left, &right);
    let similarity = similarity_score(&left, &right);
    println!("total distance: {}", distance);
    println!("similarity score: {}", similarity);
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

fn similarity_score(left: &[isize], right: &[isize]) -> isize {
    let mut total: isize = 0; // Grand total.
    let mut prev_left: isize = -1; // Assume there's no -1 in the list.
    let mut prev_sum: isize = 0; // If next left is the same, re-add prev_sum.
    let r_iter = right.iter().peekable();
    let mut right_done = false;
    for x in left {
        if *x == prev_left {
            total += prev_sum;
            continue;
        }
        if right_done {
            break;
        }
        r_iter = r_iter.skip_while(|y| **y < *x).peekable();
        let mut new_sum: isize = 0;
        loop {
            let Some(y) = r_iter.peek() else {
                right_done = false;
                break;
            };
            if **y != *x {
                continue;
            }
            new_sum += **y;
            let _ = r_iter.next(); // consume y
        }
        total += new_sum;
        prev_sum = new_sum;
    }
    total
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

    #[test]
    fn test_example_similarity() {
        let test_input = std::io::BufReader::new(EXAMPLE_INPUT.as_bytes());
        let (left, right) = super::sorted_cols(test_input);
        let result = super::similarity_score(&left, &right);
        let expected = 31; // given expected answer
        assert_eq!(result, expected)
    }
}

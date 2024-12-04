use std::collections::BTreeMap;

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
    let r_counts = counts(right.iter().cloned());
    left.iter().fold(0, |acc, x| if let Some(y) = r_counts.get(x) { acc + (x * y) } else { acc } )
}

fn counts(it: impl Iterator<Item = isize>) -> BTreeMap<isize, isize> {
    let mut m = BTreeMap::new();
    it.for_each(|x| { m.entry(x).and_modify(|curr| *curr += 1).or_insert(1); } );
    m
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
    fn test_counts() {
        let input_1: Vec<isize> = vec![3, 4, 2, 1, 3, 3];
        let result_1 = super::counts(input_1.iter().cloned());
        assert_eq!(result_1.len(), 4);
        assert_eq!(result_1[&1], 1);
        assert_eq!(result_1[&2], 1);
        assert_eq!(result_1[&3], 3);
        assert_eq!(result_1[&4], 1);
        let input_2 = vec![4, 3, 5, 3, 9, 3];
        let result_2 = super::counts(input_2.iter().cloned());
        assert_eq!(result_2.len(), 4);
        assert_eq!(result_2[&3], 3);
        assert_eq!(result_2[&4], 1);
        assert_eq!(result_2[&5], 1);
        assert_eq!(result_2[&9], 1);
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

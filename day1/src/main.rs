fn main() {
    let distance = total_distance(std::io::stdin().lock());
    println!("{}", distance)
}

fn total_distance(r: impl std::io::BufRead) -> isize {
    let (mut left, mut right): (Vec<isize>, Vec<isize>) = r.lines()
                        .filter_map(|line| line.ok())
                        .filter(|line| line.len() > 0)
                        .map(|line| line_to_tuple(line.as_str()))
                        .unzip();
    left.sort_unstable();
    right.sort_unstable();
    left.into_iter()
        .zip(right.into_iter())
        .map(|(x, y)| (x - y).abs())
        .sum()
}

fn line_to_tuple(line: &str) -> (isize, isize) {
    let mut nums = line.split_whitespace().map(|s| s.parse::<isize>().expect("failed to parse number"));
    let pair: (isize, isize) = (nums.next().unwrap(), nums.next().unwrap());
    pair
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        let test_input = std::io::BufReader::new("
3   4
4   3
2   5
1   3
3   9
3   3".as_bytes());
        let result = super::total_distance(test_input);
        let expected = 11; // given expected answer
        assert_eq!(result, expected)
    }
}

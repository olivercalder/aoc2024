fn main() {
    let safe_reports = count_safe_reports(reports(std::io::stdin().lock()));
    println!("Safe reports: {}", safe_reports);
}

fn reports(r: impl std::io::BufRead) -> impl Iterator<Item = Vec<isize>> {
    r.lines()
        .map_while(Result::ok)
        .filter(|line| !line.is_empty())
        .map(|line| line.split_whitespace().map(|s| s.parse().expect("failed to parse number")).collect())
}

/// Returns true if the report is "safe". A report is "safe" if the numbers are either all
/// increasing or all decreasing, and any two adjacent numbers differ by at least one and at most
/// three.
fn safe(report: &Vec<isize>) -> bool {
    let mut pairs = report.windows(2);
    let first = pairs.next().unwrap();
    let range = match first[1] - first[0] {
        1..=3 => 1..=3,
        -3..=-1 => -3..=-1,
        _ => return false,
    };
    for pair in pairs {
        if !range.contains(&(pair[1] - pair[0])) {
            return false;
        }
    }
    true
}

fn count_safe_reports(reports: impl Iterator<Item = Vec<isize>>) -> usize {
    reports.filter(safe).count()
}

#[cfg(test)]
mod tests {
    const EXAMPLE_INPUT: &str = "
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_reports() {
        let test_input = std::io::BufReader::new(EXAMPLE_INPUT.as_bytes());
        let result: Vec<Vec<isize>> = super::reports(test_input).collect();
        let expected: Vec<Vec<isize>> = vec![
            vec![7, 6, 4, 2, 1],
            vec![1, 2, 7, 8, 9],
            vec![9, 7, 6, 2, 1],
            vec![1, 3, 2, 4, 5],
            vec![8, 6, 4, 4, 1],
            vec![1, 3, 6, 7, 9],
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_safe() {
        assert_eq!(super::safe(&vec![7, 6, 4, 2, 1]), true);
        assert_eq!(super::safe(&vec![1, 2, 7, 8, 9]), false);
        assert_eq!(super::safe(&vec![9, 7, 6, 2, 1]), false);
        assert_eq!(super::safe(&vec![1, 3, 2, 4, 5]), false);
        assert_eq!(super::safe(&vec![8, 6, 4, 4, 1]), false);
        assert_eq!(super::safe(&vec![1, 3, 6, 7, 9]), true);
    }

    #[test]
    fn test_count_safe_reports() {
        let test_input = std::io::BufReader::new(EXAMPLE_INPUT.as_bytes());
        let result = super::count_safe_reports(super::reports(test_input));
        assert_eq!(result, 2)
    }
}

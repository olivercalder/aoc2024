fn main() {
    let safe_reports = count_safe_reports(reports(std::io::stdin().lock()));
    println!("Safe reports: {}", safe_reports.0);
    println!("Safe reports after dampener: {}", safe_reports.1);
}

fn reports(r: impl std::io::BufRead) -> impl Iterator<Item = Vec<isize>> {
    r.lines()
        .map_while(Result::ok)
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().expect("failed to parse number"))
                .collect()
        })
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

/// Returns true if the report is "safe" after at most one entry has been removed from the report.
fn dampener_safe(report: &Vec<isize>) -> bool {
    // For simplicity (since removal of the first or second level could change the (in|de)creasing
    // direction), simply run through the report twice, once checking increasing, once decreasing.
    dampener_safe_for_range(report, 1..=3) || dampener_safe_for_range(report, -3..=-1)
}

fn dampener_safe_for_range(report: &Vec<isize>, range: std::ops::RangeInclusive<isize>) -> bool {
    let mut already_removed = false;
    let mut triples = report.windows(3);
    while let Some(triple) = triples.next() {
        match (range.contains(&(triple[1] - triple[1])), range.contains(&(triple[2] - triple[1]))) {
            (true, true) => continue,
            (false, false) => {
                if already_removed {
                    return false;
                }
                already_removed = true;
                // Check if it's safe after removing the middle entry
                if !range.contains(&(triple[2] - triple[0])) {
                    return false;
                }
                _ = triples.next(); // Two back-to-back
            }
            (true, false) | (false, true) => {
                if already_removed {
                    return false;
                }
                already_removed = true;
            }
        };
    }
    return true;
}

/// Returns the number of reports which are immediately safe, and the number of reports which are
/// safe after at most one entry has been removed.
fn count_safe_reports(reports: impl Iterator<Item = Vec<isize>>) -> (usize, usize) {
    let counts = reports.fold(vec![0, 0], |mut acc, report| {
        if safe(&report) {
            acc[0] += 1;
            acc[1] += 1;
        } else if dampener_safe(&report) {
            acc[1] += 1;
        }
        acc
    });
    (counts[0], counts[1])
}

#[cfg(test)]
mod tests {
    const EXAMPLE_INPUT: &str = "
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
4 3 6 7 9
1 3 6 7 6"; // final two cases test behavior when safe once first/last is removed

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
            vec![4, 3, 6, 7, 9],
            vec![1, 3, 6, 7, 6],
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
        assert_eq!(super::safe(&vec![4, 3, 6, 7, 9]), false);
        assert_eq!(super::safe(&vec![1, 3, 6, 7, 6]), false);
    }

    #[test]
    fn test_count_safe_reports() {
        let test_input = std::io::BufReader::new(EXAMPLE_INPUT.as_bytes());
        let result = super::count_safe_reports(super::reports(test_input));
        assert_eq!(result.0, 2);
        assert_eq!(result.1, 6);
    }
}

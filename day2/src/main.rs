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
    let Some(first) = pairs.next() else { return true };
    let range = match first[1] - first[0] {
        1..=3 => 1..=3,
        -3..=-1 => -3..=-1,
        _ => return false,
    };
    for pair in pairs {
        if !pair_safe(pair, &range) {
            return false;
        }
    }
    true
}

fn pair_safe(pair: &[isize], range: &std::ops::RangeInclusive<isize>) -> bool {
    range.contains(&(pair[1] - pair[0]))
}

/// Returns true if the report is "safe" after at most one entry has been removed from the report.
fn dampener_safe(report: &Vec<isize>) -> bool {
    // For simplicity (since removal of the first or second level could change the (in|de)creasing
    // direction), simply run through the report twice, once checking increasing, once decreasing.
    dampener_safe_for_range(report, 1..=3) || dampener_safe_for_range(report, -3..=-1)
}

fn dampener_safe_for_range(report: &Vec<isize>, range: std::ops::RangeInclusive<isize>) -> bool {
    let mut already_removed = false;
    let mut pairs = report.windows(2).enumerate().peekable();
    while let Some((i, pair)) = pairs.next() {
        if pair_safe(pair, &range) {
            continue;
        }
        if already_removed {
            return false;
        }
        already_removed = true;
        // See if we're safe if we skip the next item
        let Some((_, next_pair)) = pairs.peek() else {
            // If we skip the next item, we're done, so report is safe.
            return true
        };
        if pair_safe(&[pair[0], next_pair[1]], &range) {
            continue;
        }
        // Can't skip the next item either. Only hope is if this is the first pair, and we can skip
        // the first item in that pair.
        if i == 0 {
            continue;
        }
        return false
    }
    true
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
        test_both_directions(super::safe, vec![7, 6, 4, 2, 1], true);
        test_both_directions(super::safe, vec![1, 2, 7, 8, 9], false);
        test_both_directions(super::safe, vec![9, 7, 6, 2, 1], false);
        test_both_directions(super::safe, vec![1, 3, 2, 4, 5], false);
        test_both_directions(super::safe, vec![8, 6, 4, 4, 1], false);
        test_both_directions(super::safe, vec![1, 3, 6, 7, 9], true);
        test_both_directions(super::safe, vec![4, 3, 6, 7, 9], false);
        test_both_directions(super::safe, vec![1, 3, 6, 7, 6], false);
        test_both_directions(super::safe, vec![1], true);
    }

    fn test_both_directions(safe_fn: impl Fn(&Vec<isize>) -> bool, mut report: Vec<isize>, safe: bool) {
        assert_eq!(safe_fn(&report), safe);
        report.reverse();
        assert_eq!(safe_fn(&report), safe);
    }

    #[test]
    fn test_dampener_safe() {
        test_both_directions(super::dampener_safe, vec![7, 6, 4, 2, 1], true);
        test_both_directions(super::dampener_safe, vec![1, 2, 7, 8, 9], false);
        test_both_directions(super::dampener_safe, vec![9, 7, 6, 2, 1], false);
        test_both_directions(super::dampener_safe, vec![1, 3, 2, 4, 5], true);
        test_both_directions(super::dampener_safe, vec![8, 6, 4, 4, 1], true);
        test_both_directions(super::dampener_safe, vec![1, 3, 6, 7, 9], true);
        test_both_directions(super::dampener_safe, vec![4, 3, 6, 7, 9], true);
        test_both_directions(super::dampener_safe, vec![1, 3, 6, 7, 6], true);
        test_both_directions(super::dampener_safe, vec![1], true);
    }

    #[test]
    fn test_count_safe_reports() {
        let test_input = std::io::BufReader::new(EXAMPLE_INPUT.as_bytes());
        let result = super::count_safe_reports(super::reports(test_input));
        assert_eq!(result.0, 2);
        assert_eq!(result.1, 4);
    }
}

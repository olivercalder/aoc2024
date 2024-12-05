fn main() {
    let safe_reports = count_safe_reports(reports(std::io::stdin().lock()));
    println!("Safe reports: {}", safe_reports.0);
    println!("Safe reports after dampener: {}", safe_reports.1);
    println!("Safe reports after dampener (brute forced): {}", safe_reports.2);
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
    let mut pairs = report.windows(2).peekable();
    let mut can_skip_current = true;
    while let Some(pair) = pairs.next() {
        let safe = pair_safe(pair, &range);
        if !safe {
            if already_removed {
                return false;
            }
            already_removed = true;
        }
        // See if we're safe if we skip the next item
        let Some(next_pair) = pairs.peek() else {
            // If we skip the next item, we're done, so report is safe.
            return true
        };
        let can_skip_next = pair_safe(&[pair[0], next_pair[1]], &range);
        match (safe, can_skip_next, can_skip_current) {
            (true, _, _) => can_skip_current = can_skip_next,
            (false, true, _) => {
                // skip next (the new current), using up our one skip
                _ = pairs.next();
                can_skip_current = false; // use skip next (the new current), using it up
            }
            (false, false, true) => can_skip_current = false, // can't skip anymore anyway
            (false, false, false) => return false,
        }
    }
    true
}

fn brute_force_dampener_safe(report: &Vec<isize>) -> bool {
    for i in 0..report.len() {
        let (left, right) = report.split_at(i);
        let mut joined = Vec::from(left);
        joined.extend_from_slice(&right[1..]);
        if safe(&joined) {
            return true
        }
    }
    false
}

/// Returns the number of reports which are immediately safe, and the number of reports which are
/// safe after at most one entry has been removed, computed directly or by brute force.
fn count_safe_reports(reports: impl Iterator<Item = Vec<isize>>) -> (usize, usize, usize) {
    let counts = reports.fold(vec![0, 0, 0], |mut acc, report| {
        if safe(&report) {
            acc[0] += 1;
            acc[1] += 1;
            acc[2] += 1;
            if !dampener_safe(&report) {
                println!("WARNING: dampener_safe missed a report which is already safe: {:?}", report);
            }
        } else {
            match (dampener_safe(&report), brute_force_dampener_safe(&report)) {
                (true, true) => {
                    acc[1] += 1;
                    acc[2] += 1;
                }
                (true, false) => {
                    acc[1] += 1;
                    println!("WARNING: dampener_safe incorrectly says report is safe: {:?}", report);
                }
                (false, true) => {
                    acc[2] += 1;
                    println!("WARNING: dampener_safe missed a report which is safe after dampening: {:?}", report);
                }
                (false, false) => {},
            }
        }
        acc
    });
    (counts[0], counts[1], counts[2])
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
    fn test_dampener_safe_tough_cases() {
        assert_eq!(super::dampener_safe(&vec![82, 83, 84, 81, 86]), true);
        assert_eq!(super::dampener_safe(&vec![76, 74, 71, 69, 67, 68, 64]), true);
        assert_eq!(super::dampener_safe(&vec![79, 80, 83, 81, 82]), true);
        assert_eq!(super::dampener_safe(&vec![28, 30, 33, 36, 42, 39]), true);
        assert_eq!(super::dampener_safe(&vec![85, 83, 80, 82, 78]), true);
        assert_eq!(super::dampener_safe(&vec![16, 13, 11, 8, 9, 8]), true);
        assert_eq!(super::dampener_safe(&vec![63, 60, 66, 69, 72, 73]), true);
    }

    #[test]
    fn test_count_safe_reports() {
        let test_input = std::io::BufReader::new(EXAMPLE_INPUT.as_bytes());
        let result = super::count_safe_reports(super::reports(test_input));
        assert_eq!(result.0, 2);
        assert_eq!(result.1, 4);
    }
}

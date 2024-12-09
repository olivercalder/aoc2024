use std::collections::BTreeMap;

fn main() {
    let middle_page_sum = sum_of_valid_update_middle_pages(std::io::stdin().lock());
    println!("sum of middle pages: {}", middle_page_sum);
}

fn sum_of_valid_update_middle_pages(r: impl std::io::BufRead) -> usize {
    let (rules, updates) = get_rules_and_updates(r);
    sum_of_middles(updates.iter().filter(|upd| update_is_valid(upd, &rules)))
}

// Rules are X|Y where X must come before Y (if X and Y are both present) and updates are a map
// from number to position in the list of numbers.
fn get_rules_and_updates(r: impl std::io::BufRead) -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let mut lines = r
        .lines()
        .map_while(Result::ok)
        .skip_while(|line| line.is_empty());
    let rules = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let mut nums = line
                .split('|')
                .map(|s| s.parse().expect("failed to parse number"));
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .collect();
    let updates = lines
        .by_ref()
        .skip_while(|line| line.is_empty())
        .take_while(|line| !line.is_empty())
        .map(|line| {
            line.split(',')
                .map(|s| s.parse().expect("failed to parse number"))
                .collect()
        })
        .collect();
    (rules, updates)
}

fn update_is_valid(update: &[usize], rules: &[(usize, usize)]) -> bool {
    let update_map = update_to_map(update);
    for (x, y) in rules {
        let (Some(x_ind), Some(y_ind)) = (update_map.get(x), update_map.get(y)) else {
            continue;
        };
        if x_ind >= y_ind {
            return false;
        }
    }
    true
}

fn update_to_map(update: &[usize]) -> BTreeMap<usize, usize> {
    BTreeMap::from_iter(update.iter().enumerate().map(|(i, x)| (*x, i)))
}

fn sum_of_middles<'a>(updates: impl Iterator<Item = &'a Vec<usize>>) -> usize {
    updates.map(|upd| upd[upd.len() / 2]).sum()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    const EXAMPLE_INPUT: &str = "
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_get_rules_and_updates() {
        let test_input = std::io::BufReader::new(EXAMPLE_INPUT.as_bytes());
        let (rules, updates) = super::get_rules_and_updates(test_input);
        assert_eq!(
            rules,
            [
                (47, 53),
                (97, 13),
                (97, 61),
                (97, 47),
                (75, 29),
                (61, 13),
                (75, 53),
                (29, 13),
                (97, 29),
                (53, 29),
                (61, 53),
                (97, 53),
                (61, 29),
                (47, 13),
                (75, 47),
                (97, 75),
                (47, 61),
                (75, 61),
                (47, 29),
                (75, 13),
                (53, 13),
            ]
        );
        assert_eq!(
            updates,
            vec![
                vec![75, 47, 61, 53, 29],
                vec![97, 61, 53, 29, 13],
                vec![75, 29, 13],
                vec![75, 97, 47, 61, 53],
                vec![61, 13, 29],
                vec![97, 13, 75, 29, 47],
            ]
        );
    }

    #[test]
    fn test_update_is_valid() {
        let test_input = std::io::BufReader::new(EXAMPLE_INPUT.as_bytes());
        let (rules, updates) = super::get_rules_and_updates(test_input);
        let valid_updates: Vec<Vec<usize>> = updates
            .into_iter()
            .filter(|upd| crate::update_is_valid(upd, &rules))
            .collect();
        assert_eq!(
            valid_updates,
            vec![
                vec![75, 47, 61, 53, 29],
                vec![97, 61, 53, 29, 13],
                vec![75, 29, 13]
            ]
        )
    }

    #[test]
    fn test_update_to_map() {
        assert_eq!(
            crate::update_to_map(&[75, 47, 61, 53, 29]),
            BTreeMap::from([(75, 0), (47, 1), (61, 2), (53, 3), (29, 4)])
        );
        assert_eq!(
            crate::update_to_map(&[97, 61, 53, 29, 13]),
            BTreeMap::from([(97, 0), (61, 1), (53, 2), (29, 3), (13, 4)])
        );
        assert_eq!(
            crate::update_to_map(&[75, 29, 13]),
            BTreeMap::from([(75, 0), (29, 1), (13, 2)])
        );
        assert_eq!(
            crate::update_to_map(&[75, 97, 47, 61, 53]),
            BTreeMap::from([(75, 0), (97, 1), (47, 2), (61, 3), (53, 4)])
        );
        assert_eq!(
            crate::update_to_map(&[61, 13, 29]),
            BTreeMap::from([(61, 0), (13, 1), (29, 2)])
        );
        assert_eq!(
            crate::update_to_map(&[97, 13, 75, 29, 47]),
            BTreeMap::from([(97, 0), (13, 1), (75, 2), (29, 3), (47, 4)])
        );
    }

    #[test]
    fn test_sum_of_middles() {
        let valid_updates: Vec<Vec<usize>> = vec![
            vec![75, 47, 61, 53, 29],
            vec![97, 61, 53, 29, 13],
            vec![75, 29, 13],
        ];
        let sum = crate::sum_of_middles(valid_updates.iter());
        assert_eq!(sum, 61 + 53 + 29);
    }

    #[test]
    fn test_sum_of_valid_update_middle_pages() {
        let test_input = std::io::BufReader::new(EXAMPLE_INPUT.as_bytes());
        let result = crate::sum_of_valid_update_middle_pages(test_input);
        assert_eq!(result, 143);
    }
}

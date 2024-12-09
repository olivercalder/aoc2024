use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let (sum, fixed_sum) = sum_of_update_middle_pages(std::io::stdin().lock());
    println!("sum of valid middle pages: {}", sum);
    println!("sum of fixed middle pages: {}", fixed_sum);
}

fn sum_of_update_middle_pages(r: impl std::io::BufRead) -> (usize, usize) {
    let (rules, updates) = get_rules_and_updates(r);
    let rule_map = rules_to_map(rules);
    let (valid, invalid): (Vec<Vec<usize>>, Vec<Vec<usize>>) = updates
        .into_iter()
        .partition(|upd| update_is_valid(upd, &rule_map));
    let fixed = invalid.into_iter().map(|inv| correct_order(inv, &rule_map));
    (sum_of_middles(valid.into_iter()), sum_of_middles(fixed))
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

/// Convert from X|Y to a map from X: {Y, ...} where all Y in the value may not occur before X in
/// an update.
fn rules_to_map(rules: Vec<(usize, usize)>) -> BTreeMap<usize, BTreeSet<usize>> {
    let mut map: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::new();
    for (x, y) in rules {
        map.entry(x).or_insert(BTreeSet::new()).insert(y);
    }
    map
}

fn update_is_valid(update: &[usize], rules: &BTreeMap<usize, BTreeSet<usize>>) -> bool {
    let update_map = update_to_map(update);
    for (x, ys) in rules {
        for y in ys {
            let (Some(x_ind), Some(y_ind)) = (update_map.get(x), update_map.get(y)) else {
                continue;
            };
            if x_ind >= y_ind {
                return false;
            }
        }
    }
    true
}

fn update_to_map(update: &[usize]) -> BTreeMap<usize, usize> {
    BTreeMap::from_iter(update.iter().enumerate().map(|(i, x)| (*x, i)))
}

fn sum_of_middles(updates: impl Iterator<Item = Vec<usize>>) -> usize {
    updates.map(|upd| upd[upd.len() / 2]).sum()
}

fn correct_order(update: Vec<usize>, rules: &BTreeMap<usize, BTreeSet<usize>>) -> Vec<usize> {
    let update_len = update.len();
    update
        .into_iter()
        .fold(Vec::with_capacity(update_len), |mut acc, num| {
            let Some(must_precede) = rules.get(&num) else {
                acc.push(num);
                return acc;
            };
            match acc.iter().position(|x| must_precede.contains(x)) {
                Some(pos) => acc.insert(pos, num),
                None => acc.push(num),
            }
            acc
        })
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

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
    fn test_rules_to_map() {
        let rules = vec![
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
        ];
        let map = crate::rules_to_map(rules);
        let expected: BTreeMap<usize, BTreeSet<usize>> = BTreeMap::from([
            (47, BTreeSet::from([53, 13, 61, 29])),
            (97, BTreeSet::from([13, 61, 47, 29, 53, 75])),
            (75, BTreeSet::from([29, 53, 47, 61, 13])),
            (61, BTreeSet::from([13, 53, 29])),
            (29, BTreeSet::from([13])),
            (53, BTreeSet::from([29, 13])),
        ]);
        assert_eq!(map, expected);
    }

    #[test]
    fn test_update_is_valid() {
        let test_input = std::io::BufReader::new(EXAMPLE_INPUT.as_bytes());
        let (rules, updates) = crate::get_rules_and_updates(test_input);
        let rule_map = crate::rules_to_map(rules);
        let valid_updates: Vec<Vec<usize>> = updates
            .into_iter()
            .filter(|upd| crate::update_is_valid(upd, &rule_map))
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
        let sum = crate::sum_of_middles(valid_updates.into_iter());
        assert_eq!(sum, 61 + 53 + 29);
    }

    #[test]
    fn test_sum_of_valid_update_middle_pages() {
        let test_input = std::io::BufReader::new(EXAMPLE_INPUT.as_bytes());
        let (result, _) = crate::sum_of_update_middle_pages(test_input);
        assert_eq!(result, 143);
    }

    #[test]
    fn test_sum_of_fixed_update_middle_pages() {
        let test_input = std::io::BufReader::new(EXAMPLE_INPUT.as_bytes());
        let (_, result) = crate::sum_of_update_middle_pages(test_input);
        assert_eq!(result, 123);
    }
}

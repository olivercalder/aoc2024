fn main() {
    let sum = sum_of_valid_equations(get_number_vecs(std::io::stdin().lock()));
    println!("sum: {}", sum.0);
    println!("sum with concatenation: {}", sum.1);
}

fn get_number_vecs(r: impl std::io::BufRead) -> impl Iterator<Item = (usize, Vec<usize>)> {
    r.lines()
        .map_while(Result::ok)
        .skip_while(|line| line.is_empty())
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (first, rest) = line.trim().split_once(':').unwrap();
            let total: usize = first.parse().expect("failed to parse number");
            let nums: Vec<usize> = rest
                .split_whitespace()
                .map(|s| s.parse().expect("failed to parse number"))
                .collect();
            (total, nums)
        })
}

enum Concat {
    Never,
    Later,
    Now,
}

impl Concat {
    fn next(&self) -> Concat {
        match self {
            Concat::Never => Concat::Never,
            Concat::Later => Concat::Now,
            Concat::Now => Concat::Now,
        }
    }
}

fn nums_total_up(target: usize, nums: &[usize]) -> (bool, bool) {
    let Some((&first, rest)) = nums.split_first() else {
        return (false, false);
    };
    if remaining_nums_total_up(target, first, rest, Concat::Never) {
        return (true, true);
    }
    for n in 2..=nums.len() {
        let (first, rest) = concatenate_next_n_split(nums, n);
        // Don't try to concatenate the next element, since we're going to try concatenating every
        // element until we hit the first non-concatenation.
        if remaining_nums_total_up(target, first, rest, Concat::Later) {
            return (false, true);
        }
    }
    (false, false)
}

fn remaining_nums_total_up(target: usize, current: usize, nums: &[usize], concatenate: Concat) -> bool {
    let Some((&first, rest)) = nums.split_first() else {
        return target == current;
    };
    let sum = current + first;
    if sum <= target && remaining_nums_total_up(target, sum, rest, concatenate.next()) {
        return true;
    }
    let product = current * first;
    if product <= target && remaining_nums_total_up(target, product, rest, concatenate.next()) {
        return true;
    }
    match concatenate {
        Concat::Never | Concat::Later => false,
        Concat::Now => remaining_nums_total_up_after_concatenating(target, current, nums)
    }
}

fn remaining_nums_total_up_after_concatenating(target: usize, current: usize, nums: &[usize]) -> bool {
    for n in 2..=nums.len() {
        let (first, rest) = concatenate_next_n_split(nums, n);
        let sum = current + first;
        if sum <= target && remaining_nums_total_up(target, sum, rest, Concat::Now) {
            return true;
        }
        let product = current * first;
        if product <= target && remaining_nums_total_up(target, product, rest, Concat::Now) {
            return true;
        }
    }
    false
}

fn concatenate_next_n_split(nums: &[usize], n: usize) -> (usize, &[usize]) {
    let (left, right) = nums.split_at(n);
    (left.iter().cloned().reduce(concatenate_nums).expect("tried to split empty slice"), right)
}

fn concatenate_nums(a: usize, b: usize) -> usize {
    a * 10usize.pow(b.ilog10() + 1) + b
}

fn sum_of_valid_equations(eqs: impl Iterator<Item = (usize, Vec<usize>)>) -> (usize, usize) {
    eqs.fold((0, 0), |(sum, sum_with), (target, nums)| {
        match nums_total_up(target, &nums) {
            (true, _) => (sum + target, sum_with + target),
            (false, true) => (sum, sum_with + target),
            _ => (sum, sum_with),
        }
    })
}

#[cfg(test)]
mod tests {
    const EXAMPLE_INPUT: &str = "
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_get_number_vecs() {
        let test_input = std::io::BufReader::new(EXAMPLE_INPUT.as_bytes());
        let mut num_vecs = crate::get_number_vecs(test_input);
        assert_eq!(num_vecs.next(), Some((190, vec![10, 19])));
        assert_eq!(num_vecs.next(), Some((3267, vec![81, 40, 27])));
        assert_eq!(num_vecs.next(), Some((83, vec![17, 5])));
        assert_eq!(num_vecs.next(), Some((156, vec![15, 6])));
        assert_eq!(num_vecs.next(), Some((7290, vec![6, 8, 6, 15])));
        assert_eq!(num_vecs.next(), Some((161011, vec![16, 10, 13])));
        assert_eq!(num_vecs.next(), Some((192, vec![17, 8, 14])));
        assert_eq!(num_vecs.next(), Some((21037, vec![9, 7, 18, 13])));
        assert_eq!(num_vecs.next(), Some((292, vec![11, 6, 16, 20])));
        assert_eq!(num_vecs.next(), None);
    }

    #[test]
    fn test_nums_total_up() {
        assert_eq!(crate::nums_total_up(190, &[10, 19]).0, true);
        assert_eq!(crate::nums_total_up(3267, &[81, 40, 27]).0, true);
        assert_eq!(crate::nums_total_up(83, &[17, 5]).0, false);
        assert_eq!(crate::nums_total_up(156, &[15, 6]).0, false);
        assert_eq!(crate::nums_total_up(7290, &[6, 8, 6, 15]).0, false);
        assert_eq!(crate::nums_total_up(161011, &[16, 10, 13]).0, false);
        assert_eq!(crate::nums_total_up(192, &[17, 8, 14]).0, false);
        assert_eq!(crate::nums_total_up(21037, &[9, 7, 18, 13]).0, false);
        assert_eq!(crate::nums_total_up(292, &[11, 6, 16, 20]).0, true);
        // The case which is erroneously "correct" if you pretend there's a leading 0
        assert_eq!(crate::nums_total_up(103, &[3, 1, 1, 5, 98]).0, false);
    }

    #[test]
    fn test_nums_total_up_with_concatenation() {
        assert_eq!(crate::nums_total_up(190, &[10, 19]).1, true);
        assert_eq!(crate::nums_total_up(3267, &[81, 40, 27]).1, true);
        assert_eq!(crate::nums_total_up(83, &[17, 5]).1, false);
        assert_eq!(crate::nums_total_up(156, &[15, 6]).1, true);
        assert_eq!(crate::nums_total_up(7290, &[6, 8, 6, 15]).1, true);
        assert_eq!(crate::nums_total_up(161011, &[16, 10, 13]).1, false);
        assert_eq!(crate::nums_total_up(192, &[17, 8, 14]).1, true);
        assert_eq!(crate::nums_total_up(21037, &[9, 7, 18, 13]).1, false);
        assert_eq!(crate::nums_total_up(292, &[11, 6, 16, 20]).1, true);
        // The case which is erroneously "correct" if you pretend there's a leading 0
        assert_eq!(crate::nums_total_up(103, &[3, 1, 1, 5, 98]).1, false);
    }

    #[test]
    fn test_sum_of_valid_equations() {
        let test_input = std::io::BufReader::new(EXAMPLE_INPUT.as_bytes());
        let result = crate::sum_of_valid_equations(crate::get_number_vecs(test_input));
        assert_eq!(result.0, 3749);
    }

    #[test]
    fn test_sum_of_valid_equations_with_concatenation() {
        let test_input = std::io::BufReader::new(EXAMPLE_INPUT.as_bytes());
        let result = crate::sum_of_valid_equations(crate::get_number_vecs(test_input));
        assert_eq!(result.1, 11387);
    }

    #[test]
    fn test_concatenate_next_n_split() {
        let expected: (usize, &[usize]) = (156, &[]);
        assert_eq!(crate::concatenate_next_n_split(&[15, 6], 2), expected);
        let nums = &[7, 219, 8, 44, 2, 9, 3, 2, 6, 4, 1, 6];
        let expected: Vec<(usize, &[usize])> = vec![
            (7, &[219, 8, 44, 2, 9, 3, 2, 6, 4, 1, 6]),
            (7219, &[8, 44, 2, 9, 3, 2, 6, 4, 1, 6]),
            (72198, &[44, 2, 9, 3, 2, 6, 4, 1, 6]),
            (7219844, &[2, 9, 3, 2, 6, 4, 1, 6]),
            (72198442, &[9, 3, 2, 6, 4, 1, 6]),
            (721984429, &[3, 2, 6, 4, 1, 6]),
            (7219844293, &[2, 6, 4, 1, 6]),
            (72198442932, &[6, 4, 1, 6]),
            (721984429326, &[4, 1, 6]),
            (7219844293264, &[1, 6]),
            (72198442932641, &[6]),
            (721984429326416, &[]),
        ];
        for n in 0..expected.len() {
            assert_eq!(crate::concatenate_next_n_split(nums, n+1), expected[n]);
        }
    }

    #[test]
    fn test_concatenate_nums() {
        assert_eq!(crate::concatenate_nums(123, 100), 123100);
        assert_eq!(crate::concatenate_nums(100, 123), 100123);
        assert_eq!(crate::concatenate_nums(1234, 99999), 123499999);
        assert_eq!(crate::concatenate_nums(99999, 1234), 999991234);
    }
}

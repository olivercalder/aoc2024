fn main() {
    let sum = sum_of_valid_equations(get_number_vecs(std::io::stdin().lock()));
    println!("{}", sum)
}

fn get_number_vecs(r: impl std::io::BufRead) -> impl Iterator<Item = (usize, Vec<usize>)> {
    r.lines()
        .map_while(Result::ok)
        .skip_while(|line| line.is_empty())
        .take_while(|line| !line.is_empty())
        .map(|line| {
            let (first, rest) = line.trim().split_once(':').unwrap();
            let total: usize = first.parse().expect("failed to parse number");
            let nums: Vec<usize> = rest.trim().split_whitespace().map(|s| s.parse().expect("failed to parse number")).collect();
            (total, nums)
        })
}

fn nums_total_up(target: usize, nums: &[usize]) -> bool {
    remaining_nums_total_up(target, 0, &nums)
}

fn remaining_nums_total_up(target: usize, current: usize, nums: &[usize]) -> bool {
    let Some((first, rest)) = nums.split_first() else {
        return target == current
    };
    let sum = current + *first;
    if sum <= target && remaining_nums_total_up(target, sum, rest) {
        return true
    }
    let product = current * *first;
    if product <= target && remaining_nums_total_up(target, product, rest) {
        return true
    }
    false
}

fn sum_of_valid_equations(eqs: impl Iterator<Item = (usize, Vec<usize>)>) -> usize {
    eqs.filter(|(target, nums)| nums_total_up(*target, nums)).inspect(|x| println!("{:?}", x)).map(|(target, _)| target).sum()
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
        assert_eq!(crate::nums_total_up(190, &[10, 19]), true);
        assert_eq!(crate::nums_total_up(3267, &[81, 40, 27]), true);
        assert_eq!(crate::nums_total_up(83, &[17, 5]), false);
        assert_eq!(crate::nums_total_up(156, &[15, 6]), false);
        assert_eq!(crate::nums_total_up(7290, &[6, 8, 6, 15]), false);
        assert_eq!(crate::nums_total_up(161011, &[16, 10, 13]), false);
        assert_eq!(crate::nums_total_up(192, &[17, 8, 14]), false);
        assert_eq!(crate::nums_total_up(21037, &[9, 7, 18, 13]), false);
        assert_eq!(crate::nums_total_up(292, &[11, 6, 16, 20]), true);
    }

    #[test]
    fn test_sum_of_valid_equations() {
        let test_input = std::io::BufReader::new(EXAMPLE_INPUT.as_bytes());
        let result = crate::sum_of_valid_equations(crate::get_number_vecs(test_input));
        assert_eq!(result, 3749);
    }
}

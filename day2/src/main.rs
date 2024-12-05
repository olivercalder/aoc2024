fn main() {
    println!("Hello, world!");
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
    fn test_count_safe_reports() {
        let test_input = std::io::BufReader::new(EXAMPLE_INPUT.as_bytes());
        let result = super::count_safe_reports(super::reports(test_input));
        assert_eq!(result, 2)
    }
}

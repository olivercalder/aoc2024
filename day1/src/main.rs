fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example() {
        let test_input = "
3   4
4   3
2   5
1   3
3   9
3   3";
        let result = total_distance(test_input)?;
        let expected = 11; // given expected answer
        assert_eq!(result, expected)
    }
}

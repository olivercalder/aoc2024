use regex::Regex;
use std::io::BufRead;

fn main() -> std::io::Result<()> {
    let mut buffer = String::new();
    std::io::stdin().lock().read_line(&mut buffer)?;
    let result = sum_muls(&buffer);
    println!("{}", result);
    Ok(())
}

fn sum_muls(s: &str) -> usize {
    let re = Regex::new(r"mul\((?<x>[0-9]+),(?<y>[0-9]+)\)").unwrap();
    re.captures_iter(s)
        .map(|caps| {
            let x: usize = caps.name("x").unwrap().as_str().parse().unwrap();
            let y: usize = caps.name("y").unwrap().as_str().parse().unwrap();
            x * y
        })
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sum_muls() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = super::sum_muls(input);
        assert_eq!(result, 161);
    }
}

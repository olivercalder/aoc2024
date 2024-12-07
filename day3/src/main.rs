use regex::Regex;
use std::io::Read;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = std::io::stdin().lock();
    let mut buffer = String::new();
    input.read_to_string(&mut buffer)?;
    let sum = sum_muls(&buffer);
    let sum_enabled = sum_enabled_muls(&buffer);
    println!("total: {}", sum);
    println!("total enabled: {}", sum_enabled);
    Ok(())
}

fn sum_muls(string: &str) -> usize {
    let re = Regex::new(r"mul\((?<x>[0-9]([0-9]?)([0-9]?)),(?<y>[0-9]([0-9]?)([0-9]?))\)").unwrap();
    re.captures_iter(&string)
        .map(|caps| {
            let x: usize = caps.name("x").unwrap().as_str().parse().unwrap();
            let y: usize = caps.name("y").unwrap().as_str().parse().unwrap();
            x * y
        })
        .sum()
}

fn sum_enabled_muls(string: &str) -> usize {
    string
        .split("do()")
        .map(|s| match s.find("don't()") {
            Some(ind) => &s[..ind],
            None => &s,
        })
        .map(sum_muls)
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

    #[test]
    fn test_sum_muls_multiline() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))\nxmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = super::sum_muls(input);
        assert_eq!(result, 322);
    }

    #[test]
    fn test_sum_enabled_muls() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = super::sum_enabled_muls(input);
        assert_eq!(result, 48);
    }
}

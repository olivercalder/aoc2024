use regex::Regex;

fn main() -> std::io::Result<()> {
    let result = sum_muls(std::io::stdin().lock());
    println!("{}", result);
    Ok(())
}

fn sum_muls(reader: impl std::io::BufRead) -> usize {
    let re = Regex::new(r"mul\((?<x>[0-9]([0-9]?)([0-9]?)),(?<y>[0-9]([0-9]?)([0-9]?))\)").unwrap();
    reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            re.captures_iter(&line)
                .map(|caps| {
                    let x: usize = caps.name("x").unwrap().as_str().parse().unwrap();
                    let y: usize = caps.name("y").unwrap().as_str().parse().unwrap();
                    x * y
                })
                .sum::<usize>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_sum_muls() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = super::sum_muls(std::io::BufReader::new(input.as_bytes()));
        assert_eq!(result, 161);
    }

    #[test]
    fn test_sum_muls_multiline() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))\nxmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let result = super::sum_muls(std::io::BufReader::new(input.as_bytes()));
        assert_eq!(result, 322);
    }
}

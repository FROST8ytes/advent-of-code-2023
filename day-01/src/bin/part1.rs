fn main() {
    let input = include_str!("../day01.txt");
    println!("[+] Answer: {}", part1(input));
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let numbers = parse_digits(line);
            (numbers.first().unwrap() * 10_u32) + (numbers.last().unwrap() % 10_u32)
        })
        .sum()
}

fn parse_digits(input: &str) -> Vec<u32> {
    input.chars().filter_map(|c| c.to_digit(10)).collect()
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn part_1_ok() {
        let result = part1("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet");
        assert_eq!(142_u32, result);
    }
}

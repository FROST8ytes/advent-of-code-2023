fn main() {
    let input = include_str!("../day01.txt");
    println!("[+] Answer: {}", part2(input));
}

fn part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let numbers = parse_digits(convert_text_to_digit(line));
            (numbers.first().unwrap() * 10_u32) + (numbers.last().unwrap() % 10_u32)
        })
        .sum()
}

fn parse_digits(input: String) -> Vec<u32> {
    input.chars().filter_map(|c| c.to_digit(10)).collect()
}

fn convert_text_to_digit(input: &str) -> String {
    input
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e")
}

#[cfg(test)]
mod tests {
    use super::part2;

    #[test]
    fn part_2_ok() {
        let result = part2("two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen");
        assert_eq!(281_u32, result);
    }
}

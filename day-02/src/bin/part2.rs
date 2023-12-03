fn main() {
    let input = include_str!("../day02.txt");
    println!("[+] Answer: {}", part2(input));
}

fn part2(input: &str) -> u32 {
    let mut total: u32 = 0;
    let parsed_lines: Vec<&str> = input.lines().collect();

    for line in parsed_lines {
        let split_line = line.split(':').last().unwrap();
        let split_sets: Vec<Vec<(u32, &str)>> =
            split_line.split(';').map(|set| parse_set(set)).collect();
        let (min_red, min_green, min_blue) = check_minimum(&split_sets);

        total += min_red * min_green * min_blue;
    }

    total
}

fn parse_set(set: &str) -> Vec<(u32, &str)> {
    set.split(',')
        .filter_map(|num_with_colors| {
            num_with_colors
                .trim()
                .split_once(' ')
                .and_then(|(num, color)| {
                    let num = num.parse::<u32>().unwrap();
                    Some((num, color))
                })
        })
        .collect()
}

fn check_minimum(sets: &Vec<Vec<(u32, &str)>>) -> (u32, u32, u32) {
    let mut max_red: u32 = 0;
    let mut max_green: u32 = 0;
    let mut max_blue: u32 = 0;
    for set in sets {
        let mut red: u32 = 0;
        let mut green: u32 = 0;
        let mut blue: u32 = 0;
        for (num, color) in set {
            match *color {
                "red" => red += *num,
                "green" => green += *num,
                "blue" => blue += *num,
                _ => panic!("unknown color"),
            }
        }

        if red > max_red {
            max_red = red;
        }

        if green > max_green {
            max_green = green;
        }

        if blue > max_blue {
            max_blue = blue;
        }
    }
    (max_red, max_green, max_blue)
}

#[cfg(test)]
mod tests {
    use super::part2;

    #[test]
    fn part_2_ok() {
        let result = part2("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(2286_u32, result);
    }
}

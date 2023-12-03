fn main() {
    let input = include_str!("../day02.txt");
    println!("[+] Answer: {}", part1(input));
}

fn part1(input: &str) -> usize {
    let mut total_possible: usize = 0;
    let parsed_input: Vec<bool> = input
        .lines()
        .map(|line| {
            let split_line = line.split(':').last().unwrap();
            let split_sets: Vec<bool> = split_line
                .split(';')
                .map(|set| {
                    let split_colors = set
                        .split(',')
                        .filter_map(|num_with_colors| {
                            num_with_colors
                                .trim()
                                .split_once(' ')
                                .and_then(|(num, color)| {
                                    let num = num.parse::<u32>().unwrap();
                                    Some((num, color))
                                })
                        })
                        .collect::<Vec<(u32, &str)>>();
                    check_validity(&split_colors)
                })
                .collect();
            let mut result = true;
            for is_possible in split_sets {
                if !is_possible {
                    result = false;
                }
            }
            result
        })
        .collect();

    for (game_num, is_possible) in parsed_input.iter().enumerate() {
        if *is_possible {
            total_possible += game_num + 1;
        }
    }
    total_possible
}

const RED: u32 = 12;
const GREEN: u32 = 13;
const BLUE: u32 = 14;

fn check_validity(game_result: &[(u32, &str)]) -> bool {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for (num, color) in game_result.iter() {
        match *color {
            "red" => red += num,
            "green" => green += num,
            "blue" => blue += num,
            _ => panic!("color unknown"),
        }
    }

    red <= RED && green <= GREEN && blue <= BLUE
}

#[cfg(test)]
mod tests {
    use super::part1;

    #[test]
    fn part_1_ok() {
        let result = part1("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");
        assert_eq!(8_usize, result);
    }
}

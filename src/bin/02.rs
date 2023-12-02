use regex::Regex;

advent_of_code::solution!(2);

pub fn parse_part1(input: &str) -> Vec<u32> {
    let max_counts = [12, 13, 14];
    let games = input
        .split('\n')
        .filter(|game| !game.is_empty())
        .filter(|game| {
            let red_regex: Regex = Regex::new(r"\d{1,2} red").unwrap();
            let green_regex: Regex = Regex::new(r"\d{1,2} green").unwrap();
            let blue_regex: Regex = Regex::new(r"\d{1,2} blue").unwrap();

            let valid_red_counts = red_regex
                .find_iter(game)
                .map(|count| {
                    count
                        .as_str()
                        .split(' ')
                        .next()
                        .unwrap()
                        .parse::<usize>()
                        .unwrap()
                })
                .all(|count| count <= max_counts[0]);
            let valid_green_counts = green_regex
                .find_iter(game)
                .map(|count| {
                    count
                        .as_str()
                        .split(' ')
                        .next()
                        .unwrap()
                        .parse::<usize>()
                        .unwrap()
                })
                .all(|count| count <= max_counts[1]);
            let valid_blue_counts = blue_regex
                .find_iter(game)
                .map(|count| {
                    count
                        .as_str()
                        .split(' ')
                        .next()
                        .unwrap()
                        .parse::<usize>()
                        .unwrap()
                })
                .all(|count| count <= max_counts[2]);

            valid_red_counts && valid_blue_counts && valid_green_counts
        })
        .map(|game| {
            game.split(' ')
                .nth(1)
                .unwrap()
                .replace(':', "")
                .parse::<u32>()
                .unwrap()
        })
        .collect();

    games
}

pub fn parse_part2(input: &str) -> Vec<u32> {
    let games = input
        .split('\n')
        .filter(|game| !game.is_empty())
        .map(|game| {
            let red_regex: Regex = Regex::new(r"\d{1,2} red").unwrap();
            let green_regex: Regex = Regex::new(r"\d{1,2} green").unwrap();
            let blue_regex: Regex = Regex::new(r"\d{1,2} blue").unwrap();

            let smallest_red_count = red_regex
                .find_iter(game)
                .map(|count| {
                    count
                        .as_str()
                        .split(' ')
                        .next()
                        .unwrap()
                        .parse::<u32>()
                        .unwrap()
                })
                .max()
                .unwrap();
            let smallest_green_count = green_regex
                .find_iter(game)
                .map(|count| {
                    count
                        .as_str()
                        .split(' ')
                        .next()
                        .unwrap()
                        .parse::<u32>()
                        .unwrap()
                })
                .max()
                .unwrap();
            let smallest_blue_count = blue_regex
                .find_iter(game)
                .map(|count| {
                    count
                        .as_str()
                        .split(' ')
                        .next()
                        .unwrap()
                        .parse::<u32>()
                        .unwrap()
                })
                .max()
                .unwrap();

            [
                smallest_red_count,
                smallest_green_count,
                smallest_blue_count,
            ]
            .iter()
            .product()
        })
        .collect();

    games
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(parse_part1(input).iter().sum())
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(parse_part2(input).iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}

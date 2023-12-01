use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn parse(input: &str) -> Vec<Vec<String>>{
    let calibrations = input
        .split('\n')
        .filter(|calibration| !calibration.is_empty())
        .map(|calibration| {
            calibration
                .chars()
                .filter(|c| c.is_numeric())
                .map(|c| c.to_string())
                .collect()
        })
        .collect();

    calibrations
}

pub fn part_one(input: &str) -> Option<u32> {
    let calibration_values = parse(input);
    let sum = calibration_values
        .into_iter()
        .fold(0, |acc, calibrations| {
            let first_value = calibrations.first().unwrap();
            let last_value = calibrations.last().unwrap();
            let full_value = format!("{}{}", first_value, last_value).parse::<u32>().unwrap();

            acc + full_value
        });

    Some(sum)
    
}

pub fn part_two(input: &str) -> Option<u32> {
    let number_string: HashMap<&str, &str> = HashMap::from([
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ]);

    let mut updated_input = input.to_string();

    for (key, value) in &number_string {
        updated_input = updated_input.replace(key, value.as_ref())
    }

    let calibration_values = parse(&updated_input);
    let sum = calibration_values
        .into_iter()
        .fold(0, |acc, calibrations| {
            let first_value = calibrations.first().unwrap();
            let last_value = calibrations.last().unwrap();
            let full_value = format!("{}{}", first_value, last_value).parse::<u32>().unwrap();

            acc + full_value
        });

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part("examples", DAY, 1));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part("examples", DAY, 2));
        assert_eq!(result, Some(281));
    }
}

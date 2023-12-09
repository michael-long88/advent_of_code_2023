advent_of_code::solution!(9);

pub fn parse(input: &str) -> Vec<Vec<i64>> {
    let report = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_whitespace()
                .map(|value| value.parse::<i64>().unwrap())
                .collect()
        })
        .collect();

    report
}

pub fn is_all_same(readings: &[i64]) -> bool {
    let first = &readings[0];
    readings.iter().all(|reading| reading == first)
}

pub fn get_readings_breakdown(sensor_readings: &[i64]) -> Vec<Vec<i64>> {
    let mut inital_values = sensor_readings.to_owned();
    let mut readings_breakdown: Vec<Vec<i64>> = vec![inital_values.to_vec()];
    while !is_all_same(&inital_values) {
        let mut differences = inital_values.clone();
        differences = differences
            .windows(2)
            .map(|reading_pair| reading_pair[1] - reading_pair[0])
            .collect();
        inital_values = differences;
        readings_breakdown.push(inital_values.to_vec());
    }

    readings_breakdown.reverse();

    readings_breakdown
}

pub fn part_one(input: &str) -> Option<i64> {
    let report = parse(input);

    let extrapolated_values = report.iter().map(|sensor_readings| {
        let mut readings_breakdown = get_readings_breakdown(sensor_readings);
        let readings_breakdown_length = readings_breakdown.len();

        for index in 0..readings_breakdown_length - 1 {
            let last_value = *readings_breakdown[index].last().unwrap()
                + *readings_breakdown[index + 1].last().unwrap();
            readings_breakdown[index + 1].push(last_value);
        }

        *readings_breakdown.last().unwrap().last().unwrap()
    });

    Some(extrapolated_values.sum::<i64>())
}

pub fn part_two(input: &str) -> Option<i64> {
    let report = parse(input);

    let extrapolated_values = report.iter().map(|sensor_readings| {
        let mut readings_breakdown = get_readings_breakdown(sensor_readings);
        let readings_breakdown_length = readings_breakdown.len();

        for index in 0..readings_breakdown_length - 1 {
            let first_value = *readings_breakdown[index + 1].first().unwrap()
                - *readings_breakdown[index].first().unwrap();
            readings_breakdown[index + 1].insert(0, first_value);
        }

        *readings_breakdown.last().unwrap().first().unwrap()
    });

    Some(extrapolated_values.sum::<i64>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}

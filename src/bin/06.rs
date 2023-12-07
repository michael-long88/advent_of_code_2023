use num_traits::{Num, NumCast};
use std::iter::zip;

advent_of_code::solution!(6);

pub struct Race<T> {
    pub time: T,
    pub distance: T,
}

impl<T> Race<T>
where
    T: Num + Copy + Ord + NumCast,
{
    pub fn get_minimum_time(&self) -> T {
        let mut start_time = self.distance / self.time;
        while start_time <= self.time / T::from(2).unwrap() {
            let distance_travelled = start_time * (self.time - start_time);
            if distance_travelled > self.distance {
                return start_time;
            }
            start_time = start_time + T::one();
        }
        T::zero()
    }

    pub fn get_total_possible_wins_count(&self) -> T {
        let minimum_time = self.get_minimum_time();
        self.time - (T::from(2).unwrap() * minimum_time) + T::one()
    }
}

pub fn parse(input: &str) -> Vec<Race<u32>> {
    let mut lines = input.split('\n').filter(|line| !line.is_empty());
    let times: Vec<u32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|time| time.parse::<u32>().unwrap())
        .collect();
    let distances: Vec<u32> = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|distance| distance.parse::<u32>().unwrap())
        .collect();

    zip(times, distances)
        .map(|(time, distance)| Race { time, distance })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let races = parse(input);
    let product = races
        .iter()
        .map(|race| race.get_total_possible_wins_count())
        .product::<u32>();

    Some(product)
}

pub fn part_two(input: &str) -> Option<u64> {
    let races = parse(input);
    let mut time = String::new();
    let mut distance = String::new();

    races.iter().for_each(|race| {
        time.push_str(&race.time.to_string());
        distance.push_str(&race.distance.to_string());
    });

    let race = Race {
        time: time.parse::<u64>().unwrap(),
        distance: distance.parse::<u64>().unwrap(),
    };

    Some(race.get_total_possible_wins_count())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}

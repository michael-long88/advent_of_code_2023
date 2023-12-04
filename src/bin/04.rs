use std::collections::{HashMap, HashSet};

advent_of_code::solution!(4);

pub struct Card {
    pub card_number: u32,
    pub winning_numbers: HashSet<u32>,
    pub check_numbers: Vec<u32>,
}

pub fn parse(input: &str) -> Vec<Card> {
    let winning_numbers = input
        .split('\n')
        .filter(|card| !card.is_empty())
        .map(|card| {
            let mut card_split = card.split(": ");
            let card_name = card_split
                .next()
                .unwrap()
                .split_whitespace()
                .nth(1)
                .unwrap();
            let clean_card = card_split.next().unwrap();
            let mut numbers_split = clean_card.split(" | ");
            let winning_numbers: Vec<u32> = numbers_split
                .next()
                .unwrap()
                .split_whitespace()
                .map(|number| number.parse::<u32>().unwrap())
                .collect();
            let check_numbers: Vec<u32> = numbers_split
                .next()
                .unwrap()
                .split_whitespace()
                .map(|number| number.parse::<u32>().unwrap())
                .collect();

            Card {
                card_number: card_name.parse::<u32>().unwrap(),
                winning_numbers: winning_numbers.into_iter().collect(),
                check_numbers,
            }
        })
        .collect();

    winning_numbers
}

pub fn part_one(input: &str) -> Option<u32> {
    let cards = parse(input);
    let card_points = cards.iter().map(|card| {
        let overlap: Vec<&u32> = card
            .check_numbers
            .iter()
            .filter(|item| card.winning_numbers.contains(item))
            .collect();
        let overlap_length = overlap.len();

        if overlap_length == 0 {
            0
        } else {
            2_u32.pow((overlap_length - 1) as u32)
        }
    });

    Some(card_points.sum::<u32>())
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = parse(input);
    let mut card_counts: HashMap<u32, u32> = HashMap::new();
    // the compiler was complaining about trying to do this with HashMap::From()
    // so here we are in a loop
    (1..=cards.len()).for_each(|card_number| {
        card_counts.insert(card_number as u32, 1);
    });

    cards.iter().for_each(|card| {
        let overlap: Vec<&u32> = card
            .check_numbers
            .iter()
            .filter(|item| card.winning_numbers.contains(item))
            .collect();
        let overlap_length = overlap.len();

        if overlap_length != 0 && card.card_number != cards.len() as u32 {
            let start = card.card_number + 1;
            let mut end = card.card_number + overlap_length as u32;
            if end > cards.len() as u32 {
                end = cards.len() as u32;
            }
            for _ in 1..=(*card_counts.get(&card.card_number).unwrap()) {
                for card_number in start..=end {
                    let count = card_counts.get_mut(&card_number).unwrap();
                    *count += 1;
                }
            }
        }
    });

    Some(card_counts.values().sum::<u32>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}

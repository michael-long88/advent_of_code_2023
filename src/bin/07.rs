#[macro_use]
extern crate lazy_static;

use std::{cmp::Ordering, collections::HashMap, fmt::Display};

advent_of_code::solution!(7);

lazy_static! {
    static ref CARD_ORDER_PART_1: HashMap<String, usize> = {
        let mut m = HashMap::new();
        let card_order = &vec![
            "A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2",
        ];
        for (index, card) in card_order.iter().enumerate() {
            m.insert(card.to_string(), card_order.len() - index);
        }
        m
    };
}

lazy_static! {
    static ref CARD_ORDER_PART_2: HashMap<String, usize> = {
        let mut m = HashMap::new();
        let card_order = &vec![
            "A", "K", "Q", "T", "9", "8", "7", "6", "5", "4", "3", "2", "J",
        ];
        for (index, card) in card_order.iter().enumerate() {
            m.insert(card.to_string(), card_order.len() - index);
        }
        m
    };
}

#[derive(Clone, Debug)]
pub struct Hands {
    pub card_hands: Vec<CardHand>,
}

#[derive(Clone, Debug)]
pub struct CardHand {
    pub cards: Vec<Card>,
    pub hand_score: u32,
    pub bid: u32,
    pub part1: bool,
}

impl CardHand {
    pub fn update_hand_score(&mut self) {
        let mut sorted_hand = self.clone();
        sorted_hand.cards.sort();
        sorted_hand.cards.reverse();
        let joker_count = get_joker_counts(&sorted_hand.cards);

        // check for 5 of a kind
        if is_all_same(&sorted_hand.cards) {
            self.hand_score = 7;
            return;
        }

        // check for 4 of a kind
        for window in sorted_hand.cards.windows(4) {
            if is_all_same(window) {
                self.hand_score = 6;
                return;
            }
        }

        // check for 3 of a kind and full house
        let mut jokers_used = 0;
        for (index, window) in sorted_hand.cards.windows(3).enumerate() {
            // 3 of a kind are in the middle, which means the other 2 cards can't match
            let is_3_of_a_kind = is_all_same(window);
            jokers_used += get_joker_counts(window);
            if is_3_of_a_kind && index == 1 {
                self.hand_score = 4;
                if !self.part1 && jokers_used < joker_count {
                    self.hand_score = 6;
                }
                return;
            } else if is_3_of_a_kind && index == 0 {
                let last_two = &sorted_hand.cards[sorted_hand.cards.len() - 2..];
                if !self.part1 && jokers_used < joker_count {
                    self.hand_score = 6;
                    return;
                }
                if is_all_same(last_two) {
                    self.hand_score = 5;
                    return;
                }
                self.hand_score = 4;
                return;
            } else if is_3_of_a_kind && index == 2 {
                let first_two = &sorted_hand.cards[0..2];
                if !self.part1 && jokers_used < joker_count {
                    self.hand_score = 6;
                    return;
                }
                if is_all_same(first_two) {
                    self.hand_score = 5;
                    return;
                }
                self.hand_score = 4;
                return;
            }
        }

        // check for 2 of a kind
        let mut has_2_of_a_kind = false;
        for window in sorted_hand.cards.windows(2) {
            let is_same = is_all_same(window);
            if has_2_of_a_kind && is_same {
                // pair already exists and has found another pair
                self.hand_score = 3;
                return;
            } else if is_same {
                // found a pair
                has_2_of_a_kind = true;
            }
        }
        if has_2_of_a_kind {
            self.hand_score = 2;
            return;
        }

        self.hand_score = 1;
    }
}

impl Eq for CardHand {}

impl PartialEq for CardHand {
    fn eq(&self, other: &Self) -> bool {
        self.hand_score == other.hand_score && self.cards == other.cards
    }
}

impl Ord for CardHand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_score.cmp(&other.hand_score) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            other => other,
        }
    }
}

impl PartialOrd for CardHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for CardHand {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let cards = self
            .cards
            .iter()
            .map(|card| card.value.to_string())
            .collect::<Vec<_>>()
            .join("");
        write!(
            f,
            "Cards: {}, Bid: {}, Hand score: {}",
            cards, self.bid, self.hand_score
        )
    }
}

pub fn is_all_same(cards: &[Card]) -> bool {
    // cards[0] == cards[cards.len() - 1]
    let first = &cards[0];
    cards.iter().all(|card| card == *first)
}

pub fn get_joker_counts(cards: &[Card]) -> usize {
    cards.iter().filter(|card| card.value == "J").count()
}

#[derive(Clone, Debug)]
pub struct Card {
    pub value: String,
    pub part1: bool,
}

impl Eq for Card {}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        if self.part1 {
            self.value == other.value
        } else {
            self.value == "J" || other.value == "J" || self.value == other.value
        }
    }
}

impl PartialEq<Card> for &Card {
    fn eq(&self, other: &Card) -> bool {
        if self.part1 {
            self.value == other.value
        } else {
            self.value == "J" || other.value == "J" || self.value == other.value
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.part1 {
            let self_rank = CARD_ORDER_PART_1.get(&self.value).unwrap();
            let other_rank = CARD_ORDER_PART_1.get(&other.value).unwrap();
            self_rank.cmp(other_rank)
        } else {
            let self_rank = CARD_ORDER_PART_2.get(&self.value).unwrap();
            let other_rank = CARD_ORDER_PART_2.get(&other.value).unwrap();
            self_rank.cmp(other_rank)
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn parse(input: &str, part1: bool) -> Hands {
    let card_hands = input
        .lines()
        .filter(|card_hand| !card_hand.is_empty())
        .map(|card_hand| {
            let mut splits = card_hand.split_whitespace();
            let cards = splits
                .next()
                .unwrap()
                .chars()
                .map(|card_value| Card {
                    value: card_value.to_string(),
                    part1,
                })
                .collect();

            let bid = splits
                .next()
                .unwrap()
                .parse::<u32>()
                .expect("Unable to parse hand score");

            CardHand {
                cards,
                hand_score: 1,
                bid,
                part1,
            }
        })
        .collect();

    Hands { card_hands }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut hands = parse(input, true);
    hands
        .card_hands
        .iter_mut()
        .for_each(|card_hand| card_hand.update_hand_score());

    hands.card_hands.sort();

    let winnings = hands
        .card_hands
        .iter()
        .enumerate()
        .map(|(index, hand)| hand.bid * (index as u32 + 1))
        .sum::<u32>();

    Some(winnings)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands = parse(input, false);
    hands
        .card_hands
        .iter_mut()
        .for_each(|card_hand| card_hand.update_hand_score());

    hands.card_hands.sort();
    // hands.card_hands.iter().for_each(|card_hand| println!("{}", card_hand));

    let winnings = hands
        .card_hands
        .iter()
        .enumerate()
        .map(|(index, hand)| {
            // let total = hand.bid * (index as u32 + 1);
            // println!("For {}, {} * {} = {}", hand, hand.bid, (index as u32 + 1), total);
            hand.bid * (index as u32 + 1)
        })
        .sum::<u32>();

    Some(winnings)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7190));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7460));
    }
}

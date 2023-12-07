use itertools::Itertools;
use std::{cmp::Ordering, fmt::Display};

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Card(u8);

impl Card {
    const JOKER: Card = Card(11);

    #[allow(dead_code)]
    pub fn new(value: u8) -> Result<Self, String> {
        if !(2..=14).contains(&value) {
            Err(format!("Value cannot be less 2 or greater than 14, got {value}").to_owned())
        } else {
            Ok(Card(value))
        }
    }

    pub const fn from_char(c: char) -> Result<Self, &'static str> {
        match c {
            'A' => Ok(Card(14)),
            'K' => Ok(Card(13)),
            'Q' => Ok(Card(12)),
            'J' => Ok(Card(11)),
            'T' => Ok(Card(10)),
            '2'..='9' => Ok(Card(c as u8 - b'2' + 2)),
            _ => Err("Value cannot be anything but one of \"AKQJT98765432\""),
        }
    }

    pub const fn value(&self) -> u8 {
        self.0
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A']
                [self.0 as usize]
        )
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandRank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Hand {
    pub cards: Box<[Card; 5]>,
    pub bet: u32,
}

impl Hand {
    pub fn new(cards: Box<[Card; 5]>, bet: u32) -> Self {
        Hand { cards, bet }
    }

    pub fn hand_rank(&self) -> HandRank {
        let mut counts: Box<[_; 5]> = self
            .cards
            .iter()
            .counts()
            .into_values()
            .pad_using(5, |_| 0)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        counts.sort_unstable();

        match *counts {
            [0, 0, 0, 0, 5] => HandRank::FiveOfAKind,
            [0, 0, 0, 1, 4] => HandRank::FourOfAKind,
            [0, 0, 0, 2, 3] => HandRank::FullHouse,
            [0, 0, 1, 1, 3] => HandRank::ThreeOfAKind,
            [0, 0, 1, 2, 2] => HandRank::TwoPair,
            [0, 1, 1, 1, 2] => HandRank::OnePair,
            [1, 1, 1, 1, 1] => HandRank::HighCard,
            _ => unreachable!("got impossible count of cards"),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_hand_rank = self.hand_rank();
        let other_hand_rank = other.hand_rank();

        if self_hand_rank != other_hand_rank {
            return self_hand_rank.cmp(&other_hand_rank);
        }

        for (self_card, other_card) in self.cards.iter().zip(other.cards.iter()) {
            match self_card.cmp(other_card) {
                Ordering::Equal => continue,
                Ordering::Less => return Ordering::Less,
                Ordering::Greater => return Ordering::Greater,
            }
        }

        Ordering::Equal
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct JokerHand(Hand);

impl JokerHand {
    pub fn hand(&self) -> &Hand {
        &self.0
    }

    pub fn hand_rank(&self) -> HandRank {
        let mut counts = self.0.cards.iter().counts();
        let num_jokers = counts.remove(&Card::JOKER).unwrap_or(0);
        let mut counts: Box<[_; 5]> = counts
            .into_values()
            .pad_using(5, |_| 0)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        counts.sort_unstable();

        counts[4] += num_jokers;

        match *counts {
            [0, 0, 0, 0, 5] => HandRank::FiveOfAKind,
            [0, 0, 0, 1, 4] => HandRank::FourOfAKind,
            [0, 0, 0, 2, 3] => HandRank::FullHouse,
            [0, 0, 1, 1, 3] => HandRank::ThreeOfAKind,
            [0, 0, 1, 2, 2] => HandRank::TwoPair,
            [0, 1, 1, 1, 2] => HandRank::OnePair,
            [1, 1, 1, 1, 1] => HandRank::HighCard,
            _ => unreachable!("got impossible count of cards"),
        }
    }
}

impl PartialOrd for JokerHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for JokerHand {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_hand_rank = self.hand_rank();
        let other_hand_rank = other.hand_rank();

        if self_hand_rank != other_hand_rank {
            return self_hand_rank.cmp(&other_hand_rank);
        }

        for (self_card, other_card) in self.0.cards.iter().zip(other.0.cards.iter()) {
            let self_value: &u8 = match self_card {
                &Card::JOKER => &1,
                Card(n) => n,
            };
            let other_value: &u8 = match other_card {
                &Card::JOKER => &1,
                Card(n) => n,
            };
            match self_value.cmp(other_value) {
                Ordering::Equal => continue,
                Ordering::Less => return Ordering::Less,
                Ordering::Greater => return Ordering::Greater,
            }
        }

        Ordering::Equal
    }
}

impl From<Hand> for JokerHand {
    fn from(hand: Hand) -> Self {
        JokerHand(hand)
    }
}

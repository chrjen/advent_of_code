use itertools::Itertools;
use std::{cmp::Ordering, fmt::Display};

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Card(u8);

impl Card {
    const JOKER: Card = Card(11);

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
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            [
                '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A'
            ][self.0 as usize]
        )
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum HandRank {
    FiveOfAKind = 6,
    FourOfAKind = 5,
    FullHouse = 4,
    ThreeOfAKind = 3,
    TwoPair = 2,
    OnePair = 1,
    HighCard = 0,
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

    pub fn hand_rank_joker(&self) -> HandRank {
        let mut counts = self.cards.iter().counts();
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

    pub fn cmp_cards(&self, other: &Self) -> Ordering {
        let hand_rank: Ordering = self.hand_rank().cmp(&other.hand_rank());

        if hand_rank == Ordering::Equal {
            self.cards
                .iter()
                .zip(other.cards.iter())
                .map(|(card_self, card_other)| card_self.cmp(card_other))
                .find(|&ord| ord != Ordering::Equal)
                .unwrap_or(Ordering::Equal)
        } else {
            hand_rank
        }
    }

    pub fn cmp_cards_joker(&self, other: &Self) -> Ordering {
        let hand_rank: Ordering = self.hand_rank_joker().cmp(&other.hand_rank_joker());

        fn to_value(card: &Card) -> &u8 {
            match card {
                &Card::JOKER => &1,
                Card(n) => n,
            }
        }

        if hand_rank == Ordering::Equal {
            let other_cards = other.cards.iter().map(to_value);
            self.cards
                .iter()
                .map(to_value)
                .zip(other_cards)
                .map(|(value_self, value_other)| value_self.cmp(value_other))
                .find(|&ord| ord != Ordering::Equal)
                .unwrap_or(Ordering::Equal)
        } else {
            hand_rank
        }
    }
}

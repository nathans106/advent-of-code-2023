use std::cmp::Ordering;
use std::cmp::Ordering::{Equal, Greater, Less};
use std::collections::HashMap;
use itertools::Itertools;
use crate::Card::{Ace, Eight, Five, Four, Jack, King, Nine, Queen, Seven, Six, Ten, Three, Two};
use crate::HandType::{FiveKind, FourKind, FullHouse, HighCard, OnePair, ThreeKind, TwoPair};

fn main() {
    let data = include_str!("../../data/day_7.txt");
    let mut players = parse_players(data);
    assign_ranks(&mut players);
    let winnings: usize = players.iter().map(|p| p.winnings().unwrap()).sum();
    println!("Task 1: {winnings}");
}

#[derive(Clone, PartialEq, PartialOrd, Eq, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace
}

impl Card {
    fn from_char(c: &char) -> Self {
        let char_to_card = HashMap::from([
            ('2', Two),
            ('3', Three),
            ('4', Four),
            ('5', Five),
            ('6', Six),
            ('7', Seven),
            ('8', Eight),
            ('9', Nine),
            ('T', Ten),
            ('J', Jack),
            ('Q', Queen),
            ('K', King),
            ('A', Ace)
        ]);

        char_to_card[c].clone()
    }
}

#[derive(PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind
}

struct Hand {
    cards: Vec<Card>
}

impl Hand {
    fn new(cards: Vec<Card>) -> Self {
        Hand{cards}
    }

    fn hand_type(&self) -> HandType {
        let mut counts : HashMap<Card, u32> = HashMap::new();

        for card in &self.cards {
            let maybe_count = counts.get(&card);
            match maybe_count {
                None => {counts.insert(card.clone(), 1); ()},
                Some(count) => {
                    let old_count = count.clone();
                    counts.insert(card.clone(), old_count + 1);
                    ()
                }
            }
        }

        if counts.values().contains(&5) {
            return FiveKind;
        }

        if counts.values().contains(&4) {
            return FourKind;
        }

        if counts.values().contains(&3) {
            if counts.values().contains(&2) {
                return FullHouse;
            }
            return ThreeKind;
        }

        let num_pairs = counts.values().filter(|value| value == &&2).count();

        if num_pairs == 2 {
            return TwoPair;
        }

        if num_pairs == 1 {
            return OnePair;
        }

        return HighCard;
    }
}

impl PartialEq<Self> for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }

        if self.hand_type() > other.hand_type() {
            return Some(Ordering::Greater);
        }

        if self.hand_type() < other.hand_type() {
            return Some(Ordering::Less);
        }

        for (card, other_card) in self.cards.iter().zip(&other.cards) {
            if card > other_card {
                return Some(Greater);
            }

            if card < other_card {
                return Some(Less);
            }
        }

        return Some(Equal)
    }
}

type Bid = usize;
type Winnings = usize;

struct Player {
    hand: Hand,
    bid: Bid,
    rank: Option<usize>
}

impl Player {
    fn new(hand: Hand, bid: Bid) -> Self {
        Player{hand, bid, rank: None}
    }

    fn winnings(&self) -> Option<Winnings> {
        Some(self.rank? * self.bid)
    }
}

fn parse_players(data: &str) -> Vec<Player> {
    data.lines().map(|line| {
        let (cards_str, bid_str) = line.split(" ").collect_tuple().unwrap();
        let hand = Hand::new(cards_str.chars().map(|c| Card::from_char(&c)).collect_vec());
        let bid = bid_str.parse().unwrap();
        Player::new(hand, bid)
    }).collect()
}

fn assign_ranks(players: &mut Vec<Player>) {
    for (i, player) in players.iter_mut().sorted_by(|p1, p2| p1.hand.partial_cmp(&p2.hand).unwrap()).enumerate() {
        player.rank = Some(i + 1);
    }
}


#[cfg(test)]
mod tests {
    use crate::{assign_ranks, parse_players};

    #[test]
    fn task1() {
        let data = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

        let mut players = parse_players(data);
        assign_ranks(&mut players);
        let winnings: usize = players.iter().map(|p| p.winnings().unwrap()).sum();
        assert_eq!(winnings, 6440);
    }
}
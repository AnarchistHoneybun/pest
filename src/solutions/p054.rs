use std::io::BufRead;
use crate::solutions::Solution;
use itertools::Itertools;

pub struct Problem054;

impl Problem054 {
    pub fn new() -> Self {
        Self
    }
}

impl Solution for Problem054 {
    fn number(&self) -> u32 { 54 }

    fn title(&self) -> &'static str {
        "Poker Hands"
    }

    fn solve(&self) -> String {
        let file = std::fs::File::open("resources/p054_poker.txt").unwrap();
        std::io::BufReader::new(file)
            .lines()
            .map(|l| l.unwrap())
            .map(|line| {
                let cards: Vec<Card> = line.split_whitespace().map(parse_card).collect();
                let (p1, p2) = cards.split_at(5);
                (PokerHand::new(p1), PokerHand::new(p2))
            })
            .filter(|(p1, p2)| p1.beats(p2))
            .count()
            .to_string()
    }
}

fn parse_card(card: &str) -> Card {
    let (rank_str, suit_str) = card.split_at(card.len() - 1);
    Card {
        rank: match rank_str {
            "2" => Rank::Two, "3" => Rank::Three, "4" => Rank::Four, "5" => Rank::Five,
            "6" => Rank::Six, "7" => Rank::Seven, "8" => Rank::Eight, "9" => Rank::Nine,
            "T" => Rank::Ten, "J" => Rank::Jack, "Q" => Rank::Queen, "K" => Rank::King,
            "A" => Rank::Ace, _ => panic!("Invalid rank"),
        },
        suit: match suit_str {
            "H" => Suit::Hearts, "D" => Suit::Diamonds,
            "C" => Suit::Clubs, "S" => Suit::Spades,
            _ => panic!("Invalid suit"),
        },
    }
}

#[derive(Clone)]
struct PokerHand {
    cards: Vec<Card>,
}

impl PokerHand {
    fn new(cards: &[Card]) -> Self {
        Self { cards: cards.to_vec() }
    }

    fn beats(&self, other: &Self) -> bool {
        self.evaluate() > other.evaluate()
    }

    fn evaluate(&self) -> HandRank {
        let mut ranks: Vec<Rank> = self.cards.iter().map(|c| c.rank).collect();
        ranks.sort_by_key(|r| std::cmp::Reverse(*r as u8));

        let is_flush = self.cards.iter().all(|c| c.suit == self.cards[0].suit);
        let is_straight = self.is_straight(&ranks);

        let counts = self.rank_counts();
        let sorted_counts: Vec<_> = counts.iter()
            .map(|(&rank, &count)| (rank, count))
            .sorted_by(|a, b| b.1.cmp(&a.1).then_with(|| (b.0 as u8).cmp(&(a.0 as u8))))
            .collect();

        use HandRank::*;
        match (is_flush, is_straight) {
            (true, true) if ranks.contains(&Rank::Ace) && ranks.contains(&Rank::Ten) => RoyalFlush,
            (true, true) => StraightFlush(self.straight_high(&ranks)),
            _ => match sorted_counts.as_slice() {
                [(four, 4), (single, 1)] => FourOfAKind(*four, *single),
                [(three, 3), (pair, 2)] => FullHouse(*three, *pair),
                _ if is_flush => Flush(ranks),
                _ if is_straight => Straight(self.straight_high(&ranks)),
                [(three, 3), ..] => ThreeOfAKind(*three, self.kickers(&sorted_counts, 1)),
                [(p1, 2), (p2, 2), (k, 1)] => {
                    let (high, low) = if (*p1 as u8) > (*p2 as u8) { (*p1, *p2) } else { (*p2, *p1) };
                    TwoPairs(high, low, *k)
                }
                [(pair, 2), ..] => OnePair(*pair, self.kickers(&sorted_counts, 1)),
                _ => HighCard(ranks),
            }
        }
    }

    fn is_straight(&self, ranks: &[Rank]) -> bool {
        let vals: Vec<u8> = ranks.iter().map(|r| *r as u8).sorted().collect();
        vals.windows(2).all(|w| w[1] == w[0] + 1) || vals == [2, 3, 4, 5, 14]
    }

    fn straight_high(&self, ranks: &[Rank]) -> Rank {
        if ranks == &[Rank::Ace, Rank::Five, Rank::Four, Rank::Three, Rank::Two] {
            Rank::Five
        } else {
            ranks[0]
        }
    }

    fn rank_counts(&self) -> std::collections::HashMap<Rank, usize> {
        self.cards.iter().map(|c| c.rank).fold(std::collections::HashMap::new(), |mut acc, r| {
            *acc.entry(r).or_insert(0) += 1;
            acc
        })
    }

    fn kickers(&self, sorted_counts: &[(Rank, usize)], skip: usize) -> Vec<Rank> {
        sorted_counts.iter().skip(skip).map(|(rank, _)| *rank).collect()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandRank {
    HighCard(Vec<Rank>),
    OnePair(Rank, Vec<Rank>),
    TwoPairs(Rank, Rank, Rank),
    ThreeOfAKind(Rank, Vec<Rank>),
    Straight(Rank),
    Flush(Vec<Rank>),
    FullHouse(Rank, Rank),
    FourOfAKind(Rank, Rank),
    StraightFlush(Rank),
    RoyalFlush,
}

#[derive(Debug, Clone, Copy)]
struct Card {
    rank: Rank,
    suit: Suit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Rank {
    Two = 2, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Suit {
    Hearts, Diamonds, Clubs, Spades,
}
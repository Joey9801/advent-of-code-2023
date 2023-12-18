use std::cmp::Reverse;

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum Card {
    Ace,
    King,
    Queen,
    JokerJack,
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

impl Card {
    fn from_char(c: char) -> Option<Card> {
        match c {
            'A' => Some(Card::Ace),
            'K' => Some(Card::King),
            'Q' => Some(Card::Queen),
            'J' => Some(Card::JokerJack),
            'T' => Some(Card::Ten),
            '9' => Some(Card::Nine),
            '8' => Some(Card::Eight),
            '7' => Some(Card::Seven),
            '6' => Some(Card::Six),
            '5' => Some(Card::Five),
            '4' => Some(Card::Four),
            '3' => Some(Card::Three),
            '2' => Some(Card::Two),
            _ => None,
        }
    }

    fn value_with_jacks(&self) -> u8 {
        match self {
            Card::Ace => 14,
            Card::King => 13,
            Card::Queen => 12,
            Card::JokerJack => 11,
            Card::Ten => 10,
            Card::Nine => 9,
            Card::Eight => 8,
            Card::Seven => 7,
            Card::Six => 6,
            Card::Five => 5,
            Card::Four => 4,
            Card::Three => 3,
            Card::Two => 2,
        }
    }

    fn value_with_jokers(&self) -> u8 {
        match self {
            Card::JokerJack => 1,
            _ => self.value_with_jacks(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Pattern {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Clone, Copy, Debug)]
pub struct Hand {
    cards: [Card; 5],
    bid: u32,
}

pub fn parse(input: &str) -> Vec<Hand> {
    // Input like:
    // 32T3K 765
    // T55J5 684
    // KK677 28

    input
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_at(5);
            let cards =
                std::array::from_fn(|i| Card::from_char(hand.chars().nth(i).unwrap()).unwrap());
            let bid = bid.trim().parse().unwrap();
            Hand { cards, bid }
        })
        .collect()
}

fn find_pattern(hand: &[Card; 5], use_jokers: bool) -> Pattern {
    // Count how the occurrences of each card, and sort by count
    let mut counts = [0; 13];
    for card in hand {
        counts[*card as usize] += 1;
    }

    if use_jokers {
        let joker_count = counts[Card::JokerJack as usize];
        counts[Card::JokerJack as usize] = 0;
        counts.sort();
        counts[12] += joker_count;
    } else {
        counts.sort()
    }

    match &counts[10..] {
        [0, 0, 5] => Pattern::FiveOfAKind,
        [0, 1, 4] => Pattern::FourOfAKind,
        [0, 2, 3] => Pattern::FullHouse,
        [1, 1, 3] => Pattern::ThreeOfAKind,
        [1, 2, 2] => Pattern::TwoPair,
        [1, 1, 2] => Pattern::OnePair,
        _ => Pattern::HighCard,
    }
}

fn sorting_key(hand: &Hand, use_jokers: bool) -> impl Ord + Copy + Clone {
    let pattern = find_pattern(&hand.cards, use_jokers);

    // Use reverse so that higher card values come before lower ones when sorting
    let values = if use_jokers {
        hand.cards.map(|card| Reverse(card.value_with_jokers()))
    } else {
        hand.cards.map(|card| Reverse(card.value_with_jacks()))
    };

    (pattern, values)
}

fn total_winnings(hands: &[Hand], use_jokers: bool) -> u32 {
    let mut hands = hands.to_vec();
    hands.sort_by_cached_key(|hand| sorting_key(hand, use_jokers));

    hands
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, hand)| {
            let rank = idx as u32 + 1;
            rank * hand.bid
        })
        .sum()
}

pub fn solve_part_1(input: &[Hand]) -> u32 {
    total_winnings(input, false)
}

pub fn solve_part_2(input: &[Hand]) -> u32 {
    total_winnings(input, true)
}

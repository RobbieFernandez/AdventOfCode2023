use std::collections::HashMap;
use std::env;
use std::fs::read_to_string;

#[derive(Debug, Clone)]
struct Card {
    value: u32,
    label: char,
}

#[derive(Debug, Clone)]
struct WildCard {
    value: u32,
    label: char,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Hand<C>
where
    C: IsCard,
{
    cards: [C; 5],
    hand_type: HandType,
    bid: u32,
}

trait IsCard: Ord {
    fn label(&self) -> &char;

    fn is_wildcard(&self) -> bool {
        false
    }
}

impl IsCard for Card {
    fn label(&self) -> &char {
        &self.label
    }
}

impl IsCard for WildCard {
    fn is_wildcard(&self) -> bool {
        self.label == 'J'
    }

    fn label(&self) -> &char {
        &self.label
    }
}

impl Card {
    fn new(label: char) -> Self {
        let value = Self::get_value(label);
        Self { label, value }
    }

    fn get_value(label: char) -> u32 {
        match label {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            '2'..='9' => label.to_digit(10).unwrap(),
            _ => panic!("Unknown card label : {}", label),
        }
    }
}

impl From<Card> for WildCard {
    fn from(card: Card) -> Self {
        let is_wild = card.label == 'J';
        let value = if is_wild { 1 } else { card.value };
        Self {
            label: card.label,
            value,
        }
    }
}

impl HandType {
    fn value(&self) -> u32 {
        match self {
            HandType::HighCard => 1,
            HandType::OnePair => 2,
            HandType::TwoPair => 3,
            HandType::ThreeOfAKind => 4,
            HandType::FullHouse => 5,
            HandType::FourOfAKind => 6,
            HandType::FiveOfAKind => 7,
        }
    }

    fn evaluate_cards<C: IsCard>(cards: &[C; 5]) -> Self {
        let labels = vec!['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];

        labels
            .iter()
            .map(|c| Self::evaluate_for_wildcard_label(cards, *c))
            .max()
            .unwrap()
    }

    fn evaluate_for_wildcard_label<C: IsCard>(cards: &[C; 5], with_label: char) -> Self {
        let mut card_counts = HashMap::<&char, u32>::new();

        for card in cards {
            let label = if card.is_wildcard() {
                &with_label
            } else {
                card.label()
            };

            if !card_counts.contains_key(label) {
                card_counts.insert(label, 0);
            }

            let count = card_counts.get_mut(label).unwrap();
            *count += 1;
        }

        if card_counts.values().any(|v| *v >= 5) {
            return Self::FiveOfAKind;
        } else if card_counts.values().any(|v| *v == 4) {
            return Self::FourOfAKind;
        }

        let has_three_of_a_kind = card_counts.values().any(|v| *v == 3);
        let num_pairs = card_counts.values().filter(|v| *v == &2).count();

        if has_three_of_a_kind && num_pairs == 1 {
            Self::FullHouse
        } else if has_three_of_a_kind {
            Self::ThreeOfAKind
        } else if num_pairs == 2 {
            Self::TwoPair
        } else if num_pairs == 1 {
            Self::OnePair
        } else {
            Self::HighCard
        }
    }
}

impl<C: IsCard> Hand<C> {
    fn new(cards: [C; 5], bid: u32) -> Self {
        let hand_type = HandType::evaluate_cards(&cards);
        Self {
            cards,
            bid,
            hand_type,
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.label == other.label
    }
}

impl Eq for Card {}

impl Ord for WildCard {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for WildCard {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for WildCard {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value && self.label == other.label
    }
}

impl Eq for WildCard {}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value().cmp(&other.value())
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<C: IsCard> Ord for Hand<C> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let my_hand_type = &self.hand_type;
        let other_hand_type = &other.hand_type;

        let hand_type_comparison = my_hand_type.cmp(other_hand_type);

        if hand_type_comparison.is_eq() {
            // Types are equal.
            // Return first non-equal card comparison
            std::iter::zip(self.cards.iter(), other.cards.iter())
                .map(|(my_card, other_card)| my_card.cmp(other_card))
                .find(|ord| !ord.is_eq())
                .unwrap_or(std::cmp::Ordering::Equal)
        } else {
            hand_type_comparison
        }
    }
}

impl<C: IsCard> PartialOrd for Hand<C> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_input(filename: &str) -> Vec<Hand<Card>> {
    let input = read_to_string(filename).expect("Could not read file");
    let lines = input.trim().lines();

    let hands = lines.map(|line| {
        let mut line = line.split(' ');
        let cards = line.next().unwrap();
        let bid = line.next().unwrap();

        let bid: u32 = bid.parse().unwrap();

        let cards = cards.chars();
        let cards = cards.map(Card::new);
        let cards: [Card; 5] = cards.collect::<Vec<Card>>().try_into().unwrap();

        Hand::new(cards, bid)
    });

    hands.collect()
}

fn play<C: IsCard>(hands: &mut [Hand<C>]) -> u32 {
    hands.sort();

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| {
            let rank = (i as u32) + 1;
            hand.bid * rank
        })
        .sum()
}

fn solution1(hands: &[Hand<Card>]) -> u32 {
    let mut hands = hands.to_vec();
    play(&mut hands)
}

fn solution2(hands: &[Hand<Card>]) -> u32 {
    let mut hands: Vec<Hand<WildCard>> = hands
        .iter()
        .map(|h| {
            let wild_cards: [WildCard; 5] = h
                .cards
                .clone()
                .into_iter()
                .map(Into::<WildCard>::into)
                .collect::<Vec<WildCard>>()
                .try_into()
                .unwrap();

            Hand::new(wild_cards, h.bid)
        })
        .collect();

    play(&mut hands)
}

fn main() {
    let mut args = env::args();
    let filename = args.nth(1).expect("Filename must be given.");
    let hands = parse_input(&filename);

    let answer1 = solution1(&hands);
    println!("Solution 1: {}", answer1);

    let answer2 = solution2(&hands);
    println!("Solution 2: {}", answer2);
}

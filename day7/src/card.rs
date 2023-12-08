use anyhow::anyhow;
use std::str::FromStr;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Hand([Card; 5]);

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards = s
            .chars()
            .map(|c| Card::from_str(c.to_string().as_str()))
            .collect::<Result<Vec<Card>, anyhow::Error>>()?;
        if cards.len() != 5 {
            return Err(anyhow!("Improper length of cards in input: {}", s));
        }

        let mut cds = [Card::Ace; 5];
        for (i, card) in cards.into_iter().enumerate() {
            cds[i] = card;
        }

        Ok(Self(cds))
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some((HandKind::from_hand(self), self.0).cmp(&(HandKind::from_hand(other), other.0)))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).expect("always returns an order")
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum HandKind {
    Five,
    Four,
    Full,
    Three,
    Two,
    One,
    High,
}

impl HandKind {
    pub fn from_hand(hand: &Hand) -> Self {
        let mut kind = Self::High;
        for card in Card::all_cards() {
            match hand.0.iter().filter(|&&c| c == card).count() {
                5 => return Self::Five,
                4 => return Self::Four,
                3 => match kind {
                    Self::High => kind = Self::Three,
                    Self::One => return Self::Full,
                    _ => unreachable!("Found 3"),
                },
                2 => match kind {
                    Self::High => kind = Self::One,
                    Self::One => return Self::Two,
                    Self::Three => return Self::Full,
                    _ => unreachable!("Found 2"),
                },
                _ => (),
            }
            // more than three cards used
            if kind != Self::Three && kind < Self::One {
                return kind;
            }
        }
        kind
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Card {
    Ace,
    King,
    Queen,
    Jack,
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

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => Self::Ace,
            "K" => Self::King,
            "Q" => Self::Queen,
            "J" => Self::Jack,
            "T" => Self::Ten,
            "9" => Self::Nine,
            "8" => Self::Eight,
            "7" => Self::Seven,
            "6" => Self::Six,
            "5" => Self::Five,
            "4" => Self::Four,
            "3" => Self::Three,
            "2" => Self::Two,
            _ => return Err(anyhow!("Illegal string input for card: {}", s)),
        })
    }
}

impl Card {
    fn all_cards() -> impl Iterator<Item = Card> {
        vec![
            Self::Ace,
            Self::King,
            Self::Queen,
            Self::Jack,
            Self::Ten,
            Self::Nine,
            Self::Eight,
            Self::Seven,
            Self::Six,
            Self::Five,
            Self::Four,
            Self::Three,
            Self::Two,
        ]
        .into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! hand {
        ($expression:expr) => {
            Hand::from_str($expression).unwrap()
        };
    }
    macro_rules! kind {
        ($expression:expr) => {
            HandKind::from_hand(&Hand::from_str($expression).unwrap())
        };
    }

    #[test]
    fn test_hand_ord() {
        assert!(hand!("AAAAA") < hand!("TTTTT"));
        assert!(hand!("TTTTT") < hand!("99999"));
        assert!(hand!("TTTTT") < hand!("88888"));
        assert!(hand!("TTTTT") < hand!("AAAKK"));
        assert!(hand!("KK677") < hand!("KTJJT"));
        assert!(hand!("77888") < hand!("77788"));
    }

    #[test]
    fn test_hand_kind() {
        assert_eq!(kind!("AAAAA"), HandKind::Five);
        assert_eq!(kind!("AAAAT"), HandKind::Four);
        assert_eq!(kind!("AAATT"), HandKind::Full);
        assert_eq!(kind!("AAAT9"), HandKind::Three);
        assert_eq!(kind!("AATT9"), HandKind::Two);
        assert_eq!(kind!("AAT89"), HandKind::One);
        assert_eq!(kind!("A7T89"), HandKind::High);
    }
}

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
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let sk = self.kind();
        let ok = other.kind();
        //dbg!(self, sk, other, ok);
        (sk, self.0).cmp(&(ok, other.0))
    }
}

impl Hand {
    pub fn kind(&self) -> HandKind {
        HandKind::from_hand(self)
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
        let jokers = hand.0.iter().filter(|&&c| c == Card::Joker).count();
        if jokers >= 4 {
            return Self::Five;
        }
        for card in Card::non_jokers() {
            match hand.0.iter().filter(|&&c| c == card).count() + jokers {
                n if n == jokers => (),
                5 => return Self::Five,
                4 => {
                    if kind > Self::Four {
                        kind = Self::Four
                    }
                }
                3 => match kind {
                    Self::High => kind = Self::Three,
                    Self::One if jokers == 0 => kind = Self::Full, // Otherwise would be 3
                    Self::One if jokers == 1 => kind = Self::Three,
                    Self::Three if jokers == 1 => kind = Self::Full, // Must be a joker and two pairs
                    Self::Two => kind = Self::Three,
                    _ => (),
                },
                2 => match kind {
                    Self::High => kind = Self::One,
                    Self::One if jokers == 0 => kind = Self::Two, // otherwise would be more than 2
                    Self::Three if jokers == 0 => kind = Self::Full,
                    _ => (),
                },
                _ => (),
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
    Ten,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    Joker,
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => Self::Ace,
            "K" => Self::King,
            "Q" => Self::Queen,
            "T" => Self::Ten,
            "9" => Self::Nine,
            "8" => Self::Eight,
            "7" => Self::Seven,
            "6" => Self::Six,
            "5" => Self::Five,
            "4" => Self::Four,
            "3" => Self::Three,
            "2" => Self::Two,
            "J" => Self::Joker,
            _ => return Err(anyhow!("Illegal string input for card: {}", s)),
        })
    }
}

impl Card {
    fn non_jokers() -> impl Iterator<Item = Card> {
        vec![
            Self::Ace,
            Self::King,
            Self::Queen,
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
    fn hand_ord() {
        assert!(hand!("AAAAA") < hand!("TTTTT"));
        assert!(hand!("TTTTT") < hand!("99999"));
        assert!(hand!("TTTTT") < hand!("88888"));
        assert!(hand!("TTTTT") < hand!("AAAKK"));
        assert!(hand!("KK677") > hand!("KTJJT"));
        assert!(hand!("77888") < hand!("77788"));
        assert!(hand!("QQQQ2") < hand!("JQQQ2"));

        assert!(hand!("32T3K") < hand!("J345A"));
    }

    #[test]
    fn hand_kind() {
        assert_eq!(kind!("AAAAA"), HandKind::Five);
        assert_eq!(kind!("AAAAT"), HandKind::Four);
        assert_eq!(kind!("AAATT"), HandKind::Full);
        assert_eq!(kind!("AAAT9"), HandKind::Three);
        assert_eq!(kind!("AATT9"), HandKind::Two);
        assert_eq!(kind!("AAT89"), HandKind::One);
        assert_eq!(kind!("A7T89"), HandKind::High);
        assert_eq!(kind!("AAJKK"), HandKind::Full);
        assert_eq!(kind!("2JJJJ"), HandKind::Five);
        assert_eq!(kind!("Q2KJJ"), HandKind::Three);

        assert_eq!(kind!("622J8"), HandKind::Three);
    }
}

use std::fmt;

// Useful balatro docs: https://balatrogame.fandom.com/wiki/Card_Ranks

/// Card rank or value.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy, Hash)]
pub enum Value {
    Two = 0,
    Three = 1,
    Four = 2,
    Five = 3,
    Six = 4,
    Seven = 5,
    Eight = 6,
    Nine = 7,
    Ten = 8,
    Jack = 9,
    Queen = 10,
    King = 11,
    Ace = 12,
}

/// Constant of all the values.
/// This is what `Value::values()` returns
const VALUES: [Value; 13] = [
    Value::Two,
    Value::Three,
    Value::Four,
    Value::Five,
    Value::Six,
    Value::Seven,
    Value::Eight,
    Value::Nine,
    Value::Ten,
    Value::Jack,
    Value::Queen,
    Value::King,
    Value::Ace,
];

impl Value {
    pub const fn values() -> [Self; 13] {
        VALUES
    }
}

impl From<Value> for char {
    fn from(value: Value) -> Self {
        match value {
            Value::Two => '2',
            Value::Three => '3',
            Value::Four => '4',
            Value::Five => '5',
            Value::Six => '6',
            Value::Seven => '7',
            Value::Eight => '8',
            Value::Nine => '9',
            Value::Ten => 'T',
            Value::Jack => 'J',
            Value::Queen => 'Q',
            Value::King => 'K',
            Value::Ace => 'A',
        }
    }
}

/// Enum for the four different suits.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy, Hash)]
pub enum Suit {
    Spade = 0,
    Club = 1,
    Heart = 2,
    Diamond = 3,
}

/// All of the `Suit`'s. This is what `Suit::suits()` returns.
const SUITS: [Suit; 4] = [Suit::Spade, Suit::Club, Suit::Heart, Suit::Diamond];

impl Suit {
    pub const fn suits() -> [Self; 4] {
        SUITS
    }
}

impl From<Suit> for char {
    fn from(value: Suit) -> Self {
        match value {
            Suit::Spade => 's',
            Suit::Club => 'c',
            Suit::Heart => 'h',
            Suit::Diamond => 'd',
        }
    }
}

/// Enum for card  enhancements
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy, Hash)]
pub enum Enhancement {
    Bonus,
    Mult,
    Wild,
    Glass,
    Steel,
    Stone,
    Gold,
    Lucky,
}

/// Enum for card  editions
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy, Hash)]
pub enum Edition {
    Base,
    Foil,
    Holographic,
    Polychrome,
    Negative,
}

/// Enum for card seals
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, PartialOrd, Eq, Ord, Debug, Clone, Copy, Hash)]
pub enum Seal {
    Gold,
    Red,
    Blue,
    Purple,
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Hash)]
pub struct Card {
    pub value: Value,
    pub suit: Suit,
    pub edition: Edition,
    pub enhancement: Option<Enhancement>,
    pub seal: Option<Seal>,
}

impl Card {
    pub fn new(value: Value, suit: Suit) -> Self {
        Self {
            value,
            suit,
            edition: Edition::Base,
            enhancement: None,
            seal: None,
        }
    }

    pub fn is_face(&self) -> bool {
        match self.value {
            Value::Jack | Value::Queen | Value::King => true,
            _ => false,
        }
    }

    pub fn is_even(&self) -> bool {
        self.value != Value::Ace && !self.is_face() && self.value as u16 % 2 == 0
    }

    pub fn is_odd(&self) -> bool {
        self.value == Value::Ace || !self.is_face() && self.value as u16 % 2 != 0
    }

    pub fn chips(&self) -> u16 {
        match self.value {
            Value::Two => 1,
            Value::Three => 2,
            Value::Four => 3,
            Value::Five => 4,
            Value::Six => 5,
            Value::Seven => 6,
            Value::Eight => 7,
            Value::Nine => 8,
            Value::Ten => 9,
            Value::Jack => 10,
            Value::Queen => 10,
            Value::King => 10,
            Value::Ace => 11,
        }
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Card({}{})",
            char::from(self.value),
            char::from(self.suit)
        )
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", char::from(self.value), char::from(self.suit))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constructor() {
        let c = Card::new(Value::King, Suit::Heart);
        assert_eq!(Value::King, c.value);
        assert_eq!(Suit::Heart, c.suit);
    }

    #[test]
    fn test_face() {
        let king = Card::new(Value::King, Suit::Heart);
        assert_eq!(king.is_face(), true);
        let two = Card::new(Value::Two, Suit::Diamond);
        assert_eq!(two.is_face(), false);
    }

    #[test]
    fn test_even_odd() {
        // ace is odd
        let ace = Card::new(Value::Ace, Suit::Spade);
        assert_eq!(ace.is_even(), false);
        assert_eq!(ace.is_odd(), true);

        // two is even
        let two = Card::new(Value::Two, Suit::Diamond);
        assert_eq!(two.is_even(), true);
        assert_eq!(two.is_odd(), false);

        // three is odd
        let three = Card::new(Value::Three, Suit::Heart);
        assert_eq!(three.is_even(), false);
        assert_eq!(three.is_odd(), true);

        // ten is even
        let ten = Card::new(Value::Ten, Suit::Heart);
        assert_eq!(ten.is_even(), true);
        assert_eq!(ten.is_odd(), false);

        //king is neither odd nor even
        let king = Card::new(Value::King, Suit::Club);
        assert_eq!(king.is_even(), false);
        assert_eq!(king.is_odd(), false);
    }
}

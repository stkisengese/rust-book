use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Suit {
    Heart,
    Diamonds,
    Spade,
    Club,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rank {
    Ace,
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
}

impl Suit {
    pub fn random() -> Suit {
        let mut rng = rand::thread_rng();
        let value = rng.gen_range(1..5);
        Suit::translate(value)
    }

    //  translate converts an integer value (u8) to a 
    //suit (1 -> Heart, 2 -> Diamonds, 3 -> Spade, 4 -> Club).
    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamonds,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => panic!("Invalid suit value"),
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let mut rng = rand::thread_rng();
        let value = rng.gen_range(1..14);
        Rank::translate(value)
    }

    // translate converts an integer value (u8) to a 
    // rank ( 1 -> Ace, 2 -> 2, .., 10 -> 10, 11 -> Jack, 12 -> Queen, 13 -> King).
    pub fn translate(value: u8) -> Rank {
        match value {
            1 => Rank::Ace,
            2 => Rank::Two,
            3 => Rank::Three,
            4 => Rank::Four,
            5 => Rank::Five,
            6 => Rank::Six,
            7 => Rank::Seven,
            8 => Rank::Eight,
            9 => Rank::Nine,
            10 => Rank::Ten,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            _ => panic!("Invalid rank value"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: Card) -> bool {
    card.rank == Rank::Ace && card.suit == Suit::Spade
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_winner_card() {
        let your_card = Card {
            rank: Rank::Ace,
            suit: Suit::Spade,
        };

        let result = winner_card(your_card);
        assert_eq!(result, true);
    }

    #[test]
    fn test_not_winner_card() {
        let your_card2 = Card {
            rank: Rank::Two, // Not Ace
            suit: Suit::Spade, // Not Club
        };
        let result = winner_card(your_card2);
        assert_eq!(result, false);
    }
}

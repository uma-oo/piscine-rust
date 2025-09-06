use rand::Rng;

#[derive(PartialEq, Debug)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,

}


#[derive(Debug)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Suit {
    pub fn random() -> Suit {
        let random_suit: u8 = rand::rng().random_range(1..=4);
        Suit::translate(random_suit)
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
             _ => panic!("Invalid rank value!"),
        }
        

    }
}

impl Rank {
    pub fn random() -> Rank {
        let random_rank = rand::rng().random_range(1..=13); 
        Rank::translate(random_rank)

    }

    pub fn translate(value: u8) -> Rank {
         match value {
            1=> Rank:: Ace,
            11=>Rank::Jack,
            12=>Rank::Queen,
            13=>Rank::King,
            2..=10=> Rank::Number(value),
             _ => panic!("Invalid rank value!"),
        
        }
    }
}

#[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,

}

pub fn winner_card(card: &Card) -> bool {
    card.suit == Suit::Spade
}
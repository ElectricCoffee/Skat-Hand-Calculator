use std::cmp::Ordering;

use crate::{types::CardValue, contract::Contract};

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
pub enum Suit {
    Clubs,
    Spades,
    Hearts,
    Diamonds
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
pub enum Rank {
    Jack,
    Ace,
    Ten,
    King,
    Queen,
    Nine,
    Eight,
    Seven,
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
pub struct Card(Suit, Rank);

impl Card {
    pub fn suit(&self) -> Suit {
        self.0
    }
    
    pub fn rank(&self) -> Rank {
        self.1
    }

    pub fn value(&self) -> CardValue {
        use self::Rank::*;
    
        match self.rank() {
            Ace     => 11,
            Ten     => 10,
            King    => 4,
            Queen   => 3,
            Jack    => 2,
            _       => 0
        }
    }
    
    pub fn value_null(&self) -> CardValue {
        if self.rank() == Rank::Ten {
            1
        } else {
            self.value()
        }
    }
    
    pub fn compare(&self, other: &Self, contract: Contract) -> Ordering {
        use self::Rank::Jack;
        
        // null contract
        if contract == Contract::Null {
            return self.compare_null(other);
        }
        
        // taking care of jacks
        // contracts trump and grand
        if self.rank() == Jack && other.rank() == Jack {
            return self.suit().cmp(&other.suit())
        }
        
        if self.rank() == Jack && other.rank() != Jack {
            return Ordering::Greater;
        }
        
        if self.rank() != Jack && other.rank() == Jack {
            return Ordering::Less;
        }
        
        // just trump contract
        if let Contract::Trump(trump) = contract {
            // taking care of trumps
            if self.suit() == trump && other.suit() != trump {
                return Ordering::Greater;
            }
            
            if self.suit() != trump && other.suit() == trump {
                return Ordering::Less;
            }
        }
        
        // everything else
        if self.suit() != other.suit() {
            return Ordering::Greater;
        }
        
        if self.suit() == other.suit() {
            return self.value().cmp(&other.value());
        }
        
        Ordering::Equal
    }
    
    fn compare_null(&self, other: &Self) -> Ordering {
        if self.suit() == other.suit() {
            self.value_null().cmp(&other.value_null())
        } else {
            Ordering::Greater
        }
    }
}
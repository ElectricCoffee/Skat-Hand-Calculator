#![allow(unused)]
use std::cmp::Ordering;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
enum Suit {
    Clubs,
    Spades,
    Hearts,
    Diamonds
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
enum Rank {
    Jack,
    Ace,
    Ten,
    King,
    Queen,
    Nine,
    Eight,
    Seven,
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
enum Matador {
    With(u32),
    Without(u32),
}

type GameValue = u32;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
enum SubContract {
    Skat,               // Player picks up the Skat
    Hand,               // Player does NOT pick up the Skat
    OuvertSkat,         // Open hand with the Skat; only valid in Null games
    OuvertHand,         // Open hand without the Skat
    Schneider,          // Declare scoring at least 90 Card Points
    Schwarz,            // Declare winning every trick (120 Card Points)
    OuvertSchneider,    // Declare Schneider while playing with an open hand
    OuvertSchwarz,      // Declare Schwarz while playing with an open hand
}

impl Default for SubContract {
    fn default() -> Self { Self::Skat }
}

impl SubContract {
    fn value(&self) -> GameValue {
        use self::SubContract::*;
        match self {
            Skat            => 1,
            Hand            => 2, // Skat + 1
            OuvertHand      => 3, // Hand + 1
            OuvertSkat      => panic!("This is only for use with Null contracts and shouldn't be valued!"),
            Schneider       => 4, // Hand + Announced Schneider + Actual Schneider
            OuvertSchneider => 5,
            Schwarz         => 6, // Hand + Announced Schneider + Actual Schneider + Announced Schwarz + Actual Schwarz
            OuvertSchwarz   => 7,
        }
    }
}

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
enum Contract {
    Trump(Suit),
    Grand,
    Null,
}

impl Contract {
    fn value(&self, sub: SubContract) -> GameValue {
        use self::Contract::*;
        use self::SubContract::*;
        use self::Suit::*;
        
        match (self, sub) {
            // null hands have fixed values; the value of the subcontract doesn't actually matter
            (Null, Skat)            => 32,
            (Null, Hand)            => 35,
            (Null, OuvertSkat)      => 46, // special subcontract only used for null hands
            (Null, OuvertHand)      => 59, 
            // everything else is some base-value + whatever the sub-contract is valued at
            (Trump(Diamonds), sub)  =>  9 + sub.value(),
            (Trump(Hearts),   sub)  => 10 + sub.value(),
            (Trump(Spades),   sub)  => 11 + sub.value(),
            (Trump(Clubs),    sub)  => 12 + sub.value(),
            (Grand,           sub)  => 24 + sub.value(),
            (_, _)                  => panic!("Invalid contract"),
        }
    }
}

type CardValue = u32;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq)]
struct Card(Suit, Rank);

impl Card {
    fn suit(&self) -> Suit {
        self.0
    }
    
    fn rank(&self) -> Rank {
        self.1
    }

    fn value(&self) -> CardValue {
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
    
    fn value_null(&self) -> CardValue {
        if self.rank() == Rank::Ten {
            1
        } else {
            self.value()
        }
    }
    
    fn compare(&self, other: &Self, contract: Contract) -> Ordering {
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

struct Hand(Vec<Card>);

impl Hand {
    fn sort_default(&mut self) {
        self.sort(Contract::Grand);
    }

    fn sort(&mut self, contract: Contract) {
        self.0.sort_by(|a, b| a.compare(b, contract));
    }

    fn hand_value(&self, contract: Contract) {
        assert!(self.0.len() == 10);
        
        
    }
}

fn main() {
    println!("Hello, Rust!");
}
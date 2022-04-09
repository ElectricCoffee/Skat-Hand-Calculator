use crate::types::GameValue;
use crate::subcontract::SubContract;
use crate::card::{Suit, Rank};

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
pub enum Contract {
    Trump(Suit),
    Grand,
    Null,
}

impl Contract {
    pub fn value(&self, sub: SubContract) -> GameValue {
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
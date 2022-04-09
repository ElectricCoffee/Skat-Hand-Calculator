use crate::{card::Card, contract::Contract};

pub struct Hand(Vec<Card>);


impl Hand {
    pub fn sort_default(&mut self) {
        self.sort(Contract::Grand);
    }

    pub fn sort(&mut self, contract: Contract) {
        self.0.sort_by(|a, b| a.compare(b, contract));
    }

    pub fn hand_value(&self, contract: Contract) {
        assert!(self.0.len() == 10);
        
        
    }
}
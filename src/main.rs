#![allow(unused)]
mod contract;
mod subcontract;
mod types;
mod card;
mod hand;

use std::cmp::Ordering;

use card::Card;
use contract::Contract;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
enum Matador {
    With(u32),
    Without(u32),
}

fn main() {
    println!("Hello, Rust!");
}
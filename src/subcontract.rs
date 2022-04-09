use crate::types::GameValue;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Copy, Clone)]
pub enum SubContract {
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
    pub fn value(&self) -> GameValue {
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
use crate::prelude::*;

mod negative;
pub use negative::*;

mod positive;
pub use positive::*;



pub struct PersonalityTraits(pub Vec<PersonalityTrait>);

pub enum PersonalityTrait{
    Positive(PositivePersonalityTrait),
    Negative(NegativePersonalityTrait)
}

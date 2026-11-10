use super::whole::Whole;
use super::fraction::Fraction;

pub(crate)struct UnwrappedValue<W:Copy,F:Copy>{
    whole_part:Whole<W>,
    fraction_part:Fraction<F>
}

pub struct Value<W:Copy,F:Copy>{
    value:Option<UnwrappedValue<W,F>>
}
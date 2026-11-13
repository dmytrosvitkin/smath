use crate::traits::{Integer, UnsignedInteger};

use super::whole::Whole;
use super::fraction::Fraction;

#[derive(Clone,Copy,PartialEq)]
pub struct Value<W:Integer=i32,F:UnsignedInteger=u32>{
    whole_part:Whole<W>,
    fraction_part:Fraction<F>
}

impl<W:Integer,F:UnsignedInteger>Value<W,F>{
    pub const fn from_parts(){}
}

impl<W:Integer,F:UnsignedInteger>Value<W,F>{
    pub const fn from_whole_number(whole_number:W)->Self{
        Self::from_unwrapped_parts(
            Whole::from_raw(whole_number),
            Fraction::zero()
        )
    }

    pub const fn as_whole_number_option(&self)->Option<W>{
        match self.whole_part(){
            Some(unwrapped_whole_part)=>Some(unwrapped_whole_part.as_raw()),
            None=>None
        }
    }

    pub const fn as_whole_number_unwrapped(&self)->W{
        self.whole_part_unwrapped().as_raw()
    }
}
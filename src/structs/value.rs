use crate::traits::{Integer, UnsignedInteger};

use super::whole::Whole;
use super::fraction::Fraction;

#[derive(Clone,Copy,PartialEq)]
pub struct Value<W:Integer=i32,F:UnsignedInteger=u32>{
    whole_part:Whole<W>,
    fraction_part:Fraction<F>
}

impl<W:Integer,F:UnsignedInteger>Value<W,F>{
    pub const fn from_parts(whole_part:Whole<W>,fraction_part:Fraction<F>)->Self{
        Self{
            whole_part,
            fraction_part
        }
    }

    pub const fn from_raw_parts(raw_whole_part:W,raw_fraction_part:F)->Self{
        Self::from_parts(
            Whole::from_raw(raw_whole_part),
            Fraction::from_raw(raw_fraction_part)
        )
    }

    pub const fn whole_part(&self)->Whole<W>{
        self.whole_part
    }

    pub const fn fraction_part(&self)->Fraction<F>{
        self.fraction_part
    }

    pub const fn raw_whole_part(&self)->W{
        self.whole_part().as_raw()
    }

    pub const fn raw_fraction_part(&self)->F{
        self.fraction_part().as_raw()
    }

    pub const fn as_tuple(&self)->(Whole<W>,Fraction<F>){
        (self.whole_part(),self.fraction_part())
    }

    pub const fn as_raw_tuple(&self)->(W,F){
        (self.raw_whole_part(),self.raw_fraction_part())
    }

    pub const fn change_whole_part(&mut self,new_whole_part:Whole<W>)->&mut Self{
        self.whole_part=new_whole_part;
        self
    }

    pub const fn change_fraction_part(&mut self,new_fraction_part:Fraction<F>)->&mut Self{
        self.fraction_part=new_fraction_part;
        self
    }

    pub const fn change_raw_whole_part(&mut self,new_raw_whole_part:W)->&mut Self{
        self.change_whole_part(Whole::from_raw(new_raw_whole_part))
    }

    pub const fn change_raw_fraction_part(&mut self,new_raw_fraction_part:F)->&mut Self{
        self.change_fraction_part(Fraction::from_raw(new_raw_fraction_part))
    }
}
use core::fmt;
use crate::new::traits::{Integer, UnsignedInteger};
use crate::new::utils::bytes_as_str;

use super::whole::Whole;
use super::fraction::Fraction;

/// A *fixed-point* number *value*, which consists of a whole and a fraction part.
#[derive(Clone,Copy,PartialEq,Debug)]
pub struct Value<W:fmt::Display+Integer=i32,F:fmt::Display+UnsignedInteger=u32>{
    whole_part:Whole<W>,
    fraction_part:Fraction<F>
}

impl<W:Integer+fmt::Display,F:UnsignedInteger+fmt::Display>Value<W,F>{
    /// Construct `Value` from `Whole` and `Fraction` parts.
    pub const fn from_parts(whole_part:Whole<W>,fraction_part:Fraction<F>)->Self{
        Self{
            whole_part,
            fraction_part
        }
    }

    /// Construct `Value` from raw `Whole` and `Fraction` parts.
    pub const fn from_raw_parts(raw_whole_part:W,raw_fraction_part:F)->Self{
        Self::from_parts(
            Whole::from_raw(raw_whole_part),
            Fraction::from_raw(raw_fraction_part)
        )
    }

    /// Return whole part of the `Value` as `Whole`.
    pub const fn whole_part(&self)->Whole<W>{
        self.whole_part
    }

    /// Return fraction part of the `Value` as `Fraction`.
    pub const fn fraction_part(&self)->Fraction<F>{
        self.fraction_part
    }

    /// Return whole part of the `Value` as raw integer*.
    pub const fn raw_whole_part(&self)->W{
        self.whole_part().as_raw()
    }

    /// Return fraction part of the `Value` as raw integer*.
    pub const fn raw_fraction_part(&self)->F{
        self.fraction_part().as_raw()
    }

    /// Construct and return a tuple consisting of a `Whole` and `Fraction` parts.
    pub const fn as_tuple(&self)->(Whole<W>,Fraction<F>){
        (self.whole_part(),self.fraction_part())
    }

    /// Construct and return a tuple consisting of a `Whole` and `Fraction` parts.
    pub const fn as_raw_tuple(&self)->(W,F){
        (self.raw_whole_part(),self.raw_fraction_part())
    }

    /// Change the `Whole` part of the `Value` by another `Whole` part.
    pub const fn change_whole_part(&mut self,new_whole_part:Whole<W>)->&mut Self{
        self.whole_part=new_whole_part;
        self
    }

    /// Change the `Fraction` part of the `Value` by another `Fraction` part.
    pub const fn change_fraction_part(&mut self,new_fraction_part:Fraction<F>)->&mut Self{
        self.fraction_part=new_fraction_part;
        self
    }

    /// Change the `Whole` part of the `Value` by another (raw) `Whole` part.
    pub const fn change_raw_whole_part(&mut self,new_raw_whole_part:W)->&mut Self{
        self.change_whole_part(Whole::from_raw(new_raw_whole_part))
    }

    /// Chnage the `Fraction` part of the `Value` by another (raw) `Fraction` part.
    pub const fn change_raw_fraction_part(&mut self,new_raw_fraction_part:F)->&mut Self{
        self.change_fraction_part(Fraction::from_raw(new_raw_fraction_part))
    }

    /// Construct and return a `Value` equal to 0.
    pub const fn zero()->Self{
        Self::from_parts(
            Whole::zero(),
            Fraction::zero()
        )
    }

    /// Construct and return a *minimal* `Value`.
    pub const fn min()->Self{
        Self::from_parts(
            Whole::min(),
            Fraction::min()
        )
    }

    /// Construct and return a *maximal* `Value`.
    pub const fn max()->Self{
        Self::from_parts(
            Whole::max(),
            Fraction::max()
        )
    }
}

//impl<W:Integer+fmt::Display,F:UnsignedInteger+fmt::Display>fmt::Display for Value<W,F>

impl fmt::Display for Value<i32,u32>{
    fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result{
        let mut raw_whole_part:i32=self.raw_whole_part();
        let mut raw_fraction_part:u32=self.raw_fraction_part();
        //
        if raw_whole_part<0&&raw_fraction_part!=0{
            raw_whole_part+=1;
            raw_fraction_part=u32::MAX-raw_fraction_part+1
        }
        //
        write!(f,"{}.{}",raw_whole_part,bytes_as_str(&Fraction::from_raw(raw_fraction_part).to_bytes()))
    }
}
use core::fmt;
use crate::new::{traits::UnsignedInteger, utils::match_digit_byte};

/// Number in range `[0;1[` expressed by a value of *unsigned* type.
#[derive(Clone,Copy,PartialEq,Debug)]
pub struct Fraction<T:fmt::Display+UnsignedInteger=u32>{
    fraction:T
}

impl<T:UnsignedInteger+fmt::Display>Fraction<T>{
    pub const fn from_raw(raw_fraction:T)->Fraction<T>{
        Fraction{fraction:raw_fraction}
    }

    pub const fn as_raw(&self)->T{
        self.fraction
    }

    pub const fn zero()->Self{
        Self::from_raw(T::ZERO)
    }
    
    pub const fn min()->Self{
        Self::from_raw(T::MIN)
    }
    
    pub const fn max()->Self{
        Self::from_raw(T::MAX)
    }
}

impl Fraction<u16>{
    pub const fn to_bytes(&self)->([u8;40],usize){
        let mut raw_fraction:u16=self.as_raw();
        let mut result:[u8;40]=[0;40];
        let mut index_counter:usize=1;

        if raw_fraction==0{
            result[40-1]=match_digit_byte(0)
        }
        else{
            while index_counter<40&&raw_fraction!=0{
                let raw_fraction_digit_byte:u8=match_digit_byte((raw_fraction%10)as u8);
                raw_fraction/=10;
                result[40-index_counter]=raw_fraction_digit_byte;
                index_counter+=1;
            }
            index_counter-=1
        }

        (result,index_counter)
    }
}
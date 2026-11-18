use core::fmt;
use crate::new::traits::UnsignedInteger;

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
    pub(crate)const fn to_bytes(&self)->[u8;40]{
        let result:[u8;40]=[0;40];
        result
    }
}
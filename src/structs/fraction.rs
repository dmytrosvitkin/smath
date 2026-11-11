use crate::traits::UnsignedInteger;

/// Number in range `[0;1[` expressed by a value of *unsigned* type.
#[derive(Clone,Copy,PartialEq)]
pub struct Fraction<T:UnsignedInteger=u32>{
    fraction:T
}

impl<T:UnsignedInteger>Fraction<T>{
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
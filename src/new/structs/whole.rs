use core::fmt;
use crate::new::traits::Integer;

/// Whole number expressed by an *integer* or *unsigned* type.
#[derive(Clone,Copy,PartialEq,Debug)]
pub struct Whole<T:fmt::Display+Integer=i32>{
    whole:T
}

impl<T:Integer+fmt::Display>Whole<T>{
    pub const fn from_raw(raw_whole:T)->Whole<T>{
        Whole{whole:raw_whole}
    }

    pub const fn as_raw(&self)->T{
        self.whole
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
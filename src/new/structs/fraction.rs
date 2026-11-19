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

macro_rules!impl_to_bytes_types{
    ($(($fraction_type:ident,$result_array_length:literal)),*$(,)?)=>{
        $(
            impl Fraction<$fraction_type>{
                pub const fn to_bytes(&self)->([u8;$result_array_length],usize){
                    let mut raw_fraction:$fraction_type=self.as_raw();
                    let mut result:[u8;$result_array_length]=[0;$result_array_length];
                    let mut index_counter:usize=1;
                    let mut remainder:$fraction_type=raw_fraction;
                    let fraction_type_max:$fraction_type=<$fraction_type>::MAX;

                    while index_counter<$result_array_length+1{
                        let tenth:$fraction_type=fraction_type_max/10;
                        let quotient:$fraction_type=fraction_type_max;
                        let mut fit_counter:$fraction_type=0;
                        while tenth*fit_counter+(quotient*fit_counter)/10<remainder{
                            remainder=fraction_type_max-tenth*fit_counter+(quotient*fit_counter)/10;
                            fit_counter+=1
                        }
                        index_counter+=1;
                        result[index_counter]=match_digit_byte(fit_counter as u8);
                    }

                    (result,index_counter)
                }
            }
        )*
    }
}

#[cfg(target_pointer_width="32")]
impl_to_bytes_types!{
    (u8,3),
    (u16,5),
    (u32,10),
    (u64,20),
    (u128,39),
    (usize,10)
}

#[cfg(target_pointer_width="64")]
impl_to_bytes_types!{
    (u8,3),
    (u16,5),
    (u32,10),
    (u64,20),
    (u128,39),
    (usize,20)
}
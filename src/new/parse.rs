#![allow(unused_imports)]
use core::fmt;
use crate::{Fraction,Integer,UnsignedInteger,Whole};
use crate::new::utils::match_byte_digit;
use crate::new::Value;

impl Value<i16,u16>{
    pub const fn from_str_16(input_value_str:&str)->Self{
        let input_value_bytes:&[u8]=input_value_str.as_bytes();
        let input_value_bytes_len:usize=input_value_bytes.len();

        if input_value_bytes_len==0{panic!("Number length has to be larger than 0!")}
        else if input_value_bytes_len==1&&input_value_bytes[0]==b'-'{panic!("Negative number must contain at least 1 digit!")}

        let mut whole_part:i16=0;
        let mut fraction_part:u16=0;

        let mut whole_index_counter:usize=0;
        let mut fraction_index_counter:usize=0;
        let mut is_negative:bool=false;// for -0.x cases.

        {
            if input_value_bytes[0]==b'-'{
                is_negative=true;
                whole_index_counter+=1;
                
                while whole_index_counter<input_value_bytes_len{
                    let input_value_byte:u8=input_value_bytes[whole_index_counter];
                    if input_value_byte==b'.'{break}
                    let digit:u8=match_byte_digit(input_value_byte);

                    if whole_part<(i16::MIN+digit as i16)/10{panic!("Integer underflow!")}
                    whole_part*=10;
                    whole_part-=digit as i16;

                    whole_index_counter+=1
                }
            }
            else{
                while whole_index_counter<input_value_bytes_len{
                    let input_value_byte:u8=input_value_bytes[whole_index_counter];
                    if input_value_byte==b'.'{break}
                    let digit:u8=match_byte_digit(input_value_byte);

                    if whole_part>(i16::MAX-digit as i16)/10{panic!("Integer Overflow!")}
                    whole_part*=10;
                    whole_part+=digit as i16;

                    whole_index_counter+=1
                }
            }
        }

        Self::from_raw_parts(
            whole_part,
            fraction_part
        )
    }
}

macro_rules!from_str_types{
    ($whole_type:ty,$fraction_type:ty)=>{
        impl Value<$whole_type,$fraction_type>{
            pub const fn from_str(input_value_str:&str)->Self{
                let input_value_bytes=input_value_str.as_bytes();

                Self::from_raw_parts(
                    0,
                    0
                )
            }
        }
    }
}

macro_rules!type_cross_product{
    (whole_type:[];fraction_type:[$($fraction_type:ty),*])=>{};

    (whole_type:[$whole_type:ty$(,$whole_rest:ty)*];fraction_type:[$($fraction_type:ty),*])=>{
        $(from_str_types!($whole_type,$fraction_type);)*
        
        type_cross_product!{
            whole_type:[$($whole_rest),*];
            fraction_type:[$($fraction_type),*]
        }
    };
}

macro_rules!make_from_str_types{
    (
        whole_types:[$($whole_type:ty),*$(,)?],
        fraction_types:[$($fraction_type:ty),*$(,)?]
    )=>{
        type_cross_product!{
            whole_type:[$($whole_type),*];
            fraction_type:[$($fraction_type),*]
        }
    }
}

make_from_str_types!{
    whole_types:[
        u8,i8,
        u16,i16,
        u32,i32,
        u64,i64,
        u128,i128,
        usize,isize
    ],
    fraction_types:[
        u8,
        u16,
        u32,
        u64,
        u128,
        usize,
    ]
}

#[macro_export]
macro_rules!value{
    ($input_value_str:expr,$whole_type:ty,$fraction_type:ty)=>{{
        const RES:crate::Value<$whole_type,$fraction_type>=crate::Value::<$whole_type,$fraction_type>::from_str(stringify!($input_value_str));
        RES
    }};

    ($input_value_str:expr,$whole_type:ty)=>{{
        const RES:crate::Value<$whole_type,u32>=crate::Value::<$whole_type,u32>::from_str(stringify!($input_value_str));
        RES
    }};

    ($input_value_str:expr)=>{{
        const RES:crate::Value<i32,u32>=crate::Value::<i32,u32>::from_str(stringify!($input_value_str));
        RES
    }}
}
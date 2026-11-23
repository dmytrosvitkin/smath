#![allow(unused_imports)]
use core::fmt;
use crate::{Fraction,Integer,UnsignedInteger,Whole};
use crate::new::utils::match_byte_digit;
use crate::new::Value;

macro_rules!from_str_types{
    ($whole_type:ty,$fraction_type:ty)=>{
        impl Value<$whole_type,$fraction_type>{
            pub const fn from_str(input_value_str:&str)->Self{
                let input_value_bytes:&[u8]=input_value_str.as_bytes();
                let input_value_bytes_len:usize=input_value_bytes.len();

                if input_value_bytes_len==0{panic!("The value cannot be empty.")}//compile_error!("The value cannot be empty.")}

                let mut index_counter:usize=0;
                let mut is_negative:bool=false;

                if input_value_bytes[0]==b'-'{
                    if input_value_bytes_len==1{panic!("The value cannot consist of only `-`")}
                    is_negative=true;
                    index_counter+=1
                }

                let whole_type_min:$whole_type=<$whole_type>::MIN;
                let whole_type_max:$whole_type=<$whole_type>::MAX;
                let fraction_type_max:$fraction_type=<$fraction_type>::MAX;

                let mut whole_part:$whole_type=0;
                let mut fraction_part:$fraction_type=0;
                let mut fraction_base:$fraction_type=fraction_type_max;

                let mut primary_quotient:u8=0;

                if is_negative{
                    while index_counter<input_value_bytes_len{
                        let input_value_byte:u8=input_value_bytes[index_counter];

                        if input_value_byte==b'.'{
                            index_counter+=1;
                            break
                        }

                        let digit:i8=match_byte_digit(input_value_byte)as i8;

                        if whole_part<whole_type_min/10||(whole_part==whole_type_min/10&&digit>-(whole_type_min as i8%10)){panic!("Integer Underflow.")}
                        whole_part*=10;whole_part-=digit as $whole_type;

                        index_counter+=1
                    }
                }

                else{
                    while index_counter<input_value_bytes_len{
                        let input_value_byte:u8=input_value_bytes[index_counter];

                        if input_value_byte==b'.'{
                            index_counter+=1;
                            break
                        }

                        let digit:u8=match_byte_digit(input_value_byte);

                        if whole_part>whole_type_max/10||(whole_part==whole_type_max/10&&digit>(whole_type_max%10)as u8){panic!("Integer Overflow.")}// Overflow protection.
                        whole_part*=10;whole_part+=digit as $whole_type;

                        index_counter+=1
                    }
                }

                while index_counter<input_value_bytes_len{
                    let digit:$fraction_type=match_byte_digit(input_value_bytes[index_counter])as $fraction_type;

                    let secondary_quotient:u8=primary_quotient%10;
                    primary_quotient/=10;

                    fraction_part+=(fraction_base/10)*(digit)+primary_quotient as $fraction_type;
                    primary_quotient=((fraction_base%10)*digit)as u8+secondary_quotient;
                    
                    if fraction_base==0{break}
                    fraction_base/=10;

                    index_counter+=1
                }
                fraction_part+=(primary_quotient/10+(primary_quotient%10)/6)as $fraction_type;// 10 replaced by 9 for a while.

                #[allow(unused_comparisons)]// Useful for negative signed integers.
                if whole_part<0{
                    if fraction_part!=0{
                        if whole_part==whole_type_min{panic!("Integer Underflow")}
                        whole_part-=1
                    }

                    fraction_part=fraction_type_max-fraction_part;
                    if fraction_part==fraction_type_max{fraction_part=0}
                    else{fraction_part+=1}
                }

                Self::from_raw_parts(
                    whole_part,
                    fraction_part
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
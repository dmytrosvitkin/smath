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
                //
                if input_value_bytes_len==0{panic!("Number length has to be larger than 0!")}
                else if input_value_bytes_len==1&&input_value_bytes[0]==b'-'{panic!("Negative number must contain at least 1 digit!")}
                //
                let mut whole_part:$whole_type=0;
                let mut fraction_part:$fraction_type=0;
                //
                let mut whole_index_counter:usize=0;
                let mut fraction_index_counter:usize=0;
                let mut is_negative:bool=false;// for -0.x cases.
                //
                let whole_type_min:$whole_type=<$whole_type>::MIN;
                let whole_type_max:$whole_type=<$whole_type>::MAX;
                let fraction_type_max:$fraction_type=<$fraction_type>::MAX;

                {
                    if input_value_bytes[0]==b'-'{
                        is_negative=true;
                        whole_index_counter+=1;
                        
                        while whole_index_counter<input_value_bytes_len{
                            let input_value_byte:u8=input_value_bytes[whole_index_counter];
                            //
                            if input_value_byte==b'.'{
                                whole_index_counter+=1;
                                break
                            }
                            //
                            let digit:u8=match_byte_digit(input_value_byte);
                            //
                            if whole_part<(whole_type_min+digit as $whole_type)/10{panic!("Integer underflow!")}
                            whole_part*=10;
                            whole_part-=digit as $whole_type;
                            //
                            whole_index_counter+=1
                        }
                    }
                    else{
                        while whole_index_counter<input_value_bytes_len{
                            let input_value_byte:u8=input_value_bytes[whole_index_counter];
                            //
                            if input_value_byte==b'.'{
                                whole_index_counter+=1;
                                break
                            }
                            //
                            let digit:u8=match_byte_digit(input_value_byte);
                            //
                            if whole_part>(whole_type_max-digit as $whole_type)/10{panic!("Integer Overflow!")}
                            whole_part*=10;
                            whole_part+=digit as $whole_type;
                            //
                            whole_index_counter+=1
                        }
                    }
                }

                {
                    let mut tenth:$fraction_type=0;
                    //
                    let scale_tenth:$fraction_type=fraction_type_max/10;// Alternatively `(fraction_type_max-9)/10+1`.
                    let scale_remainder:$fraction_type=fraction_type_max%10+1;// Alternatively `(fraction_type_max-9)%10`.

                    while whole_index_counter+fraction_index_counter<input_value_bytes_len{
                        let digit:u8=match_byte_digit(input_value_bytes[input_value_bytes_len-fraction_index_counter-1]);
                        //
                        let previous_fraction_part:$fraction_type=fraction_part/10;
                        let previous_tenth:$fraction_type=fraction_part%10;
                        //
                        let tenth_sum:$fraction_type=tenth+digit as $fraction_type*scale_remainder+previous_tenth;
                        let secondary_fraction_part:$fraction_type=tenth_sum/10;
                        tenth=tenth_sum%10;
                        //
                        if digit as $fraction_type>(fraction_type_max-previous_fraction_part)/scale_tenth||digit as $fraction_type*scale_tenth+previous_fraction_part>fraction_type_max-secondary_fraction_part{
                            fraction_part=fraction_type_max;
                            break
                        }
                        fraction_part=digit as $fraction_type*scale_tenth+previous_fraction_part+secondary_fraction_part;
                        //
                        fraction_index_counter+=1;
                    }
                    
                    if tenth>=5&&fraction_part<fraction_type_max{
                        fraction_part+=1
                    }
                }

                if is_negative&&fraction_part!=0{
                    if whole_part==whole_type_min{panic!("Integer Underflow!")}
                    whole_part-=1;
                    fraction_part=fraction_type_max-fraction_part+1
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
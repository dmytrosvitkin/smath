use crate::traits::*;
use crate::{Value, Whole, Fraction};

/*
#[macro_export]
macro_rules!value{
    ($w:ty,$f:ty,$value:expr)=>{Value::<$w,$f>::parse(stringify!($value))};// If types are explicit.
    ($value:expr)=>{Value::<i32,u32>::parse(stringify!($value))}// Defaults to i32 and u32 types otherwise.
}

#[macro_export]
macro_rules!impl_value_macro_for{
    ($w:ty,$f:ty)=>{
        impl Value<$w,$f>{
            pub const fn parse(input_value_str:&str)->Self{
                Self::none();
            }
        }
    };
}

// Generate parse implementations for supported type pairs
impl_value_macro_for!(i32, u32);
impl_value_macro_for!(i64, u32);
impl_value_macro_for!(u32, u32);
impl_value_macro_for!(u128, u128);
*/

impl Value<i16,u16>{
    pub const fn from_str(input_value_str:&str)->Self{
        let input_value_bytes:&[u8]=input_value_str.as_bytes();
        let input_value_bytes_len:u8=input_value_bytes.len()as u8;// `u8` is already bigger than the biggest digit's length.

        if input_value_bytes_len==0||!input_value_bytes.is_ascii(){return Self::none()}

        let mut index_counter:u8=0;
        let mut is_negative:bool=false;

        if input_value_bytes[0]==b'-'{
            if input_value_bytes_len==1{return Self::none()}
            is_negative=true;
            index_counter+=1
        }

        let mut whole_part:i16=0;

        while index_counter<input_value_bytes_len{
            let input_value_byte:u8=input_value_bytes[index_counter as usize];

            if input_value_byte==b'.'{
                index_counter+=1;
                break
            }

            let digit:i8=match_byte_digit(input_value_byte);
            if digit>9||digit<0{return Self::none()}
            //if is_negative{digit*=-1}

            if is_negative{
                if whole_part<i16::MIN/10||(whole_part==i16::MIN/10&&digit>-(i16::MIN%10)as i8){return Self::none()}// Underflow protection.
                whole_part*=10;whole_part-=digit as i16
            }
            else{
                if(i16::MAX-whole_part)/10<digit as i16{return Self::none()}// Overflow protection.
                whole_part*=10;whole_part+=digit as i16
            }

            index_counter+=1
        }

        Self::from_unwrapped_parts(
            Whole::from_raw(whole_part),
        Fraction::zero()
        )
    }
}

const fn match_byte_digit(byte:u8)->i8{// `u8::MAX` or anything outside of 0-9 is considered an illegal symbol.
    match byte{
        b'0'=>0,
        b'1'=>1,
        b'2'=>2,
        b'3'=>3,
        b'4'=>4,
        b'5'=>5,
        b'6'=>6,
        b'7'=>7,
        b'8'=>8,
        b'9'=>9,
        _=>i8::MAX,
    }
}
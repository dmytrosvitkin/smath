use crate::traits::*;
use crate::{Value, Whole, Fraction};

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
        let mut fraction_part:u16=0;
        let mut fraction_base:u16=u16::MAX;

        let mut primary_quotient:u8=0;
        let mut secondary_quotient:u8=0;

        if is_negative{
            while index_counter<input_value_bytes_len{
                let input_value_byte:u8=input_value_bytes[index_counter as usize];

                if input_value_byte==b'.'{
                    index_counter+=1;
                    break
                }

                let digit:i8=match_byte_digit(input_value_byte);
                if digit>9||digit<0{return Self::none()}

                if whole_part<i16::MIN/10||(whole_part==i16::MIN/10&&digit>-(i16::MIN%10)as i8){return Self::none()}// Underflow protection.
                whole_part*=10;whole_part-=digit as i16;

                index_counter+=1
            }

            while index_counter<input_value_bytes_len{
                index_counter+=1;
                panic!("[!] NEGATIVE INTEGERS NOT FINISHED")
            }
        }
        else{
            while index_counter<input_value_bytes_len{
                let input_value_byte:u8=input_value_bytes[index_counter as usize];

                if input_value_byte==b'.'{
                    index_counter+=1;
                    break
                }

                let digit:i8=match_byte_digit(input_value_byte);
                if digit>9||digit<0{return Self::none()}

                if whole_part>i16::MAX/10||(whole_part==i16::MAX/10&&digit<(i16::MAX%10)as i8){return Self::none()}// Overflow protection.
                whole_part*=10;whole_part+=digit as i16;

                index_counter+=1
            }

            if index_counter<input_value_bytes_len{
                let digit:u16=match_byte_digit(input_value_bytes[index_counter as usize])as u16;
                if digit>9{return Self::none()}

                fraction_part=(fraction_base/10)*(digit);
                primary_quotient=((fraction_base%10+1+2)*digit)as u8;// 2 is an initial stretch coefficient, equal to 0.0045% of the u16:MAX, covering the error caused by x.9999 being equal to u16:MAX-2.

                fraction_base/=10;
                index_counter+=1
            }
            while index_counter<input_value_bytes_len{
                let digit:u16=match_byte_digit(input_value_bytes[index_counter as usize])as u16;
                if digit>9{return Self::none()}

                secondary_quotient=primary_quotient%10;
                primary_quotient/=10;

                fraction_part+=(fraction_base/10)*(digit)+primary_quotient as u16;
                primary_quotient=((fraction_base%10)*digit)as u8+secondary_quotient;
                
                if fraction_base==0{break}
                fraction_base/=10;

                index_counter+=1
            }
            fraction_part+=(primary_quotient/10)as u16
        }

        Self::from_unwrapped_parts(
            Whole::from_raw(whole_part),
        Fraction::from_raw(fraction_part)
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
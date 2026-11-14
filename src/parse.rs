use crate::{traits::*,utils::match_byte_digit};
use crate::{Value,Whole,Fraction};

impl Value<i16,u16>{
    pub const fn from_str(input_value_str:&str)->Self{
        let input_value_bytes:&[u8]=input_value_str.as_bytes();
        let input_value_bytes_len:u8=input_value_bytes.len()as u8;// `u8` is already bigger than the biggest digit's length.

        if input_value_bytes_len==0{panic!("The value cannot be empty.")}//compile_error!("The value cannot be empty.")}

        let mut index_counter:u8=0;
        let mut is_negative:bool=false;

        if input_value_bytes[0]==b'-'{
            if input_value_bytes_len==1{panic!("The value cannot consist of only `-`")}
            is_negative=true;
            index_counter+=1
        }

        let whole_type_min:i16=i16::MIN;
        let whole_type_max:i16=i16::MAX;
        let fraction_type_max:u16=u16::MAX;

        let mut whole_part:i16=0;
        let mut fraction_part:u16=0;
        let mut fraction_base:u16=fraction_type_max;

        let mut primary_quotient:u8=0;
        let mut secondary_quotient:u8;

        if is_negative{
            while index_counter<input_value_bytes_len{
                let input_value_byte:u8=input_value_bytes[index_counter as usize];

                if input_value_byte==b'.'{
                    index_counter+=1;
                    break
                }

                let digit:u8=match_byte_digit(input_value_byte);

                if whole_part<i16::MIN/10||(whole_part==whole_type_min/10&&digit>-(whole_type_min%10)as u8){panic!("Integer Underflow.")}
                whole_part*=10;whole_part-=digit as i16;

                index_counter+=1
            }
        }

        else{
            while index_counter<input_value_bytes_len{
                let input_value_byte:u8=input_value_bytes[index_counter as usize];

                if input_value_byte==b'.'{
                    index_counter+=1;
                    break
                }

                let digit:u8=match_byte_digit(input_value_byte);

                if whole_part>whole_type_max/10||(whole_part==whole_type_max/10&&digit<(whole_type_max%10)as u8){panic!("Integer Overflow.")}// Overflow protection.
                whole_part*=10;whole_part+=digit as i16;

                index_counter+=1
            }
        }

        if index_counter<input_value_bytes_len{
            let digit:u16=match_byte_digit(input_value_bytes[index_counter as usize])as u16;

            fraction_part=(fraction_base/10)*(digit);
            primary_quotient=((fraction_base%10+1+2)*digit)as u8;// 2 is an initial stretch coefficient, equal to 0.0045% of the u16:MAX, covering the error caused by x.9999 being equal to u16:MAX-2.

            fraction_base/=10;
            index_counter+=1
        }
        while index_counter<input_value_bytes_len{
            let digit:u16=match_byte_digit(input_value_bytes[index_counter as usize])as u16;

            secondary_quotient=primary_quotient%10;
            primary_quotient/=10;

            fraction_part+=(fraction_base/10)*(digit)+primary_quotient as u16;
            primary_quotient=((fraction_base%10)*digit)as u8+secondary_quotient;
            
            if fraction_base==0{break}
            fraction_base/=10;

            index_counter+=1
        }
        fraction_part+=(primary_quotient/10)as u16;

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
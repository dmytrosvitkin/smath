use crate::{Fraction, structs::Value, traits::{Integer,UnsignedInteger}};

#[inline(always)]
const fn pow10(exponent:usize)->u32{
    let mut exponent:usize=exponent;
    let mut result:u32=1;
    while exponent>0{result*=10;exponent-=1}
    result
}

#[inline(always)]
pub const fn is_whole(value:f32)->bool{
    value==value as u32 as f32
}

/// Euclidean floor function.
///
/// Returns the greatest whole number equal or less than the given number.
#[inline(always)]
pub const fn floor(value:f32)->f32{
    let value_trunc:f32=trunc(value);
    if value<value_trunc{value_trunc-1.0}else{value_trunc}
}

/// Euclidean floor function, that leaves digits after the decimal point.
///
/// Returns the greatest whole number equal or less than the given number and decimal digits.
#[inline(always)]
pub const fn floor_digits(value:f32,digits:usize)->f32{
    let pow_10:f32=pow10(digits)as f32;
    floor(value*pow_10)/pow_10
}

/// Euclidean celling function.
///
/// Returns the least whole number equal or less than the given number.
#[inline(always)]
pub const fn celling(value:f32)->f32{
    let rcl:f32=value as i32 as f32;
    if value>rcl{rcl+1.0}else{rcl}
}

/// Euclidean celling function, that leaves digits after the decimal point.
///
/// Returns the least whole number equal or less than the given number and decimal digits.
#[inline(always)]
pub const fn celling_digits(value:f32,digits:usize)->f32{
    let pow_10:f32=pow10(digits)as f32;
    celling(value*pow_10)/pow_10
}

///Trunc function.
///
/// Returns the whole part of the given number.
#[inline(always)]
pub const fn trunc(value:f32)->f32{
    value as i32 as f32
}

/// Trunc function, that leaves digits after the decimal point.
/// 
/// Returns the whole part and decimal digits.
#[inline(always)]
pub const fn trunc_digits(value:f32,digits:usize)->f32{
    let pow_10:f32=pow10(digits)as f32;
    trunc(value*pow_10)/pow_10
}

/// Round function.
/// 
/// Returns the *closest* *whole* number.
#[inline(always)]
pub const fn round(value:f32)->f32{
  if value>0.0{floor(value+0.5)}
  else{celling(value-0.5)}
}

/// Round function that that leaves digits after the decimal point.
/// 
/// Returns the whole part of a rounded number and decimal digits.
#[inline(always)]
pub const fn round_digits(value:f32,digits:usize)->f32{
    let pow_10:f32=pow10(digits)as f32;
    round(value*pow_10)/pow_10
}

/// Simplified square root.
#[inline]
pub const fn sqrt(value:f32)->f32{
    if value<0.0{return f32::NAN}
    if value==0.0||value==f32::INFINITY{return value}
    let rcl:u32=value.to_bits();let rcl:f32=f32::from_bits(0x5f375a86-((rcl)>>1));let rcl:f32=rcl*(1.5-(0.5*value*rcl*rcl));let rcl:f32=value*rcl;0.5*(rcl+value/rcl)
}

/// Simplified inverse square root.
#[inline]
pub const fn isqrt(value:f32)->f32{
    if value<0.0{return f32::NAN}
    if value==0.0||value==f32::INFINITY{return 0.0}
    let rcl:u32=value.to_bits();let rcl:f32=f32::from_bits(0x5f375a86-((rcl)>>1));let rcl:f32=rcl*(1.5-(0.5*value*rcl*rcl));rcl
}

/// Absolute value.
/// 
/// Returns a positive number.
#[inline]
pub const fn abs(value:f32)->f32{
    if value<0.0{return-value}
    value
}

/// Factorial.
#[inline]
pub const fn factorial(value:u32)->u32{
    let mut result:u32=1;
    let mut value:u32=value;
    while value>1{result*=value;value-=1}
    result
}

/// Checks if the given number is prime.
/// 
/// Returns `true` if prime.
/// 
/// Returns `false` if non-prime.
#[inline]
pub const fn is_prime(value:u32)->bool{
    if value<2{return false}
    if value==2||value==3{return true}
    if value%2==0||value%3==0{return false}

    let mut d:u32=5;
    
    while d*d<=value{// 6k+-1 rule applied.
        if value%d==0||value%(d+2)==0{return false}
        d+=6;
    }

    true
}

#[inline(always)]
const fn gcd_2(mut a:u32,mut b:u32)->u32{
    while b!=0{
        let rcl:u32=b;
        b=a%b;
        a=rcl;
    }

    if a==0{return 1}
    a
}

/// Greatest common divider.
#[inline]
pub const fn gcd(value:&[u32])->u32{
    let value_len:usize=value.len();

    if value_len==0{return 1}
    else if value_len==1{if value[0]==0{return 1}return value[0]}
    else if value_len==2{return gcd_2(value[0],value[1])}
    
    let mut result:u32=0;
    let mut counter:usize=0;

    while counter<value_len{
        result=gcd_2(result,value[counter]);
        if result==1{return 1}
        counter+=1
    }

    result
}

#[inline(always)]
const fn lcm_2(a:u32,b:u32)->u32{
    if a==0||b==0{return 0}
    (a/gcd_2(a,b))*b
}


/// Least common Multiple.
#[inline]
pub const fn lcm(value:&[u32])->u32{
    if value.is_empty(){return 0}

    let mut result:u32=value[0];
    let mut counter:usize=1;

    while counter<value.len() {
        if result==0{return 0}
        result=lcm_2(result,value[counter]);
        counter+=1;
    }

    result
}

/// Arithmetic mean.
#[inline]
pub const fn arithmetic_mean(value:&[f32])->f32{
    let value_len:usize=value.len();
    if value_len==0{return 0.0}
    let mut result:f32=0.0;
    let mut counter:usize=0;
    while counter<value_len{counter+=1;result+=(value[counter-1]-result)/(counter as f32)}
    return result
}

/// Arithmetic average.
/// 
/// <sup>It is an alias for arithmetic mean.</sup>
#[inline]
pub const fn arithmetic_average(value:&[f32])->f32{// Alias for arithmetic_mean
    arithmetic_mean(value)
}

impl<W:Integer,F:UnsignedInteger>Value<W,F>{
    /// Euclidean floor function.
    ///
    /// Returns the greatest whole number equal or less than the given number.
    pub const fn floor(&mut self)->&mut Self{
        self.change_fraction_part(Fraction::zero())
    }
}

impl Value<i16,u16>{
    /// Whole number check.
    /// 
    /// Returns `true` if the value is *whole*.
    pub const fn is_whole(&self)->bool{
        self.raw_fraction_part()==0
    }

    /// Checks if the given number is prime.
    /// 
    /// Returns `true` if prime.
    /// 
    /// Returns `false` if non-prime.
    #[inline]
    pub const fn is_prime(&self)->bool{
        if!self.is_whole(){return false}

        let raw_whole_part:i16=self.raw_whole_part();
        if raw_whole_part<2||raw_whole_part%2==0||raw_whole_part%3==0{return false}
        else if raw_whole_part==2||raw_whole_part==3{return true}

        let mut counter:i16=5;
        
        while counter*counter<=raw_whole_part{// 6k+-1 rule.
            if raw_whole_part%counter==0||raw_whole_part%(counter+2)==0{return false}
            counter+=6;
        }

        true
    }
}
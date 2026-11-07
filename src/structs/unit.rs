use crate::utils::{is_valid_f32,is_valid_f64};

use super::value::Value;

/// Unit is a decimal number in range of `[-1; 1]` or `[0; 1]` depending on the type given: 
/// - `[0; 1]` for unsigned numerical types: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`;
/// - `[-1; 1]` for integer and float numerical types: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`, `f32`, `f64`.
pub struct Unit<T:Copy=i64>{
    unit:Value<T>
}

impl<T:Copy>Unit<T>{
    pub const fn none()->Self{
        Self{unit:Value::none()}
    }

    pub const fn is_none(&self)->bool{
        self.scaled_unit().is_none()
    }

    pub const fn from_scaled(raw_unit:Option<T>)->Self{
        Self{unit:Value::from(raw_unit)}
    }

    pub const fn from_scaled_unwrapped(raw_unwrapped_unit:T)->Self{
        Self{unit:Value::from_unwrapped(raw_unwrapped_unit)}
    }

    pub const fn scaled_unit(&self)->Option<T>{
        self.unit.value
    }

    pub const fn scaled_unit_unwrapped(&self)->T{
        self.scaled_unit().unwrap()
    }

    pub const fn scaled_unit_unwrapped_or(&self,default:T)->T{
        if self.is_none(){return default}
        self.scaled_unit_unwrapped()
    }

    pub fn scaled_unit_unwrapped_or_else<F:FnOnce()->T>(&self,or_else:F)->T{
        self.scaled_unit().unwrap_or_else(or_else)
    }

    pub const fn to_scaled_value(&self)->Value<T>{
        Value::from(self.scaled_unit())
    }
}

macro_rules!impl_unit_unsigned{
    ($($T:ty),+$(,)?)=>{
        $(
            impl Unit<$T>{
                pub const fn from_f32_unit_unchecked(unit:f32)->Self{
                    Self::from_scaled_unwrapped((unit*<$T>::MAX as f32)as $T)
                }

                pub const fn from_f32_unit(unit:f32)->Self{
                    if!is_valid_f32(unit)||unit<0.0||unit>1.0{return Self::none()}
                    Self::from_f32_unit_unchecked(unit)
                }

                pub const fn from_f32_unit_soft(unit:f32)->Self{
                    if unit.is_nan(){return Self::none()}
                    Self::from_f32_unit_unchecked(unit.clamp(0.0,1.0))
                }

                pub const fn as_f32_unchecked(&self)->f32{
                    self.scaled_unit_unwrapped()as f32/<$T>::MAX as f32
                }

                pub const fn as_f32(&self)->f32{
                    if self.is_none(){return f32::NAN}
                    self.as_f32_unchecked()
                }

                pub const fn as_option_f32(&self)->Option<f32>{
                    if self.is_none(){return None}
                    Some(self.as_f32_unchecked())
                }

                pub const fn from_f64_unit_unchecked(unit:f64)->Self{
                    Self::from_scaled_unwrapped((unit*<$T>::MAX as f64)as $T)
                }

                pub const fn from_f64_unit(unit:f64)->Self{
                    if!is_valid_f64(unit)||unit<0.0||unit>1.0{return Self::none()}
                    Self::from_f64_unit_unchecked(unit)
                }

                pub const fn from_f64_unit_soft(unit:f64)->Self{
                    if unit.is_nan(){return Self::none()}
                    Self::from_f64_unit_unchecked(unit.clamp(0.0,1.0))
                }

                pub const fn as_f64_unchecked(&self)->f64{
                    self.scaled_unit_unwrapped()as f64/<$T>::MAX as f64
                }

                pub const fn as_f64(&self)->f64{
                    if self.is_none(){return f64::NAN}
                    self.as_f64_unchecked()
                }

                pub const fn as_option_f64(&self)->Option<f64>{
                    if self.is_none(){return None}
                    Some(self.as_f64_unchecked())
                }
            }
        )+
    };
}

impl_unit_unsigned!(u8,u16,u32,u64,usize);// `u128` excluded, and will have its own logic

impl Unit<u128>{
    pub const fn from_f32_unit_unchecked(unit:f32)->Self{
        Self::from_f64_unit_unchecked(unit as f64)
    }

    pub const fn from_f32_unit(unit:f32)->Self{
        if!is_valid_f32(unit)||unit<0.0||unit>1.0{return Self::none()}
        Self::from_f32_unit_unchecked(unit)
    }

    pub const fn from_f32_unit_soft(unit:f32)->Self{
        if unit.is_nan(){return Self::none()}
        Self::from_f32_unit_unchecked(unit.clamp(0.0,1.0))
    }

    pub const fn as_f32_unchecked(&self)->f32{
        self.as_f64_unchecked()as f32
    }

    pub const fn as_f32(&self)->f32{
        if self.is_none(){return f32::NAN}
        self.as_f32_unchecked()
    }

    pub const fn as_option_f32(&self)->Option<f32>{
        if self.is_none(){return None}
        Some(self.as_f32_unchecked())
    }

    pub const fn from_f64_unit_unchecked(unit:f64)->Self{
        Self::from_scaled_unwrapped((unit*u128::MAX as f64)as u128)
    }

    pub const fn from_f64_unit(unit:f64)->Self{
        if!is_valid_f64(unit)||unit<0.0||unit>1.0{return Self::none()}
        Self::from_f64_unit_unchecked(unit)
    }

    pub const fn from_f64_unit_soft(unit:f64)->Self{
        if unit.is_nan(){return Self::none()}
        Self::from_f64_unit_unchecked(unit.clamp(0.0,1.0))
    }

    pub const fn as_f64_unchecked(&self)->f64{
        self.scaled_unit_unwrapped()as f64/u128::MAX as f64
    }

    pub const fn as_f64(&self)->f64{
        if self.is_none(){return f64::NAN}
        self.as_f64_unchecked()
    }

    pub const fn as_option_f64(&self)->Option<f64>{
        if self.is_none(){return None}
        Some(self.as_f64_unchecked())
    }
}
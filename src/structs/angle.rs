use crate::utils::{is_valid_f32, rem_euclid_f32};
use super::value::Value;

pub struct Angle<T:Copy>{
    pub(crate)angle:Value<T>
}

impl<T:Copy>Angle<T>{
    pub const fn none()->Self{
        Self{angle:Value::none()}
    }

    pub const fn is_none(&self)->bool{
        self.scaled_angle().is_none()
    }

    pub const fn from_scaled(scaled_angle:Option<T>)->Self{
        Self{angle:Value::from(scaled_angle)}
    }

    pub const fn from_scaled_unwrapped(unwrapped_scaled_angle:T)->Self{
        Self{angle:Value::from_unwrapped(unwrapped_scaled_angle)}
    }

    pub const fn scaled_angle(&self)->Option<T>{
        self.angle.value
    }

    pub const fn scaled_angle_unwrapped(&self)->T{
        self.scaled_angle().unwrap()
    }

    pub const fn scaled_angle_unwrapped_or(&self,default:T)->T{
        if self.is_none(){return default}
        self.scaled_angle_unwrapped()
    }

    pub fn scaled_angle_unwrapped_or_else<F:FnOnce()->T>(&self,or_else:F)->T{
        self.scaled_angle().unwrap_or_else(or_else)
    }

    pub const fn to_scaled_value(&self)->Value<T>{
        Value::from(self.scaled_angle())
    }
}

impl Angle<u16>{
    pub const fn from_degrees(degrees:u16)->Self{
        Angle::from_scaled_unwrapped(((degrees.rem_euclid(360) as f32/360.0)*u16::MAX as f32)as u16)
    }

    pub const fn from_f32_degrees(degrees:f32)->Self{
        if!is_valid_f32(degrees){return Angle::none()}
        Angle::from_scaled_unwrapped((rem_euclid_f32(degrees,360.0)*(u16::MAX as f32/360.0))as u16)
    }
}
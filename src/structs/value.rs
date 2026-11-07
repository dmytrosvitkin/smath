use super::angle::Angle;
use super::unit::Unit;

pub struct Value<T:Copy=i64>{
    pub(crate)value:Option<T>
}

impl<T:Copy>Value<T>{
    pub const fn none()->Self{
        Self{value:None}
    }

    pub const fn is_none(&self)->bool{
        self.value().is_none()
    }

    pub const fn from(value:Option<T>)->Self{
        Self{value}
    }

    pub const fn from_unwrapped(unwrapped_value:T)->Self{
        Self{value:Some(unwrapped_value)}
    }

    pub const fn value(&self)->Option<T>{
        self.value
    }

    pub const fn value_unwrapped(&self)->T{
        self.value().unwrap()
    }

    pub const fn value_unwrapped_or(&self,default:T)->T{
        if self.is_none(){return default}
        self.value_unwrapped()
    }

    pub fn value_unwrapped_or_else<F:FnOnce()->T>(&self,or_else:F)->T{
        self.value().unwrap_or_else(or_else)
    }

    pub const fn to_scaled_angle(&self)->Angle<T>{
        Angle::from_scaled(self.value())
    }

    pub const fn to_scaled_unit(&self)->Unit<T>{
        Unit::from_scaled(self.value())
    }
}
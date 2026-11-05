use super::value::Value;

pub struct Unit<T:Copy>{
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
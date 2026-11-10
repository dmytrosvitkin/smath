use super::whole::Whole;
use super::fraction::Fraction;

#[derive(Clone,Copy)]
pub(crate)struct UnwrappedValue<W:Copy,F:Copy>{
    whole_part:Whole<W>,
    fraction_part:Fraction<F>
}

impl<W:Copy,F:Copy>UnwrappedValue<W,F>{
    pub(crate)const fn from(whole_part:Whole<W>,fraction_part:Fraction<F>)->Self{
        Self{
            whole_part,
            fraction_part
        }
    }

    pub(crate)const fn from_raw(raw_whole_part:W,raw_fraction_part:F)->Self{
        Self::from(
            Whole::from_raw(raw_whole_part),
            Fraction::from_raw(raw_fraction_part),
        )
    }

    pub(crate)const fn as_whole_part(&self)->Whole<W>{
        self.whole_part
    }

    pub(crate)const fn as_fraction_part(&self)->Fraction<F>{
        self.fraction_part
    }

    pub(crate)const fn as_raw_whole_part(&self)->W{
        self.as_whole_part().as_raw()
    }

    pub(crate)const fn as_raw_fraction_part(&self)->F{
        self.as_fraction_part().as_raw()
    }

    pub(crate)const fn as_tuple(&self)->(Whole<W>,Fraction<F>){
        (
            self.as_whole_part(),
            self.as_fraction_part()
        )
    }

    pub(crate)const fn as_raw_tuple(&self)->(W,F){
        (
            self.as_raw_whole_part(),
            self.as_raw_fraction_part()
        )
    }

    pub(crate)const fn change_whole_part(&mut self,new_whole_part:Whole<W>)->&mut Self{
        self.whole_part=new_whole_part;
        self
    }

    pub(crate)const fn change_fraction_part(&mut self,new_fraction_part:Fraction<F>)->&mut Self{
        self.fraction_part=new_fraction_part;
        self
    }

    pub(crate)const fn change_raw_whole_part(&mut self,new_raw_whole_part:W)->&mut Self{
        self.change_whole_part(Whole::from_raw(new_raw_whole_part))
    }

    pub(crate)const fn change_raw_fraction_part(&mut self,new_raw_fraction_part:F)->&mut Self{
        self.change_fraction_part(Fraction::from_raw(new_raw_fraction_part))
    }
}

#[derive(Clone,Copy)]
pub struct Value<W:Copy,F:Copy>{
    value:Option<UnwrappedValue<W,F>>
}

impl<W:Copy,F:Copy>Value<W,F>{
    pub const fn is_none(&self)->bool{
        self.value.is_none()
    }

    pub const fn none()->Self{
        Self{value:None}
    }

    pub(crate)const fn from_value(value:Option<UnwrappedValue<W,F>>)->Self{
        Self{value}
    }

    pub(crate)const fn from_unwrapped_value(unwrapped_value:UnwrappedValue<W,F>)->Self{
        Self::from_value(Some(unwrapped_value))
    }

    pub const fn from_parts(whole_part:Whole<W>,fraction_part:Fraction<F>)->Self{
        Self{value:Some(UnwrappedValue::from(whole_part,fraction_part))}
    }

    pub const fn from_raw_parts(raw_whole_part:W,raw_fraction_part:F)->Self{
        Self{value:Some(UnwrappedValue::from_raw(raw_whole_part,raw_fraction_part))}
    }

    pub const fn from_unwrapped_tuple(unwrapped_tuple:(Whole<W>,Fraction<F>))->Self{
        Self::from_parts(
            unwrapped_tuple.0,
            unwrapped_tuple.1
        )
    }

    pub const fn from_unwrapped_raw_tuple(unwrapped_raw_tuple:(W,F))->Self{
        Self::from_raw_parts(
            unwrapped_raw_tuple.0,
            unwrapped_raw_tuple.1
        )
    }

    pub const fn from_wrapped_tuple(wrapped_tuple:Option<(Whole<W>,Fraction<F>)>)->Self{
        match wrapped_tuple{
            Some(unwrapped_tuple)=>Self::from_unwrapped_tuple(unwrapped_tuple),
            None=>Self::none()
        }
    }

    pub const fn from_wrapped_raw_tuple(wrapped_raw_tuple:Option<(W,F)>)->Self{
        match wrapped_raw_tuple{
            Some(unwrapped_raw_tuple)=>Self::from_unwrapped_raw_tuple(unwrapped_raw_tuple),
            None=>Self::none()
        }
    }

    pub const fn as_wrapped_whole_part(&self)->Option<Whole<W>>{
        match self.value{
            Some(unwrapped_value)=>Some(unwrapped_value.whole_part),
            None=>None
        }
    }

    pub const fn as_wrapped_fraction_part(&self)->Option<Fraction<F>>{
        match self.value{
            Some(unwrapped_value)=>Some(unwrapped_value.fraction_part),
            None=>None
        }
    }

    pub const fn as_unwrapped_whole_part(&self)->Whole<W>{
        self.value.unwrap().whole_part
    }

    pub const fn as_unwrapped_fraction_part(&self)->Whole<W>{
        self.value.unwrap().whole_part
    }
}
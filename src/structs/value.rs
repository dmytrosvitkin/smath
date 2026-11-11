use crate::traits::{Integer, UnsignedInteger};

use super::whole::Whole;
use super::fraction::Fraction;

#[derive(Clone,Copy)]
pub(crate)struct UnwrappedValue<W:Integer=i32,F:UnsignedInteger=u32>{
    whole_part:Whole<W>,
    fraction_part:Fraction<F>
}

#[allow(unused)]
impl<W:Integer,F:UnsignedInteger>UnwrappedValue<W,F>{
    pub(crate)const fn from(whole_part:Whole<W>,fraction_part:Fraction<F>)->Self{
        Self{
            whole_part,
            fraction_part
        }
    }

    pub(crate)const fn whole_part(&self)->Whole<W>{
        self.whole_part
    }

    pub(crate)const fn fraction_part(&self)->Fraction<F>{
        self.fraction_part
    }

    pub(crate)const fn as_tuple(&self)->(Whole<W>,Fraction<F>){
        (
            self.whole_part(),
            self.fraction_part()
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
}

#[derive(Clone,Copy)]
pub struct Value<W:Integer=i32,F:UnsignedInteger=u32>{
    value:Option<UnwrappedValue<W,F>>
}

impl<W:Integer,F:UnsignedInteger>Value<W,F>{
    #[allow(unused)]
    pub(crate)const fn from_option(value:Option<UnwrappedValue<W,F>>)->Self{
        Self{value}
    }

    #[allow(unused)]
    pub(crate)const fn from_unwrapped(unwrapped_value:UnwrappedValue<W,F>)->Self{
        Self::from_option(Some(unwrapped_value))
    }

    #[allow(unused)]
    pub(crate)const fn value(&self)->Option<UnwrappedValue<W,F>>{
        self.value
    }

    #[allow(unused)]
    pub(crate)const fn value_unwrapped(&self)->UnwrappedValue<W,F>{
        self.value().unwrap()
    }

    pub const fn is_none(&self)->bool{
        self.value().is_none()
    }

    pub const fn none()->Self{
        Self::from_option(None)
    }

    pub const fn from_unwrapped_parts(unwrapped_whole_part:Whole<W>,unwrapped_fraction_part:Fraction<F>)->Self{
        Self::from_unwrapped(UnwrappedValue::from(
            unwrapped_whole_part,
            unwrapped_fraction_part
        ))
    }

    pub const fn from_parts(option_whole_part:Option<Whole<W>>,option_fraction_part:Option<Fraction<F>>)->Self{
        let Some(unwrapped_whole_part)=option_whole_part else{return Self::none()};
        let Some(unwrapped_fraction_part)=option_fraction_part else{return Self::none()};
        Self::from_unwrapped_parts(
            unwrapped_whole_part,
            unwrapped_fraction_part
        )
    }

    pub const fn from_unwrapped_tuple(unwrapped_tuple:(Whole<W>,Fraction<F>))->Self{
        Self::from_unwrapped_parts(
            unwrapped_tuple.0,
            unwrapped_tuple.1
        )
    }

    pub const fn from_option_tuple(option_tuple:Option<(Whole<W>,Fraction<F>)>)->Self{
        match option_tuple{
            Some(unwrapped_tuple)=>Self::from_unwrapped_tuple(unwrapped_tuple),
            None=>Self::none()
        }
    }

    pub const fn whole_part(&self)->Option<Whole<W>>{
        match self.value(){
            Some(unwrapped_value)=>Some(unwrapped_value.whole_part()),
            None=>None
        }
    }

    pub const fn fraction_part(&self)->Option<Fraction<F>>{
        match self.value(){
            Some(unwrapped_value)=>Some(unwrapped_value.fraction_part()),
            None=>None
        }
    }

    pub const fn whole_part_unwrapped(&self)->Whole<W>{
        self.value_unwrapped().whole_part()
    }

    pub const fn fraction_part_unwrapped(&self)->Fraction<F>{
        self.value_unwrapped().fraction_part()
    }

    pub const fn as_tuple_option(&self)->Option<(Whole<W>,Fraction<F>)>{
        match self.value(){
            Some(unwrapped_value)=>Some(unwrapped_value.as_tuple()),
            None=>None
        }
    }

    pub const fn as_tuple_unwrapped(&self)->(Whole<W>,Fraction<F>){
        self.value_unwrapped().as_tuple()
    }

    pub const fn change_unwrapped_whole_part(&mut self,new_unwrapped_whole_part:Whole<W>)->&mut Self{
        let Some(unwrapped_value)=&mut self.value else{return self};
        unwrapped_value.change_whole_part(new_unwrapped_whole_part);
        self
    }

    pub const fn change_unwrapped_fraction_part(&mut self,new_unwrapped_fraction_part:Fraction<F>)->&mut Self{
        let Some(unwrapped_value)=&mut self.value else{return self};
        unwrapped_value.change_fraction_part(new_unwrapped_fraction_part);
        self
    }

    pub const fn change_whole_part(&mut self,new_whole_part:Option<Whole<W>>)->&mut Self{
        let Some(new_unwrapped_whole_part)=new_whole_part else{self.value=None;return self};
        self.change_unwrapped_whole_part(new_unwrapped_whole_part)
    }

    pub const fn change_fraction_part(&mut self,new_fraction_part:Option<Fraction<F>>)->&mut Self{
        let Some(new_unwrapped_fraction_part)=new_fraction_part else{self.value=None;return self};
        self.change_unwrapped_fraction_part(new_unwrapped_fraction_part)
    }

    pub const fn zero()->Self{
        Self::from_unwrapped_parts(Whole::zero(),Fraction::zero())
    }
    
    pub const fn min()->Self{
        Self::from_unwrapped_parts(Whole::min(),Fraction::min())
    }
    
    pub const fn max()->Self{
        Self::from_unwrapped_parts(Whole::max(),Fraction::max())
    }
}

impl<W:Integer,F:UnsignedInteger>Value<W,F>{
    pub const fn from_whole_number(whole_number:W)->Self{
        Self::from_unwrapped_parts(
            Whole::from_raw(whole_number),
            Fraction::zero()
        )
    }

    pub const fn as_whole_number_option(&self)->Option<W>{
        match self.whole_part(){
            Some(unwrapped_whole_part)=>Some(unwrapped_whole_part.as_raw()),
            None=>None
        }
    }

    pub const fn as_whole_number_unwrapped(&self)->W{
        self.whole_part_unwrapped().as_raw()
    }
}
use super::whole::Whole;
use super::fraction::Fraction;

#[derive(Clone,Copy)]
pub(crate)struct UnwrappedValue<W:Copy,F:Copy>{
    whole_part:Whole<W>,
    fraction_part:Fraction<F>
}

#[allow(unused)]
impl<W:Copy,F:Copy>UnwrappedValue<W,F>{
    pub(crate)const fn from(whole_part:Whole<W>,fraction_part:Fraction<F>)->Self{
        Self{
            whole_part,
            fraction_part
        }
    }

    pub(crate)const fn as_whole_part(&self)->Whole<W>{
        self.whole_part
    }

    pub(crate)const fn as_fraction_part(&self)->Fraction<F>{
        self.fraction_part
    }

    pub(crate)const fn as_tuple(&self)->(Whole<W>,Fraction<F>){
        (
            self.as_whole_part(),
            self.as_fraction_part()
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

    #[allow(unused)]
    pub(crate)const fn from_option(value:Option<UnwrappedValue<W,F>>)->Self{
        Self{value}
    }

    #[allow(unused)]
    pub(crate)const fn from_unwrapped(unwrapped_value:UnwrappedValue<W,F>)->Self{
        Self::from_option(Some(unwrapped_value))
    }

    pub const fn from_parts(whole_part:Whole<W>,fraction_part:Fraction<F>)->Self{
        Self{value:Some(UnwrappedValue::from(whole_part,fraction_part))}
    }

    pub const fn from_tuple(unwrapped_tuple:(Whole<W>,Fraction<F>))->Self{
        Self::from_parts(
            unwrapped_tuple.0,
            unwrapped_tuple.1
        )
    }

    pub const fn from_option_tuple(wrapped_tuple:Option<(Whole<W>,Fraction<F>)>)->Self{
        match wrapped_tuple{
            Some(unwrapped_tuple)=>Self::from_tuple(unwrapped_tuple),
            None=>Self::none()
        }
    }

    pub const fn option_whole_part(&self)->Option<Whole<W>>{
        match self.value{
            Some(unwrapped_value)=>Some(unwrapped_value.whole_part),
            None=>None
        }
    }

    pub const fn option_fraction_part(&self)->Option<Fraction<F>>{
        match self.value{
            Some(unwrapped_value)=>Some(unwrapped_value.fraction_part),
            None=>None
        }
    }

    pub const fn unwrap_whole_part(&self)->Whole<W>{
        self.value.unwrap().whole_part
    }

    pub const fn unwrap_fraction_part(&self)->Whole<W>{
        self.value.unwrap().whole_part
    }
}
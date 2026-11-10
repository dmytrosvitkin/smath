use super::whole::Whole;
use super::fraction::Fraction;

pub(crate)struct UnwrappedValue<W:Copy,F:Copy>{
    whole_part:Whole<W>,
    fraction_part:Fraction<F>
}

impl<W:Copy,F:Copy>UnwrappedValue<W,F>{
    pub(crate)const fn from_raw(raw_whole_part:W,raw_fraction_part:F)->Self{
        Self{
            whole_part:Whole::from_raw(raw_whole_part),
            fraction_part:Fraction::from_raw(raw_fraction_part)
        }
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

pub struct Value<W:Copy,F:Copy>{
    value:Option<UnwrappedValue<W,F>>
}
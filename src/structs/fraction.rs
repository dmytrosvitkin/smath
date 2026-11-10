/// Number in range `[0;1[` expressed by a value of *unsigned* type.
#[derive(Clone,Copy)]
pub struct Fraction<T:Copy>{
    fraction:T
}

impl<T:Copy>Fraction<T>{
    pub const fn from_raw(raw_fraction:T)->Fraction<T>{
        Fraction{fraction:raw_fraction}
    }

    pub const fn as_raw(&self)->T{
        self.fraction
    }
}
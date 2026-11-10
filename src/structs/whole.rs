/// Whole number.
#[derive(Clone,Copy)]
pub struct Whole<T:Copy>{
    whole:T
}

impl<T:Copy>Whole<T>{
    pub const fn from_raw(raw_whole:T)->Whole<T>{
        Whole{whole:raw_whole}
    }

    pub const fn as_raw(&self)->T{
        self.whole
    }
}
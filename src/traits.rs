pub trait Integer:Copy+PartialEq+Eq+'static{
    const ZERO:Self;
    const MIN:Self;
    const MAX:Self;
}

pub trait SignedInteger:Integer{}

pub trait UnsignedInteger:Integer{}

macro_rules!impl_integer_traits{
    (signed:$($s:ty),*;unsigned:$($u:ty),*)=>{
        $(
            impl Integer for $s{
                const ZERO:Self=0;
                const MIN:Self=<$s>::MIN;
                const MAX:Self=<$s>::MAX;
            }
            impl SignedInteger for $s{}
        )*
        $(
            impl Integer for $u{
                const ZERO:Self=0;
                const MIN:Self=<$u>::MIN;
                const MAX:Self=<$u>::MAX;
            }
            impl UnsignedInteger for $u{}
        )*
    };
}

impl_integer_traits!(
    signed:i8,i16,i32,i64,i128,isize;
    unsigned:u8,u16,u32,u64,u128,usize
);
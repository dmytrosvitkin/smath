#![allow(unused)]

macro_rules!float_utils{
    ($t:ident,$rem_euclid:ident,$is_valid:ident)=>{
        pub const fn $rem_euclid(value:$t,rhs:$t)->$t{
            let rcl:$t=value%rhs;
            if rcl<0.0{rcl+rhs.abs()}
            else{rcl}
        }

        pub(crate)const fn $is_valid(value:$t)->bool{
            !value.is_nan()&&value.is_finite()//&&value.is_normal()// Subnormals allowed.
        }
    };
}

float_utils!(f32,rem_euclid_f32,is_valid_f32);
float_utils!(f64,rem_euclid_f64,is_valid_f64);
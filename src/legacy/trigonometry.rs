use super::{floor,round,sqrt};
use super::constant::{COS,SIN,TAN,COTAN};

/// Simplified sinus function (degrees, 45.0 = 45°).
#[inline]
pub const fn sin(x:f32)->f32{
    let x:f32=x-floor(x*0.0027777778)*360.0;
    if x<180.0{let rcl:f32=x*0.011111111-1.0;let rcl:f32=rcl*rcl;(1.0-rcl)*(1.0-0.2240081*rcl)}
    else{let rcl:f32=x*0.011111111-3.0;let rcl:f32=rcl*rcl;(rcl-1.0)*(1.0-0.2240081*rcl)}
}

/// Simplified sinus function (degrees, 45 = 45°).
#[inline]
pub const fn sin_int(x:isize)->f32{
            let x:isize=x.rem_euclid(360);let rcl:isize=x/90;let x:isize=x%90;
            match rcl{
                0=>SIN[x as usize],
                1=>COS[x as usize],
                2=>-SIN[x as usize],
                _=>-COS[x as usize]
            }
}

/// Simplified sinus function (radians, 1.0 = π).
#[inline]
pub const fn sinr(x:f32)->f32{
    let x:f32=x-floor(x*0.5)*2.0;
    if x<1.0{let rcl:f32=x*2.0-1.0;let rcl:f32=rcl*rcl;(1.0-rcl)*(1.0-0.2240081*rcl)}
    else{let rcl:f32=x*2.0-3.0;let rcl:f32=rcl*rcl;(rcl-1.0)*(1.0-0.2240081*rcl)}
}

/// Simplified cosinus function (degrees, 45.0 = 45°).
#[inline]
pub const fn cos(x:f32)->f32{
    sin(x+90.0)
}

/// Simplified cosinus function (degrees, 45 = 45°).
#[inline]
pub const fn cos_int(x:isize)->f32{
    let x:isize=x.rem_euclid(360);let rcl:isize=x/90;let x:isize=x%90;
    match rcl{
        0=>COS[x as usize],
        1=>-SIN[x as usize],
        2=>-COS[x as usize],
        _=>SIN[x as usize]
    }
}


/// Simplified cosinus function (radians, 1.0 = π).
#[inline]
pub const fn cosr(x:f32)->f32{
    sinr(x+0.5)
}


/// Simplified tangens function (degrees, 45.0 = 45°).
#[inline]
pub const fn tan(x:f32)->f32{
    let x:f32=x*0.0055555556;let x:f32=(x-round(x))*180.0;
    let rcl:f32=x*x;(x*(141.37167-0.0031465*rcl))/(8100.0-rcl)
}

/// Simplified tangens function (degrees, 45 = 45°).
#[inline]
pub const fn tan_int(x:isize)->f32{
    let x:isize=x.rem_euclid(360);let rcl:isize=x/90;let x:isize=x%90;
    match rcl{
        0=>TAN[x as usize],
        1=>-COTAN[x as usize],
        2=>TAN[x as usize],
        _=>-COTAN[x as usize]
    }
}

/// Simplified tangens function (radians, 1.0 = π).
#[inline]
pub const fn tanr(x:f32)->f32{
    let x:f32=x-round(x);
    let rcl:f32=x*x;(x*(0.78539816-0.5663706*rcl))/(0.25-rcl)
}

/// Simplified cotangens function (degrees, 45.0 = 45°).
#[inline]
pub const fn cotan(x:f32)->f32{
    -tan(x+90.0)
}

/// Simplified cotangens function (degrees, 45 = 45°).
#[inline]
pub const fn cotan_int(x:isize)->f32{
    let x:isize=x.rem_euclid(360);let rcl:isize=x/90;let x:isize=x%90;
    match rcl{
        0=>COTAN[x as usize],
        1=>-TAN[x as usize],
        2=>COTAN[x as usize],
        _=>-TAN[x as usize]
    }
}

/// Simplified cotangens function (radians, 1.0 = π).
#[inline]
pub const fn cotanr(x:f32)->f32{
    -tanr(x+0.5)
}

/// Simplified arc-sinus function (degrees, 45.0 = 45°).
#[inline]
pub const fn asin(x:f32)->f32{
    if x<0.0{sqrt(1.0-x)*(90.0+(12.15324+(4.25484+1.07311*x)*x)*x)-90.0}
    else{90.0-sqrt(1.0-x)*(90.0-(12.15324-(4.25484-1.07311*x)*x)*x)}
}

/// Simplified arc-sinus function (radians, 1.0 = π).
#[inline]
pub const fn asinr(x:f32)->f32{
    if x<0.0{sqrt(1.0-x)*(0.5+(0.0675181+(0.0236380+0.0059617*x)*x)*x)-0.5}
    else{0.5-sqrt(1.0-x)*(0.5-(0.0675181-(0.0236380-0.0059617*x)*x)*x)}
}

/// Simplified arc-cosinus function (degrees, 45.0 = 45°).
#[inline]
pub const fn acos(x:f32)->f32{
    90.0-asin(x)
}

/// Simplified arc-cosinus function (radians, 1.0 = π).
#[inline]
pub const fn acosr(x:f32)->f32{
    0.5-asinr(x)
}

/// Simplified arc-tangens function (degrees, 45.0 = 45°).
#[inline]
pub const fn atan(x:f32)->f32{
    if x>0.0&&x<2.0{let x:f32=1.3*x;let rcl:f32=x-2.0;(x/(x+1.0))*(90.0-(rcl*rcl*10.0))}
    else if x>0.0{90.0-90.0/(1.3*x+1.0)}
    else if x>-2.0{let x:f32=-1.3*x;let rcl:f32=2.0-x;-(x/(x+1.0))*(90.0-(rcl*rcl*10.0))}
    else{90.0/(-1.3*x+1.0)-90.0}
}

/// Simplified arc-tangens function (radians, 1.0 = π).
#[inline]
pub const fn atanr(x:f32)->f32{
    if x>0.0&&x<2.0{let x:f32=1.3*x;let rcl:f32=x-2.0;(x/(x+1.0))*(0.5-(rcl*rcl*0.055555556))}
    else if x>0.0{0.5-0.5/(1.3*x+1.0)}
    else if x>-2.0{let x:f32=-1.3*x;let rcl:f32=2.0-x;-(x/(x+1.0))*(0.5-(rcl*rcl*0.055555556))}
    else{1.0/(-2.6*x+2.0)-0.5}
}

/// Simplified arc-cotangens function (degrees, 45.0 = 45°).
#[inline]
pub const fn acotan(x:f32)->f32{
    90.0-atan(x)
}

/// Simplified arc-cotangens function (degrees, 45.0 = 45°).
#[inline]
pub const fn acotanr(x:f32)->f32{
    0.5-atanr(x)
}
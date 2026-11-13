pub(crate)const fn match_byte_digit(byte:u8)->i8{// `u8::MAX` or anything outside of 0-9 is considered an illegal symbol.
    match byte{
        b'0'=>0,
        b'1'=>1,
        b'2'=>2,
        b'3'=>3,
        b'4'=>4,
        b'5'=>5,
        b'6'=>6,
        b'7'=>7,
        b'8'=>8,
        b'9'=>9,
        _=>i8::MAX,
    }
}
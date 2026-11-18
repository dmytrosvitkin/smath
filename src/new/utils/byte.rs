pub(crate)const fn match_byte_digit(byte:u8)->u8{
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
        _=>panic!("Illegal byte!")
    }
}

pub(crate)const fn match_digit_byte(digit:u8)->u8{
    match digit{
        0=>b'0',
        1=>b'1',
        2=>b'2',
        3=>b'3',
        4=>b'4',
        5=>b'5',
        6=>b'6',
        7=>b'7',
        8=>b'8',
        9=>b'9',
        _=>panic!("Illegal digit!")
    }
}

pub(crate)const unsafe fn trim_bytes_unchecked<'a,const S:usize>(untrimmed_bytes:&'a([u8;S],usize))->&'a[u8]{
    unsafe{untrimmed_bytes.0.split_at_unchecked(S-untrimmed_bytes.1).1}
}

pub(crate)const fn trim_bytes<'a,const S:usize>(untrimmed_bytes:&'a([u8;S],usize))->&'a[u8]{
    untrimmed_bytes.0.split_at(S-untrimmed_bytes.1).1}

pub(crate)const fn bytes_as_str<'a,const S:usize>(untrimmed_bytes:&'a([u8;S],usize))->&'a str{
    unsafe{core::str::from_utf8_unchecked(trim_bytes(untrimmed_bytes))}
}
pub(crate)const fn match_type_bits_length(bits:u8)->u8{
    match bits{
        8=>3,
        16=>5,
        32=>10,
        64=>20,
        128=>39,
        _=>39
    }
}
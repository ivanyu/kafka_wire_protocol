use std::io::{Error, ErrorKind, Read, Result};
use byteorder::{BigEndian, ReadBytesExt};
use varint_rs::VarintReader;

pub(crate) fn read_string(input: &mut impl Read, field_name: &str) -> Result<String> {
    let str_len = read_len(input)?;
    read_string0(input, field_name, str_len)
}

pub(crate) fn read_compact_string(input: &mut impl Read, field_name: &str) -> Result<String> {
    let str_len = read_compact_len(input, field_name)?;
    read_string0(input, field_name, str_len)
}

fn read_string0(input: &mut impl Read, field_name: &str, str_len: i16) -> Result<String> {
    check_non_null(field_name, str_len)?;
    read_string_bytes(input, str_len)
}

pub(crate) fn read_nullable_string(input: &mut impl Read, #[allow(unused)] field_name: &str) -> Result<Option<String>> {
    let str_len = read_len(input)?;
    read_nullable_string0(input, str_len)
}

pub(crate) fn read_nullable_compact_string(input: &mut impl Read, field_name: &str) -> Result<Option<String>> {
    let str_len = read_compact_len(input, field_name)?;
    read_nullable_string0(input, str_len)
}

fn read_nullable_string0(input: &mut impl Read, str_len: i16) -> Result<Option<String>> {
    if str_len < 0 {
        Ok(None)
    } else {
        read_string_bytes(input, str_len).map(Some)
    }
}

fn read_len(input: &mut impl Read) -> Result<i16> {
    input.read_i16::<BigEndian>()
}

fn read_compact_len(input: &mut impl Read, field_name: &str) -> Result<i16> {
    let str_len = input.read_u32_varint()? - 1;
    if str_len > 0x7fff {
        // TODO replace with proper error
        Err(Error::new(ErrorKind::Other, format!("string field {} had invalid length ???", field_name)))
    } else {
        Ok(str_len as i16)
    }
}

fn check_non_null(field_name: &str, str_len: i16) -> Result<()> {
    if str_len < 0 {
        // TODO replace with proper error
        Err(Error::new(ErrorKind::Other, format!("non-nullable field {} was serialized as null", field_name)))
    } else {
        Ok(())
    }
}

fn read_string_bytes(input: &mut impl Read, str_len: i16) -> Result<String> {
    let mut str_buf = vec![0_u8; str_len as usize];
    input.read_exact(&mut str_buf)?;
    Ok(String::from_utf8_lossy(&str_buf).to_string())
}

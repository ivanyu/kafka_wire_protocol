use std::io::{Error, ErrorKind, Read, Result, Write};
use serde::{Deserialize, Serialize};
#[cfg(test)] use proptest_derive::Arbitrary;
use varint_rs::{VarintReader, VarintWriter};
use crate::readable_writable::{Readable, Writable};
#[cfg(test)] use crate::test_utils::proptest_strategies;
#[cfg(test)] use crate::test_utils::serde_bytes;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(test, derive(Arbitrary))]
pub struct RawTaggedField {
    pub tag: i32,
    #[cfg_attr(test, proptest(strategy = "proptest_strategies::vec()"))]
    #[cfg_attr(test, serde(with = "serde_bytes"))]
    pub data: Vec<u8>,
}

impl Readable for RawTaggedField {
    fn read(input: &mut impl Read) -> Result<Self> {
        let tag = input.read_u32_varint()? as i32;
        let data_len = input.read_u32_varint()? as i32;
        let mut data: Vec<u8> = vec![0; data_len as usize];
        input.read(&mut data)?;
        Ok(RawTaggedField { tag, data })
    }
}

impl Writable for RawTaggedField {
    fn write(&self, output: &mut impl Write) -> Result<()> {
        output.write_u32_varint(self.tag as u32)?;
        output.write_u32_varint(self.data.len() as u32)?;
        output.write(&self.data)?;
        Ok(())
    }
}

pub(crate) fn read_tagged_fields(input: &mut impl Read, mut callback: impl FnMut(i32, &[u8]) -> Result<bool>) -> Result<Vec<RawTaggedField>> {
    let arr_len = input.read_u32_varint()?;
    let mut unknown_tagged_fields: Vec<RawTaggedField> = Vec::new();
    for _ in 0..arr_len {
        let field = RawTaggedField::read(input)?;
        if !callback(field.tag, &field.data)? {
            unknown_tagged_fields.push(field);
        }
    }
    Ok(unknown_tagged_fields)
}

pub(crate) fn write_tagged_fields(output: &mut impl Write, known_tagged_fields: &[RawTaggedField], unknown_tagged_fields: &[RawTaggedField]) -> Result<()> {
    let mut max_known_tag = -1;
    for tag_pair in known_tagged_fields.windows(2) {
        let tag0 = &tag_pair[0].tag;
        let tag1 = &tag_pair[1].tag;
        if tag0 >= tag1 {
            return Err(Error::new(ErrorKind::Other, format!(
                "Invalid raw tag field list: tag {tag1:?} comes after tag {tag0:?}, but is not higher than it."
            )));
        }
        if *tag0 > max_known_tag {
            max_known_tag = *tag0;
        }
    }
    for tag_pair in unknown_tagged_fields.windows(2) {
        let tag0 = &tag_pair[0].tag;
        let tag1 = &tag_pair[1].tag;
        if tag0 >= tag1 {
            return Err(Error::new(ErrorKind::Other, format!(
                "Invalid raw tag field list: tag {tag1:?} comes after tag {tag0:?}, but is not higher than it."
            )));
        }
        if *tag0 <= max_known_tag {
            return Err(Error::new(ErrorKind::Other, format!(
                "Invalid raw tag field list: tag {tag0:?} comes after tag {max_known_tag:?}, but is not higher than it."
            )));
        }
    }

    output.write_u32_varint((known_tagged_fields.len() + unknown_tagged_fields.len()) as u32)?;
    for el in known_tagged_fields {
        el.write(output)?
    }
    for el in unknown_tagged_fields {
        el.write(output)?
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::{Cursor, Seek, SeekFrom};
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_serde(data: RawTaggedField) {
            crate::test_utils::test_serde(&data)?;
        }
    }

    #[test]
    fn test_serde_multiple_fields() {
        let original_fields = vec! {
            RawTaggedField { tag: 0, data: vec![0, 1, 2, 3] },
            RawTaggedField { tag: 1, data: vec![0, 1] },
            RawTaggedField { tag: 4, data: vec![0, 1, 2, 3, 4, 5] }
        };
        let unknown_fields: Vec<RawTaggedField> = vec![];

        let mut cur = Cursor::new(Vec::<u8>::new());

        write_tagged_fields(&mut cur, &original_fields, &unknown_fields).unwrap();

        cur.seek(SeekFrom::Start(0)).unwrap();

        let tagged_fields_callback = |_: i32, _: &[u8]| { Ok(false) };
        let read_fields = read_tagged_fields(&mut cur, &tagged_fields_callback).unwrap();

        assert_eq!(read_fields, original_fields);
    }

    #[test]
    fn test_serde_multiple_fields_wrong_order() {
        let original_fields = vec! {
            RawTaggedField { tag: 1, data: vec![0, 1] },
            RawTaggedField { tag: 0, data: vec![0, 1, 2, 3] },
            RawTaggedField { tag: 4, data: vec![0, 1, 2, 3, 4, 5] }
        };
        let unknown_fields: Vec<RawTaggedField> = vec![];

        let mut cur = Cursor::new(Vec::<u8>::new());
        let error = write_tagged_fields(&mut cur, &original_fields, &unknown_fields)
            .expect_err("must_be_error");
        assert_eq!(error.to_string(), "Invalid raw tag field list: tag 0 comes after tag 1, but is not higher than it.");
    }

    #[test]
    fn test_serde_multiple_fields_empty() {
        let original_fields: Vec<RawTaggedField> = vec![];
        let unknown_fields: Vec<RawTaggedField> = vec![];

        let mut cur = Cursor::new(Vec::<u8>::new());
        write_tagged_fields(&mut cur, &original_fields, &unknown_fields).unwrap();

        cur.seek(SeekFrom::Start(0)).unwrap();
        let tagged_fields_callback = |_: i32, _: &[u8]| { Ok(false) };
        let read_fields = read_tagged_fields(&mut cur, &tagged_fields_callback).unwrap();

        assert_eq!(read_fields, original_fields);
    }
}

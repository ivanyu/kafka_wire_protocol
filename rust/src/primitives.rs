use std::io::{Read, Result, Write};

use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use uuid::Uuid;

pub(crate) trait KafkaReadable: Sized {
    fn read(input: &mut impl Read) -> Result<Self>;
}

pub(crate) trait KafkaWritable: Sized {
    fn write(&self, output: &mut impl Write) -> Result<()>;
}

impl KafkaReadable for bool {
    fn read(input: &mut impl Read) -> Result<Self> {
        input.read_i8().map(|v| v != 0)
    }
}

impl KafkaWritable for bool {
    #[inline]
    fn write(&self, output: &mut impl Write) -> Result<()> {
        if *self {
            output.write_i8(1)
        } else {
            output.write_i8(0)
        }
    }
}

impl KafkaReadable for i8 {
    fn read(input: &mut impl Read) -> Result<Self> {
        input.read_i8()
    }
}

impl KafkaWritable for i8 {
    #[inline]
    fn write(&self, output: &mut impl Write) -> Result<()> {
        output.write_i8(*self)
    }
}

impl KafkaReadable for u8 {
    fn read(input: &mut impl Read) -> Result<Self> {
        input.read_u8()
    }
}

impl KafkaWritable for u8 {
    #[inline]
    fn write(&self, output: &mut impl Write) -> Result<()> {
        output.write_u8(*self)
    }
}

impl KafkaReadable for u16 {
    fn read(input: &mut impl Read) -> Result<Self> {
        input.read_u16::<BigEndian>()
    }
}

impl KafkaWritable for u16 {
    #[inline]
    fn write(&self, output: &mut impl Write) -> Result<()> {
        output.write_u16::<BigEndian>(*self)
    }
}

impl KafkaReadable for i16 {
    fn read(input: &mut impl Read) -> Result<Self> {
        input.read_i16::<BigEndian>()
    }
}

impl KafkaWritable for i16 {
    #[inline]
    fn write(&self, output: &mut impl Write) -> Result<()> {
        output.write_i16::<BigEndian>(*self)
    }
}

impl KafkaReadable for u32 {
    fn read(input: &mut impl Read) -> Result<Self> {
        input.read_u32::<BigEndian>()
    }
}

impl KafkaWritable for u32 {
    #[inline]
    fn write(&self, output: &mut impl Write) -> Result<()> {
        output.write_u32::<BigEndian>(*self)
    }
}

impl KafkaReadable for i32 {
    fn read(input: &mut impl Read) -> Result<Self> {
        input.read_i32::<BigEndian>()
    }
}

impl KafkaWritable for i32 {
    #[inline]
    fn write(&self, output: &mut impl Write) -> Result<()> {
        output.write_i32::<BigEndian>(*self)
    }
}

impl KafkaReadable for i64 {
    fn read(input: &mut impl Read) -> Result<Self> {
        input.read_i64::<BigEndian>()
    }
}

impl KafkaWritable for i64 {
    #[inline]
    fn write(&self, output: &mut impl Write) -> Result<()> {
        output.write_i64::<BigEndian>(*self)
    }
}

impl KafkaReadable for f64 {
    fn read(input: &mut impl Read) -> Result<Self> {
        input.read_f64::<BigEndian>()
    }
}

impl KafkaWritable for f64 {
    #[inline]
    fn write(&self, output: &mut impl Write) -> Result<()> {
        output.write_f64::<BigEndian>(*self)
    }
}

impl KafkaReadable for Uuid {
    fn read(input: &mut impl Read) -> Result<Self> {
        input.read_u128::<BigEndian>().map(Uuid::from_u128)
    }
}

impl KafkaWritable for Uuid {
    #[inline]
    fn write(&self, output: &mut impl Write) -> Result<()> {
        output.write_u128::<BigEndian>(self.as_u128())
    }
}

#[cfg(test)]
mod tests {
    use std::io::{Cursor, Seek, SeekFrom};

    use rstest::rstest;
    use uuid::uuid;

    use super::*;

    #[rstest]
    #[case(false)]
    #[case(true)]
    fn test_bool(#[case] original_value: bool) {
        let mut cur = Cursor::new(Vec::<u8>::new());
        original_value.write(&mut cur).unwrap();
        cur.seek(SeekFrom::Start(0)).unwrap();
        let read_value = bool::read(&mut cur).unwrap();
        assert_eq!(read_value, original_value);
    }

    #[rstest]
    #[case(0)]
    #[case(1)]
    fn test_u8(#[case] original_value: u8) {
        let mut cur = Cursor::new(Vec::<u8>::new());
        original_value.write(&mut cur).unwrap();
        cur.seek(SeekFrom::Start(0)).unwrap();
        let read_value = u8::read(&mut cur).unwrap();
        assert_eq!(read_value, original_value);
    }

    #[rstest]
    #[case(-1)]
    #[case(0)]
    #[case(1)]
    fn test_i8(#[case] original_value: i8) {
        let mut cur = Cursor::new(Vec::<u8>::new());
        original_value.write(&mut cur).unwrap();
        cur.seek(SeekFrom::Start(0)).unwrap();
        let read_value = i8::read(&mut cur).unwrap();
        assert_eq!(read_value, original_value);
    }

    #[rstest]
    #[case(0)]
    #[case(1)]
    fn test_u16(#[case] original_value: u16) {
        let mut cur = Cursor::new(Vec::<u8>::new());
        original_value.write(&mut cur).unwrap();
        cur.seek(SeekFrom::Start(0)).unwrap();
        let read_value = u16::read(&mut cur).unwrap();
        assert_eq!(read_value, original_value);
    }

    #[rstest]
    #[case(-1)]
    #[case(0)]
    #[case(1)]
    fn test_i16(#[case] original_value: i16) {
        let mut cur = Cursor::new(Vec::<u8>::new());
        original_value.write(&mut cur).unwrap();
        cur.seek(SeekFrom::Start(0)).unwrap();
        let read_value = i16::read(&mut cur).unwrap();
        assert_eq!(read_value, original_value);
    }

    #[rstest]
    #[case(0)]
    #[case(1)]
    fn test_u32(#[case] original_value: u32) {
        let mut cur = Cursor::new(Vec::<u8>::new());
        original_value.write(&mut cur).unwrap();
        cur.seek(SeekFrom::Start(0)).unwrap();
        let read_value = u32::read(&mut cur).unwrap();
        assert_eq!(read_value, original_value);
    }

    #[rstest]
    #[case(-1)]
    #[case(0)]
    #[case(1)]
    fn test_i32(#[case] original_value: i32) {
        let mut cur = Cursor::new(Vec::<u8>::new());
        original_value.write(&mut cur).unwrap();
        cur.seek(SeekFrom::Start(0)).unwrap();
        let read_value = i32::read(&mut cur).unwrap();
        assert_eq!(read_value, original_value);
    }

    #[rstest]
    #[case(-1)]
    #[case(0)]
    #[case(1)]
    fn test_i64(#[case] original_value: i64) {
        let mut cur = Cursor::new(Vec::<u8>::new());
        original_value.write(&mut cur).unwrap();
        cur.seek(SeekFrom::Start(0)).unwrap();
        let read_value = i64::read(&mut cur).unwrap();
        assert_eq!(read_value, original_value);
    }

    #[rstest]
    #[case(-1.0)]
    #[case(0.0)]
    #[case(1.0)]
    fn test_f64(#[case] original_value: f64) {
        let mut cur = Cursor::new(Vec::<u8>::new());
        original_value.write(&mut cur).unwrap();
        cur.seek(SeekFrom::Start(0)).unwrap();
        let read_value = f64::read(&mut cur).unwrap();
        assert_eq!(read_value, original_value);
    }

    #[rstest]
    #[case(uuid!("00000000-0000-0000-0000-000000000000"))]
    #[case(uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8"))]
    fn test_uuid(#[case] original_value: Uuid) {
        let mut cur = Cursor::new(Vec::<u8>::new());
        original_value.write(&mut cur).unwrap();
        cur.seek(SeekFrom::Start(0)).unwrap();
        let read_value = Uuid::read(&mut cur).unwrap();
        assert_eq!(read_value, original_value);
    }
}

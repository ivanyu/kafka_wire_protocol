use std::io::{Read, Write};
use std::io;

pub trait KafkaReadable: Sized {
    fn read(input: &mut impl Read) -> io::Result<Self>;

    fn read_ext(input: &mut impl Read,
                #[allow(unused)] field_name: &str,
                #[allow(unused)] compact: bool) -> io::Result<Self> {
        KafkaReadable::read(input)
    }
}

pub trait KafkaWritable {
    fn write(&self, output: &mut impl Write) -> io::Result<()>;

    fn write_ext(&self, output: &mut impl Write,
                 #[allow(unused)] field_name: &str,
                 #[allow(unused)] compact: bool) -> io::Result<()> {
        self.write(output)
    }
}

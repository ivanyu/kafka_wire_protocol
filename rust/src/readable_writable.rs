use std::io::{Read, Write};
use std::io;

/// An entity that can be read from [`Read`].
pub trait Readable: Sized {
    /// Read this entity from a [`Read`].
    fn read(input: &mut impl Read) -> io::Result<Self>;

    /// Read this entity from a [`Read`]
    /// specifying the field name and controlling compactness.
    fn read_ext(input: &mut impl Read,
                #[allow(unused)] field_name: &str,
                #[allow(unused)] compact: bool) -> io::Result<Self> {
        Readable::read(input)
    }
}

/// An entity that can be written to [`Write`].
pub trait Writable {
    /// Write this entity to a [`Write`].
    fn write(&self, output: &mut impl Write) -> io::Result<()>;

    /// Write this entity to a [`Write`]
    /// specifying the field name and controlling compactness.
    fn write_ext(&self, output: &mut impl Write,
                 #[allow(unused)] field_name: &str,
                 #[allow(unused)] compact: bool) -> io::Result<()> {
        self.write(output)
    }
}

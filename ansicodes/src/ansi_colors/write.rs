use core::fmt;

pub trait AnyWrite {
    type Wstr: ?Sized;
    type Error;

    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Self::Error>;

    fn write_str(&mut self, s: &Self::Wstr) -> Result<(), Self::Error>;
}

impl AnyWrite for dyn fmt::Write + '_ {
    type Wstr = str;
    type Error = fmt::Error;

    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Self::Error> {
        fmt::Write::write_fmt(self, fmt)
    }

    fn write_str(&mut self, s: &Self::Wstr) -> Result<(), Self::Error> {
        fmt::Write::write_str(self, s)
    }
}

#[cfg(feature = "std")]
impl AnyWrite for dyn std::io::Write + '_ {
    //я бля сперва не так прочитал и подумал что это какой то необычный аналог for<'a>
    type Wstr = [u8];
    type Error = std::io::Error;

    fn write_fmt(&mut self, fmt: fmt::Arguments) -> Result<(), Self::Error> {
        std::io::Write::write_fmt(self, fmt)
    }

    fn write_str(&mut self, s: &Self::Wstr) -> Result<(), Self::Error> {
        std::io::Write::write_all(self, s)
    }
}

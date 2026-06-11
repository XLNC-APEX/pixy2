/// Possible errors from Pixy2
#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    Timeout,
    SpiError,
    Busy,
    BufferWontFit,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::SpiError => write!(f, "SPI communication error"),
            _ => Ok(()), // wtf did I do with this line?
        }
    }
}

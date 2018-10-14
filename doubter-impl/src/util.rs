use std::error::Error as StdError;
use std::io;

pub fn io_error<E>(cause: E) -> io::Error
where
    E: Into<Box<StdError + Send + Sync + 'static>>,
{
    io::Error::new(io::ErrorKind::Other, cause)
}

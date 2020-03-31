use std::error::Error as StdError;
use std::fs;
use std::io;
use std::io::Read;
use std::path::Path;

pub fn io_error<E>(cause: E) -> io::Error
where
    E: Into<Box<dyn StdError + Send + Sync + 'static>>,
{
    io::Error::new(io::ErrorKind::Other, cause)
}

pub fn read_to_string<P>(path: P) -> io::Result<String>
where
    P: AsRef<Path>,
{
    let mut file = fs::File::open(path)?;

    let mut content =
        String::with_capacity({ file.metadata().map(|m| m.len() as usize + 1).unwrap_or(0) });
    file.read_to_string(&mut content)?;

    Ok(content)
}

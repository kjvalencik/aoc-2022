pub use std::{io, ops, str::FromStr};

pub use anyhow::{bail, Context, Error};

pub fn read_stdin() -> Result<String, io::Error> {
    let mut buf = String::new();
    io::Read::read_to_string(&mut io::stdin(), &mut buf)?;
    Ok(buf)
}

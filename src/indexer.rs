use std::io;

use crate::EResult;

pub fn index_to_identifier(index: usize) -> String { (index + 1).to_string() }

pub fn identifier_to_index(identifier: &str) -> EResult<usize>
{
    match str::parse::<usize>(identifier)
    {
        Ok(index) => Ok(index - 1),
        Err(_) => Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Could not parse an index.",
        ))),
    }
}

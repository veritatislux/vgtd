use crate::EResult;

pub fn index_to_identifier(index: usize) -> String { index.to_string() }

pub fn identifier_to_index(identifier: &str) -> EResult<usize>
{
    match str::parse(identifier)
    {
        Ok(index) => Ok(index),
        Err(error) => Err(Box::new(error)),
    }
}

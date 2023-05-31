use std::fs;

use crate::gtd;
use crate::EResult;

pub fn parse(path: &str) -> EResult<gtd::File>
{
    match fs::read_to_string(path)
    {
        Ok(contents) =>
        {
            match toml::from_str::<gtd::File>(&contents)
            {
                Ok(file) => Ok(file),
                Err(error) => Err(Box::new(error)),
            }
        }
        Err(error) => Err(Box::new(error)),
    }
}

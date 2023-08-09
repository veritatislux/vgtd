use std::io;

use crate::EResult;

pub const GTD_FILE_PATH: &str = ".gtd.toml";

pub fn get_workspace_file_path(global: bool) -> EResult<String>
{
    if !global
    {
        return Ok(GTD_FILE_PATH.to_owned());
    }

    if let Some(base_dir) = directories::BaseDirs::new()
    {
        if let Some(path_str) =
            base_dir.home_dir().join(GTD_FILE_PATH).to_str()
        {
            Ok(path_str.to_owned())
        }
        else
        {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Unable to parse global workspace path.",
            )));
        }
    }
    else
    {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::NotFound,
            "Could not locate global workspace file path.",
        )));
    }
}

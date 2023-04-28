use std::fs::File;
use std::io::Read;

use serde::Deserialize;

use crate::error::StatusResult;
use crate::gtd::List;
use crate::gtd::Task;


#[derive(Deserialize)]
pub struct GTDFileTask
{
    pub name: String,
    pub details: String,
    pub contexts: Vec<String>,
}


#[derive(Deserialize)]
pub struct GTDFileList
{
    pub name: String,
    pub tasks: Vec<GTDFileTask>,
}


#[derive(Deserialize)]
pub struct GTDFile
{
    pub lists: Vec<GTDFileList>,
}


pub fn read_file(path: &str) -> StatusResult<String>
{
    let mut file = match File::open(path)
    {
        Ok(file) => file,
        Err(_) => { return Err("couldn't open file"); }
    };

    let mut contents = String::new();

    match file.read_to_string(&mut contents)
    {
        Ok(_) => Ok(contents),
        Err(_) => Err("couldn't read file contents")
    }
}


pub fn parse(path: &str) -> StatusResult<GTDFile>
{
    match toml::from_str::<GTDFile>(&read_file(path)?)
    {
        Ok(value) => Ok(value),
        Err(_) => Err("couldn't parse file")
    }
}


pub fn to_gtd(file: GTDFile) -> Vec<List>
{
    let mut gtd_lists: Vec<List> = vec![];

    for list in file.lists.into_iter()
    {
        let mut new_list = List::new(list.name);

        for task in list.tasks.into_iter()
        {
            let mut new_task = Task::new(task.name);
            new_task.set_contexts(task.contexts);
            new_list.push_task(new_task);
        }

        gtd_lists.push(new_list);
    }

    gtd_lists
}

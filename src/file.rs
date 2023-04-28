use std::fs;

use serde::Deserialize;
use serde::Serialize;

use crate::error::StatusResult;
use crate::gtd::List;
use crate::gtd::Task;


#[derive(Serialize, Deserialize)]
pub struct GTDFileTask
{
    pub name: String,
    pub details: String,
    pub contexts: Vec<String>,
}


#[derive(Serialize, Deserialize)]
pub struct GTDFileList
{
    pub name: String,
    pub tasks: Vec<GTDFileTask>,
}


#[derive(Serialize, Deserialize)]
pub struct GTDFile
{
    pub lists: Vec<GTDFileList>,
}


pub fn read_file(path: &str) -> StatusResult<String>
{
    match fs::read_to_string(path)
    {
        Ok(contents) => Ok(contents),
        Err(_) => Err("couldn't read file contents")
    }
}


pub fn write_file(path: &str, contents: String) -> StatusResult<()>
{
    if let Err(_) = fs::write(path, contents)
    {
        return Err("couldn't write file contents")
    }

    Ok(())
}


pub fn parse_to_file(path: &str) -> StatusResult<GTDFile>
{
    match toml::from_str::<GTDFile>(&read_file(path)?)
    {
        Ok(value) => Ok(value),
        Err(_) => Err("couldn't parse from file")
    }
}


pub fn parse_to_string(file: GTDFile) -> StatusResult<String>
{
    match toml::to_string_pretty(&file)
    {
        Ok(value) => Ok(value),
        Err(_) => Err("couldn't parse to file")
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


pub fn from_gtd(lists: &Vec<List>) -> GTDFile
{
    let mut gtd_file_lists = Vec::<GTDFileList>::new();

    for list in lists
    {
        let mut tasks = Vec::<GTDFileTask>::new();

        for task in list.tasks()
        {
            tasks.push(GTDFileTask {
                name: task.message.clone(),
                details: task._details.clone(),
                contexts: task.contexts().clone()
            });
        }

        gtd_file_lists.push(GTDFileList {
            name: list.name.clone(),
            tasks
        });
    }

    GTDFile { lists: gtd_file_lists }
}

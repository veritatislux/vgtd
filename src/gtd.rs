use std::fs;

use serde::Deserialize;
use serde::Serialize;

use crate::EResult;

#[derive(Serialize, Deserialize)]
pub struct Task
{
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct Project
{
    pub name: String,
    pub tasks: Vec<Task>,
}

#[derive(Serialize, Deserialize)]
pub struct List
{
    pub name: String,
    pub tasks: Vec<Task>,
    pub projects: Vec<Project>,
}

impl List
{
    pub fn get_task(&mut self, name: &str) -> Option<&mut Task>
    {
        self.tasks.iter_mut().find(|task| task.name == name)
    }

    pub fn get_project(&mut self, name: &str) -> Option<&mut Project>
    {
        self.projects
            .iter_mut()
            .find(|project| project.name == name)
    }
}

impl List
{
    pub fn new(name: String) -> Self
    {
        Self {
            name,
            tasks: vec![],
            projects: vec![],
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct File
{
    pub lists: Vec<List>,
}

impl File
{
    pub fn get_list(&mut self, name: &str) -> Option<&mut List>
    {
        self.lists.iter_mut().find(|list| list.name == name)
    }

    pub fn write_to_file(&self, path: &str) -> EResult<()>
    {
        let contents = toml::to_string(self)?;

        if let Err(error) = fs::write(path, contents)
        {
            return Err(Box::new(error));
        }

        Ok(())
    }
}

use std::fs;
use std::io;
use std::io::ErrorKind;

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

impl Project
{
    pub fn new(name: String) -> Self
    {
        Self {
            name,
            tasks: vec![],
        }
    }
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
    pub fn new(name: String) -> Self
    {
        Self {
            name,
            tasks: vec![],
            projects: vec![],
        }
    }

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

#[derive(Serialize, Deserialize)]
pub struct File
{
    pub lists: Vec<List>,
}

impl File
{
    pub fn get_list(&mut self, name: &str) -> EResult<&List>
    {
        self.lists
            .iter()
            .find(|list: &&List| list.name == name)
            .ok_or(Box::new(io::Error::new(
                ErrorKind::NotFound,
                "List not found.",
            )))
    }

    pub fn get_list_mut(&mut self, name: &str) -> EResult<&mut List>
    {
        self.lists
            .iter_mut()
            .find(|list: &&mut List| list.name == name)
            .ok_or(Box::new(io::Error::new(
                ErrorKind::NotFound,
                "List not found.",
            )))
    }

    pub fn get_move_anchors(
        &mut self,
        origin_name: &str,
        target_name: &str,
    ) -> EResult<(&mut List, &mut List)>
    {
        if origin_name == target_name
        {
            return Err(Box::new(io::Error::new(
                ErrorKind::InvalidInput,
                "List names must be different.",
            )));
        }

        let mut origin_list: Option<&mut List> = None;
        let mut target_list: Option<&mut List> = None;

        let mut found_origin = false;
        let mut found_target = false;

        for list in self.lists.iter_mut()
        {
            if list.name == origin_name
            {
                if found_origin
                {
                    return Err(Box::new(io::Error::new(
                        ErrorKind::InvalidData,
                        "File error: two lists with the same name.",
                    )));
                }

                origin_list = Some(list);
                found_origin = true;
            }
            else if list.name == target_name
            {
                if found_target
                {
                    return Err(Box::new(io::Error::new(
                        ErrorKind::InvalidData,
                        "File error: two lists with the same name.",
                    )));
                }

                target_list = Some(list);
                found_target = true;
            }
        }

        Ok((
            match origin_list
            {
                Some(list) => list,
                None =>
                {
                    return Err(Box::new(io::Error::new(
                        ErrorKind::NotFound,
                        "Origin list not found.",
                    )));
                }
            },
            match target_list
            {
                Some(list) => list,
                None =>
                {
                    return Err(Box::new(io::Error::new(
                        ErrorKind::NotFound,
                        "Target list not found.",
                    )));
                }
            },
        ))
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

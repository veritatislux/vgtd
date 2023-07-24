use std::fs;
use std::io;

use serde::Deserialize;
use serde::Serialize;

use crate::EResult;

#[derive(Serialize, Deserialize)]
pub struct Task
{
    pub name: String,
    pub description: Option<String>,
}

impl Task
{
    pub fn new(name: String, description: Option<String>) -> Self
    {
        Self { name, description }
    }
}

pub trait TaskContainer
{
    fn tasks(&self) -> &Vec<Task>;

    fn tasks_mut(&mut self) -> &mut Vec<Task>;

    fn get_task(&self, index: usize) -> Option<&Task>
    {
        self.tasks().get(index)
    }

    fn get_task_mut(&mut self, index: usize) -> Option<&mut Task>
    {
        self.tasks_mut().get_mut(index)
    }

    fn get_task_forced(&self, index: usize) -> EResult<&Task>
    {
        match self.get_task(index)
        {
            Some(project) => Ok(project),
            None =>
            {
                return Err(Box::new(io::Error::new(
                    io::ErrorKind::NotFound,
                    "Task not found.",
                )));
            }
        }
    }

    fn get_task_mut_forced(&mut self, index: usize) -> EResult<&mut Task>
    {
        match self.get_task_mut(index)
        {
            Some(project) => Ok(project),
            None =>
            {
                return Err(Box::new(io::Error::new(
                    io::ErrorKind::NotFound,
                    "Task not found.",
                )));
            }
        }
    }

    fn task_exists(&self, name: &str) -> bool
    {
        self.tasks().iter().find(|task| task.name == name).is_some()
    }

    fn task_exists_forced(&self, name: &str) -> EResult<bool>
    {
        if self.task_exists(name)
        {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "Task already exists.",
            )));
        }

        Ok(true)
    }

    fn push_task(&mut self, task: Task) -> () { self.tasks_mut().push(task); }

    fn remove_task(&mut self, index: usize) -> Task
    {
        self.tasks_mut().remove(index)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Project
{
    pub name: String,
    tasks: Vec<Task>,
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

impl TaskContainer for Project
{
    fn tasks(&self) -> &Vec<Task> { &self.tasks }

    fn tasks_mut(&mut self) -> &mut Vec<Task> { &mut self.tasks }
}

pub trait ProjectContainer
{
    fn projects(&self) -> &Vec<Project>;

    fn projects_mut(&mut self) -> &mut Vec<Project>;

    fn get_project(&self, index: usize) -> Option<&Project>
    {
        self.projects().get(index)
    }

    fn get_project_mut(&mut self, index: usize) -> Option<&mut Project>
    {
        self.projects_mut().get_mut(index)
    }

    fn get_project_forced(&self, index: usize) -> EResult<&Project>
    {
        match self.get_project(index)
        {
            Some(project) => Ok(project),
            None =>
            {
                return Err(Box::new(io::Error::new(
                    io::ErrorKind::NotFound,
                    "Task not found.",
                )));
            }
        }
    }

    fn get_project_mut_forced(&mut self, index: usize)
        -> EResult<&mut Project>
    {
        match self.get_project_mut(index)
        {
            Some(project) => Ok(project),
            None =>
            {
                return Err(Box::new(io::Error::new(
                    io::ErrorKind::NotFound,
                    "Task not found.",
                )));
            }
        }
    }

    fn project_exists(&self, name: &str) -> bool
    {
        self.projects()
            .iter()
            .find(|project| project.name == name)
            .is_some()
    }

    fn project_exists_forced(&self, name: &str) -> EResult<bool>
    {
        if self.project_exists(name)
        {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "Project already exists.",
            )));
        }

        Ok(true)
    }

    fn push_project(&mut self, project: Project) -> ()
    {
        self.projects_mut().push(project);
    }

    fn remove_project(&mut self, index: usize) -> Project
    {
        self.projects_mut().remove(index)
    }
}

#[derive(Serialize, Deserialize)]
pub struct List
{
    pub name: String,
    tasks: Vec<Task>,
    projects: Vec<Project>,
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

impl TaskContainer for List
{
    fn tasks(&self) -> &Vec<Task> { &self.tasks }

    fn tasks_mut(&mut self) -> &mut Vec<Task> { &mut self.tasks }
}

impl ProjectContainer for List
{
    fn projects(&self) -> &Vec<Project> { &self.projects }

    fn projects_mut(&mut self) -> &mut Vec<Project> { &mut self.projects }
}

pub trait ListContainer
{
    fn lists(&self) -> &Vec<List>;

    fn lists_mut(&mut self) -> &mut Vec<List>;

    fn get_list(&self, name: &str) -> Option<&List>
    {
        self.lists().iter().find(|list| list.name == name)
    }

    fn get_list_mut(&mut self, name: &str) -> Option<&mut List>
    {
        self.lists_mut().iter_mut().find(|list| list.name == name)
    }

    fn get_list_forced(&mut self, name: &str) -> EResult<&List>
    {
        match self.get_list(name)
        {
            Some(list) => Ok(list),
            None =>
            {
                return Err(Box::new(io::Error::new(
                    io::ErrorKind::NotFound,
                    "List not found.",
                )));
            }
        }
    }

    fn get_list_mut_forced(&mut self, name: &str) -> EResult<&mut List>
    {
        match self.get_list_mut(name)
        {
            Some(list) => Ok(list),
            None =>
            {
                return Err(Box::new(io::Error::new(
                    io::ErrorKind::NotFound,
                    "List not found.",
                )));
            }
        }
    }

    fn list_exists(&self, name: &str) -> bool
    {
        self.lists()
            .iter()
            .find(|project| project.name == name)
            .is_some()
    }

    fn list_exists_forced(&self, name: &str) -> EResult<bool>
    {
        if self.list_exists(name)
        {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::AlreadyExists,
                "List already exists.",
            )));
        }

        Ok(true)
    }

    fn push_list(&mut self, list: List) -> () { self.lists_mut().push(list); }
}

#[derive(Serialize, Deserialize)]
pub struct File
{
    pub lists: Vec<List>,
}

impl File
{
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

impl ListContainer for File
{
    fn lists(&self) -> &Vec<List> { &self.lists }

    fn lists_mut(&mut self) -> &mut Vec<List> { &mut self.lists }
}

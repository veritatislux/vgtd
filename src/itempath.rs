use std::io;

use crate::EResult;
use crate::tos::OutputFormattable;
use crate::tos;

const PATH_DIVISOR: char = '/';

pub fn parse_index(source: &str) -> EResult<usize>
{
    match source.parse::<usize>()
    {
        Ok(idx) => Ok(idx),
        Err(_) =>
        {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Could not parse an index.",
            )))
        }
    }
}

pub struct TaskPath
{
    pub list_name: String,
    pub project_index: Option<usize>,
    pub task_index: usize,
}

impl TaskPath
{
    pub fn parse(source: &str) -> EResult<Self>
    {
        let source = source.to_lowercase();

        if source.is_empty()
        {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid task path: empty path.",
            )));
        }

        let sections: Vec<&str> = source.split(PATH_DIVISOR).collect();

        if sections.len() < 2
        {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid task path: no task index provided.",
            )));
        };

        if sections.len() > 3
        {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid task path: unrecognized additional path section.",
            )));
        };

        let list_name: String = sections[0].to_owned();

        let project_index: Option<usize> = if sections.len() == 2
        {
            None
        }
        else
        {
            Some(parse_index(sections[1])?)
        };

        let task_index: usize = parse_index(
            sections
                .last()
                .expect("Last path section should be the task index"),
        )?;

        Ok(Self {
            list_name,
            project_index,
            task_index,
        })
    }
}

impl OutputFormattable for TaskPath
{
    fn tos_format(&self) -> String {
        format!(
            "{}{}/{}",
            tos::format_list_name(&self.list_name),
            if let Some(project_index) = self.project_index
            {
                format!("/{}", tos::format_index(project_index))
            }
            else
            {
                String::new()
            },
            tos::format_index(self.task_index)
        )
    }
}

pub struct ContainerPath
{
    pub list_name: String,
    pub project_index: Option<usize>,
}

impl ContainerPath
{
    pub fn parse(source: &str) -> EResult<Self>
    {
        let source = source.to_lowercase();

        if source.is_empty()
        {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid container path: empty path.",
            )));
        }

        let sections: Vec<&str> = source.split(PATH_DIVISOR).collect();

        if sections.len() > 2
        {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Invalid container path: unrecognized additional path section.",
            )));
        };

        if sections.len() > 1
        {
            Ok(Self {
                list_name: sections[0].to_owned(),
                project_index: Some(parse_index(sections[1])?),
            })
        }
        else
        {
            Ok(Self {
                list_name: sections[0].to_owned(),
                project_index: None,
            })
        }
    }
}

impl OutputFormattable for ContainerPath
{
    fn tos_format(&self) -> String {
        format!(
            "{}{}",
            tos::format_list_name(&self.list_name),
            if let Some(project_index) = self.project_index
            {
                format!("/{}", tos::format_index(project_index))
            }
            else
            {
                String::new()
            }
        )
    }
}

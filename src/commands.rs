use std::io;
use std::io::ErrorKind;
use std::path;

use crate::gtd;
use crate::gtd::File;
use crate::gtd::List;
use crate::gtd::Project;
use crate::gtd::Task;
use crate::indexer;
use crate::itempath;
use crate::text::Formattable;
use crate::EResult;

use crate::gtd::ListContainer;
use crate::gtd::ProjectContainer;
use crate::gtd::TaskContainer;

pub const GTD_FILE_PATH: &str = ".gtd.toml";

pub fn write_project_defaults() -> EResult<()>
{
    let basic_structure = File {
        lists: vec![
            gtd::List::new("inbox".to_owned()),
            gtd::List::new("next".to_owned()),
            gtd::List::new("done".to_owned()),
        ],
    };

    basic_structure.write_to_file(GTD_FILE_PATH)?;

    Ok(())
}

pub fn reset_project() -> EResult<()>
{
    if !path::Path::new(GTD_FILE_PATH).exists()
    {
        return Err(Box::new(io::Error::new(
            ErrorKind::NotFound,
            "Project not found.",
        )));
    }

    write_project_defaults()?;

    println!("The project has been reset.");

    Ok(())
}

pub fn init_project() -> EResult<()>
{
    if path::Path::new(GTD_FILE_PATH).exists()
    {
        return Err(Box::new(io::Error::new(
            ErrorKind::AlreadyExists,
            "Project already exists.",
        )));
    }

    write_project_defaults()?;

    println!("A new project has been initialized at this directory.");

    Ok(())
}

pub fn create_task(
    file: &mut File,
    path: String,
    name: String,
    description: Option<String>,
) -> EResult<()>
{
    let task_path = itempath::ContainerPath::parse(&path)?;

    let list = file.get_list_mut_forced(&task_path.list_name)?;

    if let Some(project_index) = task_path.project_index
    {
        let project = list.get_project_mut_forced(project_index)?;

        project.task_exists_forced(&name)?;

        let task = Task { name, description };

        let task_name = task.name.clone();

        project.push_task(task);

        println!(
            "Task {}/{} (\"{}\") created.",
            path.to_titlecase(),
            project.tasks().len(),
            &task_name,
        );
    }
    else
    {
        list.task_exists_forced(&name)?;

        let task = Task { name, description };

        let task_name = task.name.clone();

        list.push_task(task);

        println!(
            "Task {}/{} (\"{}\") created.",
            path.to_titlecase(),
            list.tasks().len(),
            &task_name,
        );
    }

    Ok(())
}

pub fn remove_task(file: &mut File, path: String) -> EResult<()>
{
    let task_path = itempath::TaskPath::parse(&path)?;

    let list = file.get_list_mut_forced(&task_path.list_name)?;

    if let Some(project_index) = task_path.project_index
    {
        let project = list.get_project_mut_forced(project_index)?;

        let task_name =
            project.get_task_forced(task_path.task_index)?.name.clone();

        project.remove_task(task_path.task_index);

        println!("Task {} ({}) removed.", &path, &task_name);
    }
    else
    {
        let task_name =
            list.get_task_forced(task_path.task_index)?.name.clone();

        list.remove_task(task_path.task_index);

        println!("Task {} ({}) removed.", &path, &task_name);
    };

    Ok(())
}

pub fn move_task(file: &mut File, source: &str, target: &str) -> EResult<()>
{
    let source_path = itempath::TaskPath::parse(source)?;
    let target_path = itempath::ContainerPath::parse(target)?;

    let source_list = file.get_list_mut_forced(&source_path.list_name)?;

    let task = if let Some(project_index) = source_path.project_index
    {
        let project = source_list.get_project_mut_forced(project_index)?;

        project.get_task_forced(source_path.task_index)?;

        project.remove_task(source_path.task_index)
    }
    else
    {
        source_list.get_task_forced(source_path.task_index)?;

        source_list.remove_task(source_path.task_index)
    };

    let task_name = task.name.clone();

    let target_list = file.get_list_mut_forced(&target_path.list_name)?;

    if let Some(project_index) = source_path.project_index
    {
        let project = target_list.get_project_mut_forced(project_index)?;

        project.push_task(task);
    }
    else
    {
        target_list.push_task(task);
    };

    println!("Moved task {} to {} (\"{}\").", source, target, task_name,);

    Ok(())
}

pub fn move_project(file: &mut File, source: &str, target: &str)
    -> EResult<()>
{
    let source_path = itempath::ContainerPath::parse(source)?;

    let target_path = itempath::ContainerPath::parse(target)?;

    let source_index = match source_path.project_index
    {
        Some(index) => index,
        None =>
        {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Please specify a project in the source path.",
            )));
        }
    };

    if target_path.project_index.is_some()
    {
        return Err(Box::new(io::Error::new(
            io::ErrorKind::InvalidInput,
            "Can not move a project to another project.",
        )));
    }

    let source_list = file.get_list_mut_forced(&source_path.list_name)?;

    source_list.get_project_forced(source_index)?;

    let project = source_list.remove_project(source_index);

    let project_name = project.name.clone();

    let target_list = file.get_list_mut_forced(&target_path.list_name)?;

    target_list.push_project(project);

    println!(
        "Project {} moved to {} (\"{}\")",
        &source, &target, &project_name
    );

    Ok(())
}

pub fn create_list(file: &mut File, name: String) -> EResult<()>
{
    if let Some(_) = file.lists.iter().find(|list: &&List| list.name == name)
    {
        return Err(Box::new(io::Error::new(
            ErrorKind::AlreadyExists,
            "List already exists.",
        )));
    }

    let list = List::new(name.clone());

    let formatted_name = list.name.to_titlecase();

    file.lists.push(list);

    println!("List {formatted_name} created.");

    Ok(())
}

pub fn remove_list(file: &mut File, name: &str) -> EResult<()>
{
    let index =
        match file.lists.iter().position(|list: &List| list.name == name)
        {
            Some(index) => index,
            None =>
            {
                return Err(Box::new(io::Error::new(
                    ErrorKind::NotFound,
                    "List not found.",
                )));
            }
        };

    file.lists.remove(index);

    println!("List {} removed.", name.to_titlecase());

    Ok(())
}

pub fn show_list(file: &mut File, name: &str) -> EResult<()>
{
    let name = name.to_lowercase();

    let list = file.get_list_forced(&name)?;

    let formatted_name = name.to_titlecase();

    if list.tasks().is_empty() && list.projects().is_empty()
    {
        println!("List {formatted_name} is empty.");

        return Ok(());
    }

    println!("List {formatted_name}'s contents:");

    for (index, project) in list.projects().iter().enumerate()
    {
        println!(
            "(Project) {} - {} ({} tasks)",
            indexer::index_to_identifier(index),
            project.name.to_titlecase(),
            project.tasks().len()
        );
    }

    for (index, task) in list.tasks().iter().enumerate()
    {
        println!("{} - {}", indexer::index_to_identifier(index), task.name);
    }

    Ok(())
}

pub fn create_project(
    file: &mut File,
    list_name: &str,
    name: String,
) -> EResult<()>
{
    let name = name.to_lowercase();

    let list = file.get_list_mut_forced(list_name)?;

    list.project_exists_forced(&name)?;

    let project = Project::new(name);

    let project_name = project.name.clone();

    list.push_project(project);

    println!(
        "Project {}/{} (\"{}\") created.",
        list_name.to_titlecase(),
        list.projects().len() - 1,
        project_name.to_titlecase()
    );

    Ok(())
}

pub fn remove_project(file: &mut File, path: &str) -> EResult<()>
{
    let project_path = itempath::ContainerPath::parse(path)?;

    let list = file.get_list_mut_forced(&project_path.list_name)?;

    if let Some(index) = project_path.project_index
    {
        let project_name = list.get_project_forced(index)?.name.clone();

        list.remove_project(index);

        println!("Project {} (\"{}\") removed.", &path, &project_name);
    }

    Ok(())
}

// TODO: Implement this
// pub fn move_project(
//     file: &mut File,
//     source: &str,
//     destination: &str
// ) -> EResult<()>
// {
//     let source_path = itempath::ContainerPath::parse(source)?;
// }

pub fn show_all_lists(file: &mut File) -> EResult<()>
{
    println!("Lists in the current VoltGTD project:");

    for list in file.lists.iter_mut()
    {
        println!("- {}", list.name.to_titlecase());
    }

    Ok(())
}

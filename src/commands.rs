use std::io;
use std::io::ErrorKind;
use std::path;

use colored::Colorize;

use crate::gtd;
use crate::gtd::File;
use crate::gtd::List;
use crate::gtd::Project;
use crate::gtd::Task;
use crate::indexer;
use crate::itempath;
use crate::tos;
use crate::tos::OutputFormattable;
use crate::EResult;

use crate::gtd::ListContainer;
use crate::gtd::ProjectContainer;
use crate::gtd::TaskContainer;

pub const GTD_FILE_PATH: &str = ".gtd.toml";

pub fn write_workspace_defaults() -> EResult<()>
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

pub fn reset_workspace() -> EResult<()>
{
    if !path::Path::new(GTD_FILE_PATH).exists()
    {
        return Err(Box::new(io::Error::new(
            ErrorKind::NotFound,
            "Project not found.",
        )));
    }

    write_workspace_defaults()?;

    tos::send_success("The workspace has been reset.");

    Ok(())
}

pub fn initialize_workspace() -> EResult<()>
{
    if path::Path::new(GTD_FILE_PATH).exists()
    {
        return Err(Box::new(io::Error::new(
            ErrorKind::AlreadyExists,
            "Workspace already exists.",
        )));
    }

    write_workspace_defaults()?;

    tos::send_success("New workspace initialized in this directory.");

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

        let task = Task::new(name.clone(), description.clone());

        project.push_task(task);
    }
    else
    {
        list.task_exists_forced(&name)?;

        let task = Task::new(name.clone(), description.clone());

        list.push_task(task);
    }

    tos::send_success(&format!(
        "Task {} created at {}.",
        &name,
        &task_path.tos_format(),
    ));

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

        tos::send_success(&format!(
            "Task {} ({}) removed.",
            &path, &task_name
        ));
    }
    else
    {
        let task_name =
            list.get_task_forced(task_path.task_index)?.name.clone();

        list.remove_task(task_path.task_index);

        tos::send_success(&format!(
            "Task {} ({}) removed.",
            &path, &task_name
        ));
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

    let new_index = if let Some(project_index) = target_path.project_index
    {
        let project = target_list.get_project_mut_forced(project_index)?;

        project.push_task(task);

        project.tasks().len() - 1
    }
    else
    {
        target_list.push_task(task);

        target_list.tasks().len() - 1
    };

    tos::send_success(&format!(
        "Moved task {} to {}/{} ({}).",
        &source_path.tos_format(),
        &target_path.tos_format(),
        tos::format_index(new_index),
        task_name,
    ));

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

    let project_status = project.status();

    let target_list = file.get_list_mut_forced(&target_path.list_name)?;

    target_list.push_project(project);

    tos::send_success(&format!(
        "Project {} moved to {}/{} ({})",
        &source_path.tos_format(),
        &target_path.tos_format(),
        tos::format_index(target_list.projects().len() - 1),
        tos::format_project(&project_name, &project_status),
    ));

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

    let formatted_name = tos::format_list_name(&list.name);

    file.lists.push(list);

    tos::send_success(&format!("List {formatted_name} created."));

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

    tos::send_success(&format!(
        "List {} removed.",
        tos::format_list_name(&name)
    ));

    Ok(())
}

pub fn show_list(file: &mut File, name: &str, all: bool) -> EResult<()>
{
    let name = name.to_lowercase();

    let list = file.get_list_forced(&name)?;

    let formatted_name = tos::format_list_name(&name);

    if list.tasks().is_empty() && list.projects().is_empty()
    {
        tos::send_success(&format!("List {formatted_name} is empty."));

        return Ok(());
    }

    let mut output = tos::OutputBlock::new();

    output
        .insert_line(
            &format!(
                "Contents of list {}",
                formatted_name.color(tos::COLOR_IDENTIFIER)
            ),
            0,
        )
        .insert_text("\n");

    // TODO: Show the empty projects first
    if !list.projects().is_empty()
    {
        output.insert_line(
            &format!(
                "{} {}/{} ({}%)",
                &tos::format_section_name("projects"),
                tos::format_number(list.projects_completed()),
                tos::format_number(list.projects().len()),
                tos::format_number(format!(
                    "{:.1}",
                    list.projects_completion() * 100.0
                )),
            ),
            1,
        );

        for (index, project) in list.projects().iter().enumerate()
        {
            output.insert_line(
                &format!(
                    "{}. {} {}/{} ({}%)",
                    indexer::index_to_identifier(index)
                        .color(tos::COLOR_NUM_VALUE),
                    tos::format_project(&project.name, &project.status()),
                    tos::format_number(project.tasks_completed()),
                    tos::format_number(project.tasks().len()),
                    tos::format_number(format!(
                        "{:.1}",
                        project.tasks_completion() * 100.0
                    )),
                ),
                2,
            );

            if all && !project.tasks().is_empty()
            {
                for (index, task) in project.tasks().iter().enumerate()
                {
                    output.insert_line(
                        &format!(
                            "{}. {}",
                            // TODO: Use this function at the formatting func
                            indexer::index_to_identifier(index)
                                .color(tos::COLOR_NUM_VALUE),
                            tos::format_task(&task)
                        ),
                        3,
                    );
                }

                if index < list.projects().len() - 1
                {
                    output.insert_text("\n");
                }
            }
        }

        output.insert_text("\n");
    }

    if !list.tasks().is_empty()
    {
        output.insert_line(
            &format!(
                "{} {}/{} ({}%)",
                &tos::format_section_name("tasks"),
                tos::format_number(list.tasks_completed()),
                tos::format_number(list.tasks().len()),
                tos::format_number(format!(
                    "{:.1}",
                    list.tasks_completion() * 100.0
                )),
            ),
            1,
        );

        for (index, task) in list.tasks().iter().enumerate()
        {
            output.insert_line(
                &format!(
                    "{}. {}",
                    indexer::index_to_identifier(index)
                        .color(tos::COLOR_NUM_VALUE),
                    tos::format_task(&task),
                ),
                2,
            );
        }
    }

    output.send();

    Ok(())
}

pub fn show_project(file: &mut File, path: &str) -> EResult<()>
{
    let project_path = itempath::ContainerPath::parse(path)?;

    let project_index = match project_path.project_index
    {
        Some(index) => index,
        None =>
        {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidInput,
                "No project index provided.",
            )));
        }
    };

    let list = file.get_list_forced(&project_path.list_name)?;

    let project = list.get_project_forced(project_index)?;

    let mut output = tos::OutputBlock::new();

    output
        .insert_line(
            &format!(
                "Contents of project {}",
                tos::format_project(&project.name, &project.status()),
            ),
            0,
        )
        .insert_text("\n");

    for (index, task) in project.tasks().iter().enumerate()
    {
        output.insert_line(
            &format!(
                "{}. {}",
                indexer::index_to_identifier(index)
                    .color(tos::COLOR_NUM_VALUE),
                tos::format_task(&task)
            ),
            0,
        );
    }

    output.send();

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

    let project_status = project.status();

    list.push_project(project);

    tos::send_success(&format!(
        "Project {}/{} ({}) created.",
        tos::format_list_name(&list_name),
        (list.projects().len() - 1)
            .to_string()
            .color(tos::COLOR_NUM_VALUE),
        tos::format_project(&project_name, &project_status),
    ));

    Ok(())
}

pub fn remove_project(file: &mut File, path: &str) -> EResult<()>
{
    let project_path = itempath::ContainerPath::parse(path)?;

    let list = file.get_list_mut_forced(&project_path.list_name)?;

    if let Some(index) = project_path.project_index
    {
        let project = list.get_project_forced(index)?;
        let project_name = project.name.clone();
        let project_status = project.status();

        list.remove_project(index);

        tos::send_success(&format!(
            "Project {}/{} ({}) removed.",
            tos::format_list_name(&project_path.list_name),
            index.to_string().color(tos::COLOR_NUM_VALUE),
            tos::format_project(&project_name, &project_status),
        ));
    }

    Ok(())
}

pub fn show_all_lists(file: &mut File) -> EResult<()>
{
    if file.lists().is_empty()
    {
        tos::send_info(&format!("There are no lists in this workspace."));
        return Ok(());
    }

    let mut output = tos::OutputBlock::new();

    output
        .insert_line("Lists in the current workspace", 0)
        .insert_text("\n");

    for list in file.lists.iter()
    {
        output.insert_line(
            &format!(
                "• {} ({} tasks, {} projects)",
                tos::format_list_name(&list.name),
                tos::format_number(list.tasks().len()),
                tos::format_number(list.projects().len())
            ),
            1,
        );
    }

    output.send();

    Ok(())
}

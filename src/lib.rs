mod file;
mod gtd;
mod indexer;

use std::error::Error;
use std::io;
use std::io::ErrorKind;
use std::path;

use clap::Parser;
use clap::Subcommand;
use gtd::List;
use gtd::File;
use gtd::Project;
use gtd::Task;

pub type EResult<T> = Result<T, Box<dyn Error>>;

const GTD_FILE_PATH: &str = ".gtd.toml";

/// Commands to deal with projects
#[derive(Subcommand)]
pub enum ProjectSubcommand
{
    /// Create a project
    Create
    {
        /// The list path where the project will be created
        list: String,
        /// The project's name
        name: String,
    },

    /// Remove a project
    Remove
    {
        /// The list path where the project is
        list: String,
        /// The name of the project to be removed
        project: String,
    },
}

/// Commands to deal with lists
#[derive(Subcommand)]
pub enum ListSubcommand
{
    /// Create a list
    Create
    {
        /// The new list's name
        name: String,
    },

    /// Remove a list
    Remove
    {
        /// The name of the list to be removed
        list: String,
    },

    /// Show the contents of a list
    Show
    {
        /// The name of the list to show the contents of
        list: String,
    },
}

/// Commands to deal with tasks
#[derive(Subcommand)]
pub enum TaskSubcommand
{
    /// Create a task
    Create
    {
        /// The list path where the task will be created
        list: String,
        /// The task's name
        name: String,
        /// The task's description
        description: Option<String>,
    },

    /// Remove a task
    Remove
    {
        /// The list path where the task is
        list: String,
        /// The task's identifier
        identifier: String,
    },

    /// Move a task from one list to the other
    Move
    {
        /// The list path where the task is
        list: String,
        /// The task's identifier
        identifier: String,
        /// The list path to move the task to
        new_list: String,
    },
}

#[derive(Subcommand)]
pub enum GTDSubcommand
{
    Task
    {
        #[command(subcommand)]
        sub: TaskSubcommand,
    },

    List
    {
        #[command(subcommand)]
        sub: ListSubcommand,
    },

    Project
    {
        #[command(subcommand)]
        sub: ProjectSubcommand,
    },

    /// Initialize a new VoltGTD project (create .gtd.toml file)
    Init,

    /// Reset an existing VoltGTD project
    Reset,

    /// Show all the lists in the VoltGTD project
    Lists,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args
{
    #[command(subcommand)]
    sub: GTDSubcommand,
}

pub fn write_project_defaults() -> EResult<()>
{
    let basic_structure = File {
        lists: vec![
            gtd::List {
                name: "Inbox".to_owned(),
                tasks: vec![],
                projects: vec![],
            },
            gtd::List {
                name: "Next".to_owned(),
                tasks: vec![],
                projects: vec![],
            },
            gtd::List {
                name: "Done".to_owned(),
                tasks: vec![],
                projects: vec![],
            },
        ],
    };

    basic_structure.write_to_file(GTD_FILE_PATH)?;

    Ok(())
}

// Command functions
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

    println!("Project initialized.");
    Ok(())
}

pub fn create_task(
    file: &mut File,
    list_name: String,
    name: String,
    description: Option<String>,
) -> EResult<()>
{
    let list = file.get_list_mut(&list_name)?;

    if let Some(_) = list.get_task(&name)
    {
        return Err(Box::new(io::Error::new(
            ErrorKind::AlreadyExists,
            "Task already exists.",
        )));
    }

    let task = Task {
        name: name.clone(),
        description: description.unwrap_or(String::new()),
    };

    list.tasks.push(task);

    let identifier = indexer::index_to_identifier(list.tasks.len() - 1);

    println!("Task {list_name}/{identifier} ({name}) created.");
    Ok(())
}

pub fn remove_task(
    file: &mut File,
    list_name: String,
    identifier: String,
) -> EResult<()>
{
    let list = file.get_list_mut(&list_name)?;

    let index = indexer::identifier_to_index(&identifier)?;

    if index >= list.tasks.len()
    {
        return Err(Box::new(io::Error::new(
            ErrorKind::NotFound,
            "Task not found.",
        )));
    }

    let removed_task = list.tasks.remove(index);

    println!(
        "Task {}/{} ({}) removed.",
        &list_name, &identifier, removed_task.name
    );

    Ok(())
}

pub fn move_task(
    file: &mut File,
    origin_list_name: String,
    identifier: String,
    target_list_name: String,
) -> EResult<()>
{
    let (origin_list, target_list) =
        file.get_move_anchors(&origin_list_name, &target_list_name)?;

    let index = indexer::identifier_to_index(&identifier)?;

    if index >= origin_list.tasks.len()
    {
        return Err(Box::new(io::Error::new(
            ErrorKind::NotFound,
            "Task not found.",
        )));
    }

    let task = origin_list.tasks.remove(index);

    target_list.tasks.push(task);

    let new_index = target_list.tasks.len() - 1;

    let new_identifier = indexer::index_to_identifier(new_index);

    println!(
        "Moved task {}/{} to {}/{} ({}).",
        origin_list_name,
        identifier,
        target_list_name,
        new_identifier,
        target_list.tasks[new_index].name
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

    file.lists.push(list);

    println!("List {name} created.");

    Ok(())
}

pub fn remove_list(file: &mut File, name: String) -> EResult<()>
{
    let index = match file.lists.iter().position(|list: &List| list.name == name)
    {
        Some(index) => index,
        None => {
            return Err(Box::new(io::Error::new(
                ErrorKind::NotFound,
                "List not found.",
            )));
        }
    };

    file.lists.remove(index);

    println!("List {name} removed.");

    Ok(())
}

pub fn show_list(file: &mut File, name: String) -> EResult<()>
{
    let list = file.get_list(&name)?;

    if list.tasks.is_empty() && list.projects.is_empty()
    {
        println!("List {name} is empty.");

        return Ok(());
    }

    println!("List {name}'s contents:");

    for (index, project) in list.projects.iter().enumerate()
    {
        println!(
            "(Project) {} - {} ({} tasks)",
            indexer::index_to_identifier(index),
            project.name,
            project.tasks.len()
        );
    }

    for (index, task) in list.tasks.iter().enumerate()
    {
        println!("{} - {}", indexer::index_to_identifier(index), task.name);
    }

    Ok(())
}

pub fn create_project(
    file: &mut File,
    list_name: String,
    name: String,
) -> EResult<()>
{
    let list = file.get_list_mut(&list_name)?;

    if let Some(_) = list.get_project(&name)
    {
        return Err(Box::new(io::Error::new(
            ErrorKind::AlreadyExists,
            "Project already exists.",
        )));
    }

    let project = Project::new(name.clone());

    list.projects.push(project);

    println!("Project {list_name}/{name} created.");
    Ok(())
}

pub fn remove_project(file: &mut File, list_name: String, name: String) -> EResult<()>
{
    let list = file.get_list_mut(&list_name)?;

    let index = match list.projects.iter().position(|project: &Project| project.name == name)
    {
        Some(index) => index,
        None => {
            return Err(Box::new(io::Error::new(
                ErrorKind::NotFound,
                "Project not found.",
            )));
        }
    };

    let removed_project = list.projects.remove(index);

    println!("Project {} removed.", removed_project.name);

    Ok(())
}

pub fn show_all_lists(file: &mut File) -> EResult<()>
{
    println!("Lists in the current VoltGTD project:");

    for list in file.lists.iter_mut()
    {
        println!("- {}", list.name);
    }

    Ok(())
}

pub fn parse_cli_arguments() -> EResult<()>
{
    let args = Args::parse();

    if let GTDSubcommand::Init = args.sub
    {
        return init_project();
    }

    if let GTDSubcommand::Reset = args.sub
    {
        return reset_project();
    }

    let mut file = file::parse(GTD_FILE_PATH)?;

    match args.sub
    {
        GTDSubcommand::Task { sub } =>
        {
            match sub
            {
                TaskSubcommand::Create {
                    list,
                    name,
                    description,
                } => create_task(&mut file, list, name, description)?,
                TaskSubcommand::Remove { list, identifier } =>
                {
                    remove_task(&mut file, list, identifier)?
                }
                TaskSubcommand::Move {
                    list,
                    identifier,
                    new_list,
                } => move_task(&mut file, list, identifier, new_list)?,
            }
        }
        GTDSubcommand::List { sub } =>
        {
            match sub
            {
                ListSubcommand::Create { name } =>
                {
                    create_list(&mut file, name)?
                },
                ListSubcommand::Show { list } => show_list(&mut file, list)?,
                ListSubcommand::Remove { list } => {
                    remove_list(&mut file, list)?
                },
            }
        }
        GTDSubcommand::Lists => show_all_lists(&mut file)?,
        GTDSubcommand::Project { sub } =>
        {
            match sub
            {
                ProjectSubcommand::Create { list, name } => create_project(&mut file, list, name)?,
                ProjectSubcommand::Remove { list, project } => remove_project(&mut file, list, project)?,
            }
        },
        _ =>
        {}
    };

    file.write_to_file(GTD_FILE_PATH)
}

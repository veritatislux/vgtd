mod file;
mod gtd;

use std::error::Error;
use std::io;
use std::io::ErrorKind;
use std::path;

use clap::Parser;
use clap::Subcommand;
use gtd::List;

use crate::gtd::Task;

pub type EResult<T> = Result<T, Box<dyn Error>>;

const GTD_FILE_PATH: &str = ".gtd.toml";

/// Commands to deal with projects
#[derive(Subcommand)]
pub enum ProjectSubcommand
{
    /// Create a project
    Create
    {
        /// The list where the project will be created
        list: String,
        /// The new project's name
        name: String,
    },

    /// Remove a project
    Remove
    {
        /// The list where the project is
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
        /// The task's name
        task: String,
    },

    /// Move a task from one list to the other
    Move
    {
        /// The list path where the task is
        list: String,
        /// The task's name
        task: String,
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
    let basic_structure = gtd::File {
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
    file: &mut gtd::File,
    list_name: String,
    name: String,
    description: Option<String>,
) -> EResult<()>
{
    let list = match file.get_list(&list_name)
    {
        Some(list) => list,
        None =>
        {
            return Err(Box::new(io::Error::new(
                ErrorKind::NotFound,
                "List not found.",
            )));
        }
    };

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

    println!("Task {list_name}/{name} created succesfully.");
    Ok(())
}

pub fn create_list(file: &mut gtd::File, name: String) -> EResult<()>
{
    let list = List::new(name.clone());

    file.lists.push(list);

    println!("List {name} created succesfully.");
    Ok(())
}

pub fn show_list(file: &mut gtd::File, name: String) -> EResult<()>
{
    let list = match file.get_list(&name)
    {
        Some(list) => list,
        None =>
        {
            return Err(Box::new(io::Error::new(
                ErrorKind::NotFound,
                "List not found.",
            )));
        }
    };

    println!("List {name}'s contents:");

    for task in list.tasks.iter_mut()
    {
        println!("- {}", task.name);
    }

    Ok(())
}

pub fn show_all_lists(file: &mut gtd::File) -> EResult<()>
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
                _ =>
                {}
            }
        }
        GTDSubcommand::List { sub } =>
        {
            match sub
            {
                ListSubcommand::Create { name } =>
                {
                    create_list(&mut file, name)?
                }
                ListSubcommand::Show { list } => show_list(&mut file, list)?,
                _ =>
                {}
            }
        }
        GTDSubcommand::Lists => show_all_lists(&mut file)?,
        _ =>
        {}
    };

    file.write_to_file(GTD_FILE_PATH)
}

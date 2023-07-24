mod commands;
mod file;
mod gtd;
mod indexer;
mod itempath;
mod text;

use std::error::Error;

use clap::Parser;
use clap::Subcommand;

use crate::commands::GTD_FILE_PATH;

pub type EResult<T> = Result<T, Box<dyn Error>>;

/// Commands to deal with projects
#[derive(Subcommand)]
pub enum ProjectSubcommand
{
    /// Create a project
    Create
    {
        /// The path to create the project at
        path: String,
        name: String,
    },

    /// Remove a project
    Remove
    {
        /// The path of the project to be removed
        path: String,
    },
    /// Move a project
    Move
    {
        /// The current location of the project
        source: String,
        /// The final location of the project
        destination: String,
    },
    /// Show the contents of a project
    Show
    {
        /// The path to the list to be shown
        path: String,
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
        /// The path to create the task at
        path: String,
        /// The task's title
        name: String,
        /// The task's description
        description: Option<String>,
    },

    /// Remove a task
    Remove
    {
        /// The path of the task to be deleted
        path: String,
    },

    /// Move a task from one list to the other
    Move
    {
        /// The path to the current location of the task
        source: String,
        /// The path to the final location of the task
        destination: String,
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

pub fn parse_cli_arguments() -> EResult<()>
{
    let args = Args::parse();

    if let GTDSubcommand::Init = args.sub
    {
        return commands::init_project();
    }

    if let GTDSubcommand::Reset = args.sub
    {
        return commands::reset_project();
    }

    let mut file = file::parse(GTD_FILE_PATH)?;

    match args.sub
    {
        GTDSubcommand::Task { sub } =>
        {
            match sub
            {
                TaskSubcommand::Create {
                    path,
                    name,
                    description,
                } =>
                {
                    commands::create_task(&mut file, path, name, description)?
                }
                TaskSubcommand::Remove { path } =>
                {
                    commands::remove_task(&mut file, path)?
                }
                TaskSubcommand::Move {
                    source,
                    destination,
                } => commands::move_task(&mut file, &source, &destination)?,
            }
        }
        GTDSubcommand::List { sub } =>
        {
            match sub
            {
                ListSubcommand::Show { list } =>
                {
                    commands::show_list(&mut file, &list)?
                }
                ListSubcommand::Create { name } =>
                {
                    commands::create_list(&mut file, name)?
                }
                ListSubcommand::Remove { list } =>
                {
                    commands::remove_list(&mut file, &list)?
                }
            }
        }
        GTDSubcommand::Lists => commands::show_all_lists(&mut file)?,
        GTDSubcommand::Project { sub } =>
        {
            match sub
            {
                ProjectSubcommand::Create { path, name } =>
                {
                    commands::create_project(&mut file, &path, name)?
                }
                ProjectSubcommand::Remove { path } =>
                {
                    commands::remove_project(&mut file, &path)?
                }
                ProjectSubcommand::Move {
                    source,
                    destination,
                } => commands::move_project(&mut file, &source, &destination)?,
                ProjectSubcommand::Show { path } =>
                {
                    commands::show_project(&mut file, &path)?
                }
            }
        }
        _ =>
        {}
    };

    file.write_to_file(GTD_FILE_PATH)
}

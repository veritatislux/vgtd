mod commands;
mod dirs;
mod file;
mod gtd;
mod indexer;
mod itempath;
mod text;
pub mod tos;

use std::error::Error;

use clap::Parser;
use clap::Subcommand;
use gtd::Status;

pub type EResult<T> = Result<T, Box<dyn Error>>;

#[derive(Subcommand)]
pub enum ContextSubcommand
{
    /// Create a context within the item
    Create
    {
        /// The item path to create the context at
        path: String,
        /// The context's name
        name: String,
    },

    /// Remove a context within the item
    Remove
    {
        /// The item path of the context's container
        path: String,
        /// The name of the context to be removed
        name: String,
    },

    /// List the contexts inside the item
    List
    {
        /// The item path of the context's container
        path: String,
    },
}

/// Commands to deal with projects
#[derive(Subcommand)]
pub enum ProjectSubcommand
{
    /// Create a project
    Create
    {
        /// The path to create the project at
        path: String,
        /// The name of the project to be created
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
        /// If provided, lists project's tasks as well
        #[arg(long, short)]
        all: bool,
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

    /// Change the status of a task to a new one
    Mark
    {
        /// The path to the task to be modified
        path: String,
        /// The new status for the task (default: DONE)
        new_status: Option<String>,
    },

    Context
    {
        #[command(subcommand)]
        sub: ContextSubcommand,
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

    /// Initialize a new workspace (create .gtd.toml file)
    Init,

    /// Reset an existing workspace
    Reset,

    /// Show all the lists in the workspace
    Lists,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args
{
    #[command(subcommand)]
    sub: GTDSubcommand,

    /// If provided, initialize the global workspace
    #[arg(long, short)]
    global: bool,
}

pub fn parse_cli_arguments() -> EResult<()>
{
    let args = Args::parse();

    let file_path = dirs::get_workspace_file_path(args.global)?;

    if let GTDSubcommand::Init = args.sub
    {
        return commands::initialize_workspace(&file_path);
    }

    if let GTDSubcommand::Reset = args.sub
    {
        return commands::reset_workspace(&file_path);
    }

    let mut file = file::parse(&file_path)?;

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
                TaskSubcommand::Mark { path, new_status } =>
                {
                    commands::mark_task(
                        &mut file,
                        &path,
                        Status::parse(&new_status)?,
                    )?
                }
                TaskSubcommand::Context { sub } =>
                {
                    match sub
                    {
                        ContextSubcommand::Create { path, name } =>
                        {
                            commands::create_context_in_task(
                                &mut file, &path, &name,
                            )?
                        }
                        _ =>
                        {}
                    }
                }
            }
        }
        GTDSubcommand::List { sub } =>
        {
            match sub
            {
                ListSubcommand::Show { list, all } =>
                {
                    commands::show_list(&mut file, &list, all)?
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

    file.write_to_file(&file_path)
}

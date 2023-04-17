pub mod gtd;
pub mod tui;
pub mod render;
pub mod error;

use std::io::stdout;
use std::io::Stdout;

use crossterm::ExecutableCommand;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEventKind;
use crossterm::terminal;

use tui::input;
use tui::Rectangle;
use tui::Position;
use tui::Size;
use error::MessageOnError;


fn add_task_to_list(
    stdout_handle: &mut Stdout,
    terminal_size: Size,
    list: &mut gtd::List
) -> MessageOnError
{
    let task_message: String = match input::get_string(
        "task name",
        stdout_handle,
        terminal_size,
    )?
    {
        None => { return Ok(()); },
        Some(message) => message
    };

    let task = gtd::ListItem::new(task_message);

    list.push_item(task);

    Ok(())
}


fn main_loop() -> Result<(), &'static str>
{
    let mut stdout_handle = stdout();

    let mut current_list = gtd::List::new("example list".to_string());

    let mut item1 = gtd::ListItem::new("build a map".to_string());
    let mut item2 = gtd::ListItem::new("tell a story".to_string());
    let mut item3 = gtd::ListItem::new("read a book".to_string());
    let mut item4 = gtd::ListItem::new("help someone".to_string());
    let mut item5 = gtd::ListItem::new("unexplode creepers".to_string());
    let mut item6 = gtd::ListItem::new("unlearn javascript".to_string());

    item1
        .add_context("cartography lounge".to_string());
    item2
        .add_context("home".to_string())
        .add_context("library".to_string())
        .add_context("university".to_string());
    item3
        .add_context("home".to_string());
    item4
        .add_context("everywhere".to_string());
    item5
        .add_context("minecraft".to_string());
    item6
        .add_context("everywhere".to_string());

    current_list
        .push_item(item1)
        .push_item(item2)
        .push_item(item3)
        .push_item(item4)
        .push_item(item5)
        .push_item(item6);

    let terminal_size = tui::get_terminal_size()?;

    let list_rectangle = Rectangle {
        position: Position { x: 0, y: 0 },
        size: terminal_size
    };

    loop
    {
        render::draw_list(&mut stdout_handle, &current_list, list_rectangle)?;

        render::flush(&mut stdout_handle)?;

        match input::get_event()?
        {
            Event::Key(key_event) => {
                if key_event.kind != KeyEventKind::Press
                {
                    continue;
                }

                match key_event.code
                {
                    KeyCode::Char(character) => {
                        match character
                        {
                            'q' => { break },
                            'a' => {
                                add_task_to_list(
                                    &mut stdout_handle,
                                    terminal_size,
                                    &mut current_list
                                )?;
                            },
                            _ => {}
                        }
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }

    Ok(())
}


pub fn run() -> Result<(), &'static str>
{
    let mut stdout_handle = stdout();

    if let Err(_) = stdout_handle.execute(terminal::EnterAlternateScreen)
    {
        return Err("couldn't enter alternate screen");
    }

    // Process
    main_loop()?;

    // Teardown
    if let Err(_) = stdout_handle.execute(terminal::LeaveAlternateScreen) {
        return Err("couldn't leave alternate screen");
    }

    Ok(())
}

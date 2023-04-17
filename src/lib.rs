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
use error::StatusResult;


fn add_task_to_list(
    stdout_handle: &mut Stdout,
    terminal_size: Size,
    list: &mut gtd::List
) -> StatusResult<()>
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


fn main_loop() -> StatusResult<()>
{
    let mut stdout_handle = stdout();

    let mut current_list = gtd::List::new("example list".to_string());

    let mut item1 = gtd::ListItem::new("build a map".to_string());

    item1.add_context("cartography lounge".to_string());

    current_list.push_item(item1);

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


pub fn run() -> StatusResult<()>
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

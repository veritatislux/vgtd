pub mod gtd;
pub mod tui;
pub mod render;
pub mod error;

use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEventKind;
use crossterm::terminal;

use tui::input;
use tui::Rectangle;
use tui::Position;
use tui::Size;
use error::StatusResult;
use render::Renderer;


fn add_task_to_list(
    renderer: &mut Renderer,
    terminal_size: Size,
    list: &mut gtd::List
) -> StatusResult<()>
{
    let task_name: String = match input::get_string(
        renderer,
        "Create new task",
        "Task name",
        terminal_size,
    )?
    {
        None => { return Ok(()); },
        Some(name) => name
    };

    let task = gtd::Task::new(task_name);

    list.push_task(task);

    Ok(())
}


fn change_task_name(
    renderer: &mut Renderer,
    terminal_size: Size,
    list_task: &mut gtd::Task
) -> StatusResult<()>
{
    let new_task_name: String = match input::get_string_with_preview(
        renderer,
        "Change task name",
        "New task name",
        list_task.message.as_str(),
        terminal_size
    )?
    {
        None => { return Ok(()); },
        Some(new_name) => new_name
    };

    list_task.message = new_task_name;

    Ok(())
}


fn main_loop(renderer: &mut Renderer) -> StatusResult<()>
{
    let mut current_list = gtd::List::new("Example list".to_string());

    let mut task1 = gtd::Task::new("build a map".to_string());

    let mut selected_task_index: usize = 0;

    task1
        .add_context("cartography lounge".to_string())
        .add_context("Santander workplace".to_string());

    current_list.push_task(task1);

    let terminal_size = tui::get_terminal_size()?;

    let list_rectangle = Rectangle {
        position: Position { x: 0, y: 0 },
        size: terminal_size
    };

    loop
    {
        renderer.draw_list(
            &current_list,
            list_rectangle,
            selected_task_index
        )?;

        renderer.flush()?;

        let key_event = if let Event::Key(event) = input::get_event()?
        {
            if event.kind != KeyEventKind::Press
            {
                continue;
            }

            event
        }
        else
        {
            continue;
        };

        if let KeyCode::Char(character) = key_event.code
        {
            match character
            {
                'q' => { break },
                'j' if selected_task_index < current_list.len() - 1 => {
                    selected_task_index += 1;
                },
                'k' if selected_task_index > 0 => {
                    selected_task_index -= 1;
                },
                'a' => {
                    add_task_to_list(
                        renderer,
                        terminal_size,
                        &mut current_list
                    )?;
                },
                'c' => {
                    change_task_name(
                        renderer,
                        terminal_size,
                        &mut current_list.mut_tasks()[selected_task_index]
                    )?;
                },
                _ => {}
            }
        }
    }

    Ok(())
}


pub fn run() -> StatusResult<()>
{
    let mut renderer = Renderer::new();

    if let Err(_) = renderer.execute(terminal::EnterAlternateScreen)
    {
        return Err("couldn't enter alternate screen");
    }

    renderer.hide_cursor()?;

    // Process
    main_loop(&mut renderer)?;

    // Teardown
    if let Err(_) = renderer.execute(terminal::LeaveAlternateScreen) {
        return Err("couldn't leave alternate screen");
    }

    renderer.show_cursor()?;
    renderer.flush()?;

    Ok(())
}

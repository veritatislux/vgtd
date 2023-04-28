pub mod gtd;
pub mod tui;
pub mod render;
pub mod error;
pub mod file;

use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEventKind;

use error::StatusResult;
use gtd::List;
use gtd::Task;
use render::Renderer;
use tui::Position;
use tui::Rectangle;
use tui::Size;
use tui::input;


fn add_task_to_list(
    renderer: &mut Renderer,
    terminal_size: Size,
    list: &mut List
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

    let task = Task::new(task_name);

    list.push_task(task);

    Ok(())
}


fn change_task_name(
    renderer: &mut Renderer,
    terminal_size: Size,
    list_task: &mut Task
) -> StatusResult<()>
{
    let new_task_name: String = match input::get_string_with_preview(
        renderer,
        "Change task name",
        "New task name",
        &list_task.message,
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
    let mut lists = file::to_gtd(file::parse_to_file(".gtd.toml")?);

    let mut selected_task_index: usize = 0;

    let terminal_size = tui::get_terminal_size()?;

    let list_rectangle = Rectangle {
        position: Position { x: 0, y: 0 },
        size: terminal_size
    };

    loop
    {
        renderer.draw_list(
            &lists[0],
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
                's' => {
                    file::write_file(
                        ".gtd.toml",
                        file::parse_to_string(file::from_gtd(&lists))?
                    )?;
                },
                'o' if !lists[0].is_empty() => {
                    selected_task_index = lists[0].sort(
                        selected_task_index
                    );
                },
                'j' if selected_task_index < lists[0].len() - 1 => {
                    selected_task_index += 1;
                },
                'k' if selected_task_index > 0 => {
                    selected_task_index -= 1;
                },
                'a' => {
                    add_task_to_list(
                        renderer,
                        terminal_size,
                        &mut lists[0]
                    )?;

                    selected_task_index = lists[0].len() - 1;
                },
                'c' if !lists[0].is_empty() => {
                    change_task_name(
                        renderer,
                        terminal_size,
                        &mut lists[0].tasks_mut()[selected_task_index]
                    )?;
                },
                'd' if !lists[0].is_empty() => {
                    // TODO: Add a yes/no input asking for confirmation.
                    lists[0].remove_task(selected_task_index);
                },
                _ => {}
            }
        }
    }

    Ok(())
}


pub fn start_interactive_mode(renderer: &mut Renderer) -> StatusResult<()>
{
    renderer.enter_alternate_screen()?;
    renderer.hide_cursor()?;

    main_loop(renderer)?;

    renderer.leave_alternate_screen()?;
    renderer.show_cursor()?;
    renderer.flush()?;

    Ok(())
}


pub fn run()
{
    let mut renderer = Renderer::new();

    if let Err(message) = start_interactive_mode(&mut renderer)
    {
        renderer.leave_alternate_screen().ok();

        eprintln!("(VoltGTD) Error: {message}.");
        std::process::exit(1);
    }
}

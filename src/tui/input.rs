use std::io::Stdout;

use crossterm::cursor;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEventKind;
use crossterm::event::read;
use crossterm::style::Print;

use crate::render;
use crate::tui::Position;
use crate::tui::Rectangle;
use crate::tui::Size;
use crate::tui::get_cursor_position;
use crate::tui::get_terminal_size;


const INPUT_BOX_VERTICAL_PADDING: u16 = 0;
const INPUT_BOX_HORIZONTAL_PADDING: u16 = 1;


pub fn get_event() -> Result<Event, &'static str>
{
    match read()
    {
        Ok(event) => Ok(event),
        Err(_) => Err("couldn't read event")
    }
}


fn capture_string(
    stdout: &mut Stdout,
    terminal_size: Size
) -> Result<Option<String>, &'static str>
{
    let mut input_text = String::new();
    let mut cursor_position = get_cursor_position()?;

    loop
    {
        match get_event()?
        {
            Event::Key(key_event) => {
                if key_event.kind != KeyEventKind::Press
                {
                    continue;
                }

                match key_event.code
                {
                    KeyCode::Char(character) => {
                        input_text.push(character);
                        render::queue(stdout, Print(character))?;
                        cursor_position.x += 1;
                    },
                    KeyCode::Backspace => {
                        match input_text.pop()
                        {
                            Some(_) => {
                                cursor_position.x -= 1;

                                render::queue(
                                    stdout,
                                    cursor::MoveTo(
                                        cursor_position.x,
                                        cursor_position.y
                                    )
                                )?;

                                render::queue(stdout, Print(' '))?;

                                render::queue(
                                    stdout,
                                    cursor::MoveTo(
                                        cursor_position.x,
                                        cursor_position.y
                                    )
                                )?;
                            },
                            None => {}
                        }
                    },
                    KeyCode::Enter => {
                        return Ok(Some(input_text));
                    },
                    KeyCode::Esc => {
                        return Ok(None);
                    },
                    _ => {}
                }
            },
            _ => {}
        }

        render::flush(stdout)?;
    }
}


pub fn get_string(
    request: &str,
    stdout: &mut Stdout,
    terminal_size: Size,
) -> Result<Option<String>, &'static str>
{
    let input_box_height: u16 = 3 + 2 * INPUT_BOX_VERTICAL_PADDING;
    let position = Position {
        x: 0,
        y: terminal_size.height() - input_box_height
    };
    let size = Size::new(terminal_size.width(), input_box_height);
    let rectangle = Rectangle { position, size };

    render::draw_input_box(stdout, rectangle)?;

    render::queue(
        stdout,
        cursor::MoveTo(1, rectangle.position.y)
    )?;

    render::queue(
        stdout,
        Print(request.to_uppercase())
    )?;

    render::queue(
        stdout,
        cursor::MoveTo(
            INPUT_BOX_HORIZONTAL_PADDING + 1,
            terminal_size.height() - INPUT_BOX_VERTICAL_PADDING - 2
        )
    )?;

    render::flush(stdout)?;

    Ok(capture_string(stdout, terminal_size)?)
}

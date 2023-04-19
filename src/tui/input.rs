use std::io::Stdout;

use crossterm::cursor;
use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEventKind;
use crossterm::event::read;
use crossterm::style::Print;
use crossterm::style::Attribute;
use crossterm::style::Color;

use crate::render;
use crate::tui::Position;
use crate::tui::Rectangle;
use crate::tui::Size;
use crate::tui::get_cursor_position;
use crate::error::StatusResult;


const INPUT_BOX_VERTICAL_PADDING: u16 = 0;
const INPUT_BOX_HORIZONTAL_PADDING: u16 = 1;


pub fn get_event() -> StatusResult<Event>
{
    match read()
    {
        Ok(event) => Ok(event),
        Err(_) => Err("couldn't read event")
    }
}


fn capture_string(stdout: &mut Stdout) -> StatusResult<Option<String>>
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
                            None => {},
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
                            }
                        }
                    },
                    KeyCode::Enter => {
                        return match input_text.trim()
                        {
                            "" => Ok(None),
                            trimmed_input => Ok(Some(
                                trimmed_input.to_string()
                            ))
                        }
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
    input_box_title: &str,
    request: &str,
    stdout: &mut Stdout,
    terminal_size: Size,
) -> StatusResult<Option<String>>
{
    let input_box_height: u16 = 3 + 2 * INPUT_BOX_VERTICAL_PADDING;
    let position = Position {
        x: 0,
        y: terminal_size.height() - input_box_height
    };
    let size = Size::new(terminal_size.width(), input_box_height);
    let rectangle = Rectangle { position, size };

    render::draw_input_box(stdout, rectangle)?;

    render::draw_text_at(
        stdout,
        input_box_title,
        Attribute::Reset,
        Color::Cyan,
        Color::Reset,
        2,
        rectangle.position.y
    )?;

    render::queue(
        stdout,
        cursor::MoveTo(
            INPUT_BOX_HORIZONTAL_PADDING + 1,
            terminal_size.height() - INPUT_BOX_VERTICAL_PADDING - 2
        )
    )?;

    render::draw_text(
        stdout,
        format!("{}: ", request).as_str(),
        Attribute::Bold,
        Color::Yellow,
        Color::Reset
    )?;

    render::flush(stdout)?;

    return capture_string(stdout)
}

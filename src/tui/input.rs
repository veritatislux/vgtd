use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEventKind;
use crossterm::event::read;
use crossterm::style::Attribute;
use crossterm::style::Color;

use crate::error::StatusResult;
use crate::render::Renderer;
use crate::render::draw_input_frame;
use crate::tui::Size;
use crate::tui::get_cursor_position;


pub fn get_event() -> StatusResult<Event>
{
    read().or(Err("couldn't read event"))
}


fn read_string(
    renderer: &mut Renderer,
    initial_text: String
) -> StatusResult<Option<String>>
{
    let mut input_text = initial_text;
    let mut cursor_position = get_cursor_position()?;

    renderer.show_cursor()?;
    renderer.flush()?;

    let result: StatusResult<Option<String>> = loop
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
                        renderer.print(character)?;

                        cursor_position.x += 1;
                    },
                    KeyCode::Backspace => {
                        match input_text.pop()
                        {
                            None => {},
                            Some(_) => {
                                cursor_position.x -= 1;

                                renderer.move_cursor_to(cursor_position)?;
                                renderer.print(' ')?;
                                renderer.move_cursor_to(cursor_position)?;
                            }
                        }
                    },
                    KeyCode::Enter => {
                        break Ok(match input_text.trim()
                        {
                            "" => None,
                            trimmed_input => Some(trimmed_input.to_string())
                        });
                    },
                    KeyCode::Esc => {
                        break Ok(None);
                    },
                    _ => {}
                }
            },
            _ => {}
        }

        renderer.flush()?;
    };

    renderer.hide_cursor()?;
    renderer.flush()?;

    result
}


pub fn get_string(
    renderer: &mut Renderer,
    input_box_title: &str,
    request: &str,
    terminal_size: Size,
) -> StatusResult<Option<String>>
{
    draw_input_frame(renderer, input_box_title, request, terminal_size)?;

    renderer.flush()?;

    let result = read_string(renderer, String::new());

    result
}


pub fn get_string_with_preview(
    renderer: &mut Renderer,
    input_box_title: &str,
    request: &str,
    preview_text: &str,
    terminal_size: Size,
) -> StatusResult<Option<String>>
{
    draw_input_frame(renderer, input_box_title, request, terminal_size)?;

    renderer.draw_text(
        preview_text,
        Attribute::Reset,
        Color::Reset,
        Color::Reset
    )?;

    renderer.flush()?;

    let result = read_string(renderer, preview_text.to_string());

    result
}

use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEventKind;
use crossterm::event::read;
use crossterm::style::Attribute;
use crossterm::style::Color;

use crate::error::StatusResult;
use crate::render::Renderer;
use crate::tui::Position;
use crate::tui::Rectangle;
use crate::tui::Size;
use crate::tui::get_cursor_position;


const INPUT_BOX_VERTICAL_PADDING: u16 = 0;
const INPUT_BOX_HORIZONTAL_PADDING: u16 = 1;


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

        renderer.flush()?;
    }
}


pub fn draw_input_frame(
    renderer: &mut Renderer,
    input_box_title: &str,
    request: &str,
    terminal_size: Size,
) -> StatusResult<()>
{
    let input_box_height: u16 = 3 + 2 * INPUT_BOX_VERTICAL_PADDING;
    let position = Position {
        x: 0,
        y: terminal_size.height() - input_box_height
    };
    let size = Size::new(terminal_size.width(), input_box_height);
    let rectangle = Rectangle { position, size };

    renderer.draw_input_box(rectangle)?;

    renderer.draw_text_at(
        input_box_title,
        Attribute::Reset,
        Color::Cyan,
        Color::Reset,
        Position { x: 2, y: rectangle.position.y }
    )?;

    renderer.move_cursor_to(
        Position {
            x: INPUT_BOX_HORIZONTAL_PADDING + 1,
            y: terminal_size.height() - INPUT_BOX_VERTICAL_PADDING - 2
        }
    )?;

    renderer.draw_text(
        format!("{}: ", request),
        Attribute::Bold,
        Color::Yellow,
        Color::Reset
    )?;

    Ok(())
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

    renderer.hide_cursor()?;
    renderer.flush()?;

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

    renderer.hide_cursor()?;
    renderer.flush()?;

    result
}

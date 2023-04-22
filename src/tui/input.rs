use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEventKind;
use crossterm::event::KeyModifiers;
use crossterm::event::read;
use crossterm::style::Attribute;
use crossterm::style::Color;

use crate::error::StatusResult;
use crate::render::Renderer;
use crate::render::draw_input_frame;
use crate::tui::Size;
use crate::tui::Position;
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

    let result: Option<String> = loop
    {
        if let Event::Key(event) = get_event()?
        {
            if event.kind != KeyEventKind::Press
            {
                continue;
            }

            match event.code
            {
                KeyCode::Char(character)
                if event.modifiers.contains(KeyModifiers::CONTROL) => {
                    match character
                    {
                        'u' => {
                            let input_length: u16 =
                                input_text.len().try_into().unwrap();

                            for x in
                                cursor_position.x - input_length
                                ..(cursor_position.x)
                            {
                                renderer.print_at(
                                    ' ',
                                    Position {
                                        x,
                                        y: cursor_position.y
                                    }
                                )?;
                            }

                            cursor_position.x -= input_length;
                            renderer.move_cursor_to(cursor_position)?;

                            input_text.clear();
                        },
                        _ => {}
                    }
                },
                KeyCode::Char(character) => {
                    cursor_position.x += 1;
                    renderer.print(character)?;
                    input_text.push(character);
                },
                KeyCode::Backspace => {
                    if let Some(_) = input_text.pop()
                    {
                        cursor_position.x -= 1;

                        renderer.print_at(' ', cursor_position)?;
                        renderer.move_cursor_to(cursor_position)?;
                    }
                },
                KeyCode::Enter => {
                    let trimmed_input = input_text.trim();

                    break if trimmed_input.is_empty()
                    {
                        None
                    }
                    else
                    {
                        Some(trimmed_input.to_string())
                    };
                },
                KeyCode::Esc => {
                    break None;
                },
                _ => {}
            }

            renderer.flush()?;
        }
    };

    renderer.hide_cursor()?;
    renderer.flush()?;

    Ok(result)
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

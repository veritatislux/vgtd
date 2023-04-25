use crossterm::event::Event;
use crossterm::event::KeyCode;
use crossterm::event::KeyEventKind;
use crossterm::event::KeyModifiers;
use crossterm::event::read;
use crossterm::style::Attribute;
use crossterm::style::Color;
use regex::Regex;
use lazy_static::lazy_static;

use crate::error::StatusResult;
use crate::render::Renderer;
use crate::render::draw_input_frame;
use crate::tui::Size;
use crate::tui::Position;
use crate::tui::get_cursor_position;


// Confirmation popup draft:
//
//
// ┌─Duplicate Task────────────────┐
// │                               │
// │  Would you like to create a   │
// │  new task instead of          │
// │  duplicating this one?        │
// │                               │
// │ [F] Accept         Refuse [J] │
// └───────────────────────────────┘


// List selection popup draft:
//
//
// ┌─Select an option──────┐
// │ opt2                  │
// ├───────────────────────┤
// │  Option 1             │
// │▶ Option 2             │
// │  Option 3             │
// │  Option 4             │
// │  Option 5             │
// └───────────────────────┘


pub fn get_event() -> StatusResult<Event>
{
    read().or(Err("couldn't read event"))
}


fn remove_from_input(
    amount: usize,
    renderer: &mut Renderer,
    input_text: &mut String,
    mut cursor_position: Position
) -> StatusResult<Position>
{
    let amount_u16: u16 = amount.try_into().unwrap();

    cursor_position.x -= amount_u16;

    for x in cursor_position.x .. cursor_position.x + amount_u16
    {
        renderer.print_at(' ', Position { x, ..cursor_position })?;
    }

    renderer.move_cursor_to(cursor_position)?;

    input_text.truncate(input_text.len() - amount);

    Ok(cursor_position)
}


fn clear_input(
    renderer: &mut Renderer,
    input_text: &mut String,
    cursor_position: Position
) -> StatusResult<Position>
{
    remove_from_input(input_text.len(), renderer, input_text, cursor_position)
}


fn delete_input_word(
    renderer: &mut Renderer,
    input_text: &mut String,
    cursor_position: Position
) -> StatusResult<Position>
{
    lazy_static! {
        static ref WORD_START_RE: Regex = Regex::new(r"\W\w+\s*$").unwrap();
    }

    match WORD_START_RE.find(input_text)
    {
        Some(re_match) => {
            remove_from_input(
                input_text.len() - re_match.start() - 1,
                renderer,
                input_text,
                cursor_position
            )
        },
        None => clear_input(renderer, input_text, cursor_position)
    }
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
                        'u' if !input_text.is_empty() => {
                            cursor_position = clear_input(
                                renderer,
                                &mut input_text,
                                cursor_position
                            )?;
                        },
                        'w' if !input_text.is_empty() => {
                            cursor_position = delete_input_word(
                                renderer,
                                &mut input_text,
                                cursor_position
                            )?;
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

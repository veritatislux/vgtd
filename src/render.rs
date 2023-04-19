use std::io::Stdout;
use std::io::Write;

use crossterm::Command;
use crossterm::ExecutableCommand;
use crossterm::QueueableCommand;
use crossterm::cursor;
use crossterm::style::Attribute;
use crossterm::style::Color;
use crossterm::style::Print;
use crossterm::style::ResetColor;
use crossterm::style::SetAttribute;
use crossterm::style::SetBackgroundColor;
use crossterm::style::SetForegroundColor;

use crate::error::StatusResult;
use crate::gtd::List;
use crate::gtd::ListItem;
use crate::tui::Rectangle;


const BOX_BOTTOM_LEFT_CHAR: char = '┗';
const BOX_BOTTOM_RIGHT_CHAR: char = '┛';
const BOX_HORIZONTAL_CHAR: char = '━';
const BOX_TOP_LEFT_CHAR: char = '┏';
const BOX_TOP_RIGHT_CHAR: char = '┓';
const BOX_VERTICAL_CHAR: char = '┃';
const EMPTY_SPACE: char = ' ';


pub fn queue(stdout: &mut Stdout, command: impl Command) -> StatusResult<()>
{
    if let Err(_) = stdout.queue(command)
    {
        return Err("couldn't queue command");
    }

    Ok(())
}


pub fn execute(stdout: &mut Stdout, command: impl Command) -> StatusResult<()>
{
    if let Err(_) = stdout.execute(command)
    {
        return Err("couldn't execute command");
    }

    Ok(())
}


pub fn flush(stdout: &mut Stdout) -> StatusResult<()>
{
    if let Err(_) = stdout.flush()
    {
        return Err("couldn't flush to stdout properly");
    }

    Ok(())
}


fn make_bordered_line(
    width: u16,
    left_char: char,
    fill_char: char,
    right_char: char
) -> String
{
    let mut bordered_line = String::with_capacity(width.into());

    bordered_line.push(left_char);

    for _ in 0..(width - 2)
    {
        bordered_line.push(fill_char);
    }

    bordered_line.push(right_char);

    bordered_line
}


fn make_top_line(width: u16) -> String
{
    make_bordered_line(
        width,
        BOX_TOP_LEFT_CHAR,
        BOX_HORIZONTAL_CHAR,
        BOX_TOP_RIGHT_CHAR
    )
}


fn make_middle_line(width: u16) -> String
{
    make_bordered_line(
        width,
        BOX_VERTICAL_CHAR,
        EMPTY_SPACE,
        BOX_VERTICAL_CHAR
    )
}


fn make_bottom_line(width: u16) -> String
{
    make_bordered_line(
        width,
        BOX_BOTTOM_LEFT_CHAR,
        BOX_HORIZONTAL_CHAR,
        BOX_BOTTOM_RIGHT_CHAR
    )
}


pub fn draw_box(stdout: &mut Stdout, rectangle: Rectangle) -> StatusResult<()>
{
    queue(
        stdout,
        cursor::MoveTo(rectangle.position.x, rectangle.position.y)
    )?;

    let mut box_str = make_top_line(rectangle.size.width());

    box_str.push('\n');

    for _ in 0..(rectangle.size.height() - 2)
    {
        box_str.push_str(make_middle_line(rectangle.size.width()).as_str());
        box_str.push('\n');
    }

    box_str.push_str(make_bottom_line(rectangle.size.width()).as_str());

    queue(stdout, Print(box_str))?;

    Ok(())
}


pub fn draw_text(
    stdout: &mut Stdout,
    text: &str,
    attribute: Attribute,
    foreground_color: Color,
    background_color: Color,
) -> StatusResult<()>
{
    queue(stdout, SetAttribute(attribute))?;
    queue(stdout, SetForegroundColor(foreground_color))?;
    queue(stdout, SetBackgroundColor(background_color))?;
    queue(stdout, Print(text))?;
    queue(stdout, ResetColor)?;
    queue(stdout, SetAttribute(Attribute::Reset))?;

    Ok(())
}


pub fn draw_text_at(
    stdout: &mut Stdout,
    text: &str,
    attribute: Attribute,
    foreground_color: Color,
    background_color: Color,
    x: u16,
    y: u16
) -> StatusResult<()>
{
    queue(stdout, cursor::MoveTo(x, y))?;
    draw_text(stdout, text, attribute, foreground_color, background_color)?;

    Ok(())
}


pub fn draw_title(
    stdout: &mut Stdout,
    title: &str,
    rectangle: Rectangle
) -> StatusResult<()>
{
    draw_text_at(
        stdout,
        format!(" {} ", title.to_uppercase()).as_str(),
        Attribute::Bold,
        Color::Black,
        Color::Green,
        rectangle.position.x + 4,
        rectangle.position.y + 2
    )?;

    Ok(())
}


pub fn draw_item_contexts(
    stdout: &mut Stdout,
    contexts: &Vec::<String>
) -> StatusResult<()>
{
    for context in contexts
    {
        queue(stdout, Print("  "))?;

        draw_text(
            stdout,
            format!("@{}", context).as_str(),
            Attribute::Bold,
            Color::Magenta,
            Color::Black
        )?;
    }

    Ok(())
}


pub fn draw_item(stdout: &mut Stdout, item: &ListItem) -> StatusResult<()>
{
    queue(stdout, Print(format!("* {}", item.message)))?;

    if item.contexts().len() > 0
    {
        draw_item_contexts(stdout, item.contexts())?;
    }

    Ok(())
}


pub fn draw_selected_item(
    stdout: &mut Stdout,
    item: &ListItem
) -> StatusResult<()>
{
    draw_text(
        stdout,
        format!("> {}", item.message).as_str(),
        Attribute::Bold,
        Color::Cyan,
        Color::Reset
    )?;

    if item.contexts().len() > 0
    {
        draw_item_contexts(stdout, item.contexts())?;
    }

    Ok(())
}


pub fn draw_items(
    stdout: &mut Stdout,
    items: &Vec<ListItem>,
    rectangle: Rectangle,
    selected_item: usize
) -> StatusResult<()>
{
    for (index, item) in items.iter().enumerate()
    {
        let y_offset: u16 = index.try_into().unwrap();

        queue(
            stdout,
            cursor::MoveTo(
                4 + rectangle.position.x,
                4 + rectangle.position.y + y_offset
            )
        )?;

        if usize::from(y_offset) == selected_item
        {
            draw_selected_item(stdout, item)?;
        }
        else
        {
            draw_item(stdout, item)?;
        }
    }

    Ok(())
}


pub fn draw_list(
    stdout: &mut Stdout,
    list: &List,
    rectangle: Rectangle,
    selected_item: usize
) -> StatusResult<()>
{
    draw_box(stdout, rectangle)?;
    draw_title(stdout, &list.name, rectangle)?;

    if list.items().len() > 0
    {
        draw_items(stdout, &list.items(), rectangle, selected_item)?;
    }

    Ok(())
}


pub fn draw_input_box(
    stdout: &mut Stdout,
    rectangle: Rectangle
) -> StatusResult<()>
{
    draw_box(stdout, rectangle)?;

    Ok(())
}

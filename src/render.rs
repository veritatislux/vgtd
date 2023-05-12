use std::io::Stdout;
use std::io::stdout;
use std::io::Write;
use std::fmt::Display;

use crossterm::Command;
use crossterm::ExecutableCommand;
use crossterm::QueueableCommand;
use crossterm::cursor;
use crossterm::terminal;
use crossterm::style::Attribute;
use crossterm::style::Color;
use crossterm::style::Print;
use crossterm::style::ResetColor;
use crossterm::style::SetAttribute;
use crossterm::style::SetBackgroundColor;
use crossterm::style::SetForegroundColor;

use crate::error::StatusResult;
use crate::gtd::List;
use crate::gtd::Task;
use crate::tui::Rectangle;
use crate::tui::Position;
use crate::tui::Size;
use crate::tui::get_terminal_size;


// TODO: Make a struct that has these values as fields so different boxes can
// be drawn using different characters instead of having to change constants
// all the time.
const BOX_BOTTOM_LEFT_CHAR: char = '┗';
const BOX_BOTTOM_RIGHT_CHAR: char = '┛';
const BOX_HORIZONTAL_CHAR: char = '━';
const BOX_TOP_LEFT_CHAR: char = '┏';
const BOX_TOP_RIGHT_CHAR: char = '┓';
const BOX_VERTICAL_CHAR: char = '┃';
const EMPTY_SPACE: char = ' ';
const INPUT_BOX_VERTICAL_PADDING: u16 = 0;
const INPUT_BOX_HORIZONTAL_PADDING: u16 = 1;


pub struct Renderer
{
    stdout_handle: Stdout,
}


impl Renderer
{
    pub fn new() -> Self
    {
        Self
        {
            stdout_handle: stdout(),
        }
    }

    pub fn stdout_handle(&mut self) -> &mut Stdout
    {
        &mut self.stdout_handle
    }

    pub fn queue(&mut self, command: impl Command) -> StatusResult<()>
    {
        match self.stdout_handle().queue(command)
        {
            Ok(_) => Ok(()),
            Err(_) => Err("couldn't queue command")
        }
    }

    pub fn execute(&mut self, command: impl Command) -> StatusResult<()>
    {
        match self.stdout_handle().execute(command)
        {
            Ok(_) => Ok(()),
            Err(_) => Err("couldn't execute command")
        }
    }

    pub fn flush(&mut self) -> StatusResult<()>
    {
        match self.stdout_handle().flush()
        {
            Ok(_) => Ok(()),
            Err(_) => Err("couldn't flush to stdout")
        }
    }

    pub fn enter_alternate_screen(&mut self) -> StatusResult<()>
    {
        self.execute(terminal::EnterAlternateScreen)
    }

    pub fn leave_alternate_screen(&mut self) -> StatusResult<()>
    {
        self.execute(terminal::LeaveAlternateScreen)
    }

    pub fn move_cursor_to(&mut self, position: Position) -> StatusResult<()>
    {
        self.queue(cursor::MoveTo(position.x, position.y))
    }

    pub fn clear(&mut self) -> StatusResult<()>
    {
        self.queue(terminal::Clear(terminal::ClearType::All))
    }

    pub fn purge(&mut self) -> StatusResult<()>
    {
        self.queue(terminal::Clear(terminal::ClearType::Purge))
    }

    pub fn clear_current_line(&mut self) -> StatusResult<()>
    {
        self.queue(terminal::Clear(terminal::ClearType::CurrentLine))
    }

    pub fn clear_downwards(&mut self) -> StatusResult<()>
    {
        self.queue(terminal::Clear(terminal::ClearType::FromCursorDown))
    }

    pub fn clear_upwards(&mut self) -> StatusResult<()>
    {
        self.queue(terminal::Clear(terminal::ClearType::FromCursorUp))
    }

    pub fn clear_to_end_of_line(&mut self) -> StatusResult<()>
    {
        self.queue(terminal::Clear(terminal::ClearType::UntilNewLine))
    }

    pub fn print<T: Display>(&mut self, text: T) -> StatusResult<()>
    {
        self.queue(Print(text))
    }

    pub fn print_at<T: Display>(
        &mut self,
        text: T,
        position: Position
    ) -> StatusResult<()>
    {
        self.move_cursor_to(position)?;
        self.print(text)?;

        Ok(())
    }

    pub fn hide_cursor(&mut self) -> StatusResult<()>
    {
        self.queue(cursor::Hide)
    }

    pub fn show_cursor(&mut self) -> StatusResult<()>
    {
        self.queue(cursor::Show)
    }

    pub fn make_line(width: u16, character: char) -> String
    {
        let mut line = String::with_capacity(width.into());

        for _ in 0..width
        {
            line.push(character);
        }

        line
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
        Self::make_bordered_line(
            width,
            BOX_TOP_LEFT_CHAR,
            BOX_HORIZONTAL_CHAR,
            BOX_TOP_RIGHT_CHAR
        )
    }

    fn make_middle_line(width: u16) -> String
    {
        Self::make_bordered_line(
            width,
            BOX_VERTICAL_CHAR,
            EMPTY_SPACE,
            BOX_VERTICAL_CHAR
        )
    }

    fn make_bottom_line(width: u16) -> String
    {
        Self::make_bordered_line(
            width,
            BOX_BOTTOM_LEFT_CHAR,
            BOX_HORIZONTAL_CHAR,
            BOX_BOTTOM_RIGHT_CHAR
        )
    }

    pub fn draw_box(&mut self, rectangle: Rectangle) -> StatusResult<()>
    {
        self.move_cursor_to(rectangle.position)?;

        let mut box_str = Self::make_top_line(rectangle.size.width());

        box_str.push('\n');

        for _ in 0..(rectangle.size.height() - 2)
        {
            box_str.push_str(
                &Self::make_middle_line(rectangle.size.width())
            );
            box_str.push('\n');
        }

        box_str.push_str(
            &Self::make_bottom_line(rectangle.size.width())
        );

        self.print(box_str)?;

        Ok(())
    }

    pub fn draw_text<T: Display>(
        &mut self,
        text: T,
        attribute: Attribute,
        foreground_color: Color,
        background_color: Color,
    ) -> StatusResult<()>
    {
        // Can I make it better with macros, perhaps?
        self.queue(SetAttribute(attribute))?;
        self.queue(SetForegroundColor(foreground_color))?;
        self.queue(SetBackgroundColor(background_color))?;
        self.queue(Print(text))?;
        self.queue(ResetColor)?;
        self.queue(SetAttribute(Attribute::Reset))?;

        Ok(())
    }

    pub fn draw_text_at<T: Display>(
        &mut self,
        text: T,
        attribute: Attribute,
        foreground_color: Color,
        background_color: Color,
        position: Position
    ) -> StatusResult<()>
    {
        self.move_cursor_to(position)?;
        self.draw_text(text, attribute, foreground_color, background_color)?;

        Ok(())
    }

    pub fn draw_title<T: Display>(
        &mut self,
        title: T,
        rectangle: Rectangle
    ) -> StatusResult<()>
    {
        self.draw_text_at(
            format!(" {} ", title),
            Attribute::Bold,
            Color::Black,
            Color::Green,
            Position {
                x: rectangle.position.x + 4,
                y: rectangle.position.y + 2
            }
        )?;

        Ok(())
    }

    pub fn draw_task_contexts(
        &mut self,
        contexts: &Vec::<String>
    ) -> StatusResult<()>
    {
        for context in contexts
        {
            self.print("  ")?;

            self.draw_text(
                format!("@{}", context),
                Attribute::Bold,
                Color::Magenta,
                Color::Black
            )?;
        }

        Ok(())
    }

    pub fn draw_task(&mut self, task: &Task) -> StatusResult<()>
    {
        self.print(format!("* {}", task.message))?;

        if task.contexts().len() > 0
        {
            self.draw_task_contexts(task.contexts())?;
        }

        Ok(())
    }

    pub fn draw_selected_task(&mut self, task: &Task) -> StatusResult<()>
    {
        self.draw_text(
            format!("> {}", task.message),
            Attribute::Bold,
            Color::Cyan,
            Color::Reset
        )?;

        if task.contexts().len() > 0
        {
            self.draw_task_contexts(task.contexts())?;
        }

        Ok(())
    }

    pub fn draw_tasks(
        &mut self,
        tasks: &Vec<Task>,
        rectangle: Rectangle,
        selected_task: usize
    ) -> StatusResult<()>
    {
        for (index, task) in tasks.iter().enumerate()
        {
            let y_offset: u16 = index.try_into().unwrap();

            self.move_cursor_to(
                Position {
                    x: rectangle.position.x + 4,
                    y: rectangle.position.y + 4 + y_offset
                }
            )?;

            if usize::from(y_offset) == selected_task
            {
                self.draw_selected_task(task)?;
            }
            else
            {
                self.draw_task(task)?;
            }
        }

        Ok(())
    }

    pub fn draw_list(
        &mut self,
        list: &List,
        rectangle: Rectangle,
        selected_task: usize
    ) -> StatusResult<()>
    {
        self.draw_box(rectangle)?;
        self.draw_title(&list.name, rectangle)?;

        if list.tasks().len() > 0
        {
            self.draw_tasks(&list.tasks(), rectangle, selected_task)?;
        }

        Ok(())
    }

    pub fn draw_input_frame(
        &mut self,
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

        self.draw_box(rectangle)?;

        self.draw_text_at(
            input_box_title,
            Attribute::Reset,
            Color::Cyan,
            Color::Reset,
            Position { x: 2, y: rectangle.position.y }
        )?;

        self.move_cursor_to(
            Position {
                x: INPUT_BOX_HORIZONTAL_PADDING + 1,
                y: terminal_size.height() - INPUT_BOX_VERTICAL_PADDING - 2
            }
        )?;

        self.draw_text(
            format!("{}: ", request),
            Attribute::Bold,
            Color::Yellow,
            Color::Reset
        )?;

        Ok(())
    }

    pub fn draw_bottom_pad(&mut self, size: u16) -> StatusResult<()>
    {
        let terminal_size = get_terminal_size()?;

        let pad_position = Position {
            x: 0,
            y: terminal_size.height() - size - 2
        };

        self.move_cursor_to(pad_position)?;

        self.print(
            Self::make_line(
                terminal_size.width(),
                BOX_HORIZONTAL_CHAR
            )
        )?;

        self.move_cursor_to(
            Position {
                y: pad_position.y + 1,
                ..pad_position
            }
        )?;

        self.queue(terminal::Clear(terminal::ClearType::CurrentLine))?;

        self.move_cursor_to(
            Position {
                y: pad_position.y + 2,
                ..pad_position
            }
        )?;

        self.print(
            Self::make_line(
                terminal_size.width(),
                BOX_HORIZONTAL_CHAR
            )
        )?;

        self.move_cursor_to(
            Position {
                x: pad_position.x + 2,
                y: pad_position.y + 1
            }
        )?;

        Ok(())
    }

    pub fn draw_notification(&mut self, message: &str) -> StatusResult<()>
    {
        self.draw_bottom_pad(1)?;
        self.print(message)?;

        Ok(())
    }
}

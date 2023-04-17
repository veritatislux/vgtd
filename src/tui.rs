pub mod containers;
pub mod input;

use crossterm::cursor;
use crossterm::terminal;


#[derive(Copy)]
pub struct Position
{
    pub x: u16,
    pub y: u16,
}


impl Clone for Position
{
    fn clone(&self) -> Self
    {
        Self {
            x: self.x,
            y: self.y
        }
    }
}


#[derive(Copy)]
pub struct Size
{
    width: u16,
    height: u16,
}


impl Size
{
    pub fn new(width: u16, height: u16) -> Self
    {
        let mut instance = Self { width: 0, height: 0 };

        instance.set_width(width);
        instance.set_height(height);

        instance
    }


    pub fn width(&self) -> u16 { self.width }


    pub fn set_width(&mut self, value: u16)
    {
        if value < 1
        {
            panic!("Width has to be greater than or equal to 1.");
        }

        self.width = value;
    }


    pub fn height(&self) -> u16 { self.height }


    pub fn set_height(&mut self, value: u16)
    {
        if value < 1
        {
            panic!("Height has to be greater than or equal to 1.");
        }

        self.height = value;
    }


    pub fn clone(&self) -> Self
    {
        Self {
            width: self.width,
            height: self.height
        }
    }
}


impl Clone for Size
{
    fn clone(&self) -> Self
    {
        Self {
            width: self.width,
            height: self.height
        }
    }
}


#[derive(Copy)]
pub struct Rectangle
{
    pub position: Position,
    pub size: Size,
}


impl Clone for Rectangle
{
    fn clone(&self) -> Self
    {
        Self {
            position: self.position,
            size: self.size
        }
    }
}


pub fn get_cursor_position() -> Result<Position, &'static str>
{
    let (x, y) = match cursor::position()
    {
        Err(_) => return Err("couldn't fetch terminal position"),
        Ok(value) => value
    };

    Ok(Position { x, y })
}


pub fn get_terminal_size() -> Result<Size, &'static str>
{
    let (terminal_width, terminal_height) = match terminal::size()
    {
        Err(_) => return Err("couldn't fetch terminal size"),
        Ok(value) => value
    };

    Ok(Size::new(terminal_width, terminal_height))
}

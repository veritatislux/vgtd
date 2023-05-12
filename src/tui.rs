pub mod containers;
pub mod input;

use crossterm::cursor;
use crossterm::terminal;

use crate::error::StatusResult;


pub type Coord = u16;
pub type CoordOffset = i16;


fn offset_axis(value: Coord, offset: CoordOffset) -> u16
{
    u16::try_from(i32::from(value) + i32::from(offset)).unwrap()
}


#[derive(Copy)]
pub struct Offset
{
    pub x: CoordOffset,
    pub y: CoordOffset
}


impl Clone for Offset
{
    fn clone(&self) -> Self
    {
        Self { ..*self }
    }
}


impl Offset
{
    pub fn new(x: CoordOffset, y: CoordOffset) -> Self
    {
        Self { x, y }
    }

    pub fn new_zero() -> Self
    {
        Self::new(0, 0)
    }
}


#[derive(Copy)]
pub struct Position
{
    pub x: Coord,
    pub y: Coord,
}


impl Clone for Position
{
    fn clone(&self) -> Self
    {
        Self { ..*self }
    }
}


impl Position
{
    pub fn new(x: Coord, y: Coord) -> Self
    {
        Self { x, y }
    }

    pub fn new_zero() -> Self
    {
        Self::new(0, 0)
    }

    pub fn offset_by(&mut self, offset: Offset) -> Self
    {
        self.x = offset_axis(self.x, offset.x);
        self.y = offset_axis(self.y, offset.y);

        *self
    }
}


#[derive(Copy)]
pub struct Size
{
    width: Coord,
    height: Coord,
}


impl Size
{
    pub fn new(width: Coord, height: Coord) -> Self
    {
        let mut instance = Self { width: 0, height: 0 };

        instance.set_width(width);
        instance.set_height(height);

        instance
    }


    pub fn width(&self) -> Coord { self.width }


    pub fn set_width(&mut self, value: Coord)
    {
        if value < 1
        {
            panic!("Width has to be greater than or equal to 1.");
        }

        self.width = value;
    }


    pub fn height(&self) -> Coord { self.height }


    pub fn set_height(&mut self, value: Coord)
    {
        if value < 1
        {
            panic!("Height has to be greater than or equal to 1.");
        }

        self.height = value;
    }


    pub fn clone(&self) -> Self
    {
        Self { ..*self }
    }
}


impl Clone for Size
{
    fn clone(&self) -> Self
    {
        Self { ..*self }
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
        Self { ..*self }
    }
}


pub fn get_cursor_position() -> StatusResult<Position>
{
    let (x, y) = match cursor::position()
    {
        Err(_) => return Err("couldn't fetch terminal position"),
        Ok(value) => value
    };

    Ok(Position { x, y })
}


pub fn get_terminal_size() -> StatusResult<Size>
{
    let (terminal_width, terminal_height) = match terminal::size()
    {
        Err(_) => return Err("couldn't fetch terminal size"),
        Ok(value) => value
    };

    Ok(Size::new(terminal_width, terminal_height))
}


pub trait VisualItem
{
    fn offset(&self) -> Offset;

    fn set_offset(&mut self, new_offset: Offset);

    fn children(&self) -> &Vec<Box<dyn VisualItem>>;

    fn children_mut(&mut self) -> &mut Vec<Box<dyn VisualItem>>;

    fn add_child(&mut self, new_child: Box<dyn VisualItem>)
    {
        self.children_mut().push(new_child);
    }

    fn draw_self(&self, position: Position);

    fn draw_children(&self, position: Position)
    {
        for child in self.children()
        {
            child.draw(position)
        }
    }

    fn draw(&self, mut position: Position)
    {
        let item_position = position.offset_by(self.offset());

        self.draw_self(item_position);
        self.draw_children(item_position);
    }
}


pub struct VisualContainer
{
    offset: Offset,
    children: Vec<Box<dyn VisualItem>>,
}


impl VisualContainer
{
    pub fn new() -> Self
    {
        Self {
            offset: Offset::new_zero(),
            children: vec![]
        }
    }
}


impl VisualItem for VisualContainer
{
    fn offset(&self) -> Offset
    {
        self.offset
    }

    fn set_offset(&mut self, new_offset: Offset)
    {
        self.offset = new_offset;
    }

    fn children(&self) -> &Vec<Box<dyn VisualItem>>
    {
        &self.children
    }

    fn children_mut(&mut self) -> &mut Vec<Box<dyn VisualItem>>
    {
        &mut self.children
    }

    fn draw_self(&self, _position: Position) {}
}

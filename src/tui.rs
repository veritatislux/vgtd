use crate::gtd::List;

pub mod containers;


pub struct Position
{
    pub x: u16,
    pub y: u16,
}


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
}


pub struct Rectangle
{
    pub position: Position,
    pub size: Size,
}


pub struct ListView
{
    pub list: List,
    pub rectangle: Rectangle,
}


impl ListView
{
    pub fn new(
        list: List,
        x: u16,
        y: u16,
        width: u16,
        height: u16
    ) -> Self
    {
        let position = Position { x, y };
        let size = Size::new(width, height);
        let rectangle = Rectangle { position, size };

        ListView { list, rectangle }
    }
}

use crate::tui;


struct VBox
{
    subviews: Vec<tui::ListView>,
    rectangle: tui::Rectangle
}


impl VBox
{
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self
    {
        let subviews: Vec<tui::ListView> = vec![];

        let position = tui::Position { x, y };
        let size = tui::Size::new(width, height);
        let rectangle = tui::Rectangle { position, size };

        VBox { subviews, rectangle }
    }


    fn adjust_subviews(&mut self)
    {
    }


    pub fn get_subviews(&self) -> &Vec<tui::ListView>
    {
        &self.subviews
    }


    pub fn add_subview(&mut self, list_view: tui::ListView)
    {
        self.subviews.push(list_view);
    }


    pub fn draw(&self) -> Result<(), &'static str>
    {
        Ok(())
    }
}

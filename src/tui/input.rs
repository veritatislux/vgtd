use std::io::Stdout;

use crate::render;
use crate::tui::Rectangle;
use crate::tui::Size;
use crate::tui::Position;


const INPUT_BOX_VERTICAL_PADDING: u16 = 1;
const INPUT_BOX_HORIZONTAL_PADDING: u16 = 1;


pub fn get_string(
    request: &str,
    stdout: &mut Stdout,
    terminal_size: Size,
) -> Result<Option<String>, &'static str>
{
    let input_text = String::new();

    let input_box_height: u16 = 3 + 2 * INPUT_BOX_VERTICAL_PADDING;
    let position = Position {
        x: 0,
        y: terminal_size.height() - input_box_height
    };
    let size = Size::new(terminal_size.width(), input_box_height);
    let rectangle = Rectangle { position, size };

    render::draw_input_box(stdout, rectangle)?;

    render::flush(stdout)?;

    std::thread::sleep(std::time::Duration::from_secs(5));

    Ok(Some(input_text))
}

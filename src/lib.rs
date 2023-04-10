pub mod gtd;
pub mod tui;
pub mod render;
pub mod error;

use std::io::stdout;
use std::io::Write;

use crossterm::ExecutableCommand;


pub fn run() -> Result<(), &'static str>
{
    let mut list = gtd::List::new("example list".to_string());

    let mut item1 = gtd::ListItem::new("build a map".to_string());
    let mut item2 = gtd::ListItem::new("tell a story".to_string());
    let mut item3 = gtd::ListItem::new("read a book".to_string());
    let mut item4 = gtd::ListItem::new("help someone".to_string());
    let mut item5 = gtd::ListItem::new("unexplode creepers".to_string());
    let mut item6 = gtd::ListItem::new("unlearn javascript".to_string());

    item1
        .add_context("cartography lounge".to_string());
    item2
        .add_context("home".to_string())
        .add_context("library".to_string())
        .add_context("university".to_string());
    item3
        .add_context("home".to_string());
    item4
        .add_context("everywhere".to_string());
    item5
        .add_context("minecraft".to_string());
    item6
        .add_context("everywhere".to_string());

    list
        .push_item(item1)
        .push_item(item2)
        .push_item(item3)
        .push_item(item4)
        .push_item(item5)
        .push_item(item6);

    let mut stdout = stdout();

    if let Err(_) = stdout.execute(crossterm::terminal::EnterAlternateScreen)
    {
        return Err("couldn't enter alternate screen");
    }

    let (terminal_width, terminal_height) = match crossterm::terminal::size()
    {
        Err(_) => return Err("couldn't fetch console size"),
        Ok(value) => value
    };

    let list_view = tui::ListView::new(
        list,
        0,
        0,
        terminal_width,
        terminal_height
    );

    render::draw_list_view(&mut stdout, &list_view)?;

    if let Err(_) = stdout.flush()
    {
        return Err("couldn't flush to stdout properly");
    }

    std::thread::sleep(std::time::Duration::from_secs(30));

    if let Err(_) = stdout.execute(
        crossterm::terminal::LeaveAlternateScreen
    ) {
        return Err("couldn't leave alternate screen");
    }

    Ok(())
}

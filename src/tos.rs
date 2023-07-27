// Terminal Output System

use crate::text::Formattable;

use colored::Color;
use colored::Colorize;

const PADDING_CHAR: char = ' ';
const BASE_PADDING_LEVEL: usize = 2;
const BLOCK_PREFIX: &str = "\n";
const BLOCK_POSTFIX: &str = "\n";
const PREFIX: &str = "[vGTD]";
pub const COLOR_PREFIX: Color = Color::BrightGreen;
pub const COLOR_SUCCESS: Color = Color::BrightGreen;
pub const COLOR_ERROR: Color = Color::BrightRed;
pub const COLOR_INFO: Color = Color::BrightYellow;
pub const COLOR_NUM_VALUE: Color = Color::BrightGreen;
pub const COLOR_TITLE: Color = Color::BrightYellow;
pub const COLOR_GROUP: Color = Color::BrightBlue;
pub const COLOR_IDENTIFIER: Color = Color::BrightMagenta;

pub fn get_padding_length(level: usize) -> usize
{
    BASE_PADDING_LEVEL + level * 2
}

pub fn get_padding(level: usize) -> String
{
    PADDING_CHAR.to_string().repeat(get_padding_length(level))
}

pub fn format_list_name(name: &str) -> String
{
    format!("{}", name.to_titlecase().color(COLOR_IDENTIFIER))
}

pub fn format_project_name(name: &str) -> String
{
    format!("{}", name.to_titlecase().color(COLOR_IDENTIFIER))
}

pub fn format_task_name(name: &str) -> String
{
    format!("{}", name.to_titlecase().color(COLOR_IDENTIFIER))
}

pub fn format_section_name(name: &str) -> String
{
    format!("{}", name.to_titlecase().color(COLOR_GROUP).bold())
}

pub struct OutputBlock
{
    text: String,
}

impl OutputBlock
{
    pub fn new() -> Self
    {
        Self {
            text: String::new(),
        }
    }

    pub fn insert_line(
        &mut self,
        line: &str,
        padding_level: usize,
    ) -> &mut Self
    {
        self.text
            .reserve(line.len() + get_padding_length(padding_level));
        self.text.push_str(&get_padding(padding_level));
        self.text.push_str(line);
        self.text.push('\n');

        self
    }

    pub fn insert_text(&mut self, text: &str) -> &mut Self
    {
        self.text.push_str(text);

        self
    }

    pub fn send(&self) -> ()
    {
        println!("{}{}{}", BLOCK_PREFIX, &self.text.trim_end(), BLOCK_POSTFIX,);
    }
}

// TODO: Reduce code repetition between these `send*` functions
pub fn send_info(message: &str) -> ()
{
    OutputBlock::new()
        .insert_line(&format!("{} {}", PREFIX.color(COLOR_INFO), message), 0)
        .send()
}

pub fn send_error(message: &str) -> ()
{
    OutputBlock::new()
        .insert_line(
            &format!("{} Error: {}", PREFIX.color(COLOR_ERROR), message),
            0,
        )
        .send()
}

pub fn send_success(message: &str) -> ()
{
    OutputBlock::new()
        .insert_line(
            &format!("{} {}", PREFIX.color(COLOR_SUCCESS), message),
            0,
        )
        .send()
}

use crossterm::{
    cursor, queue,
    style::{self, Stylize},
    terminal,
};

use crate::renderer;

pub struct Map {
    top: String,
    buttom: String,
    left: String,
    right: String,
    corner: String,
}

impl Map {
    pub fn new(top: &str, buttom: &str, left: &str, right: &str, corner: &str) -> Self {
        Self {
            top: top.to_string(),
            buttom: buttom.to_string(),
            left: left.to_string(),
            right: right.to_string(),
            corner: corner.to_string(),
        }
    }
}

impl<'a> renderer::Render<'a> for Map {
    fn render(&self, stdout: &mut std::io::Stdout) -> crossterm::Result<()> {
        let (x_max, y_max) = terminal::size()?;
        queue!(
            stdout,
            cursor::MoveTo(1, 0),
            style::PrintStyledContent(self.top.repeat(x_max as usize - 2).as_str().on_dark_green()),
            cursor::MoveTo(1, y_max),
            style::PrintStyledContent(
                self.buttom
                    .repeat(x_max as usize - 2)
                    .as_str()
                    .on_dark_green()
            ),
            cursor::MoveTo(0, 0),
            style::PrintStyledContent(self.corner.as_str().on_dark_green()),
            cursor::MoveTo(0, y_max),
            style::PrintStyledContent(self.corner.as_str().on_dark_green()),
            cursor::MoveTo(x_max, 0),
            style::PrintStyledContent(self.corner.as_str().on_dark_green()),
            cursor::MoveTo(x_max, y_max),
            style::PrintStyledContent(self.corner.as_str().on_dark_green()),
        )?;

        for i in 1..(y_max - 1) {
            queue!(
                stdout,
                cursor::MoveTo(0, i),
                style::PrintStyledContent(self.left.as_str().on_dark_green()),
                cursor::MoveTo(x_max, i),
                style::PrintStyledContent(self.right.as_str().on_dark_green()),
            )?;
        }
        Ok(())
    }
}

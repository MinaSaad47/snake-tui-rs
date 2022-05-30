use std::io::Stdout;

use crossterm::{
    cursor, queue,
    style::{self, Stylize},
};

use crate::{math::Vec2, renderer};

pub struct Food {
    pub pos: Vec2,
    pub icon: String,
}

impl Food {
    pub fn new(icon: &str, pos: &Vec2) -> Self {
        Self {
            icon: icon.to_string(),
            pos: *pos,
        }
    }
}

impl renderer::Display for Food {
    fn display(&self, stdout: &mut Stdout) -> crossterm::Result<()> {
        queue!(
            stdout,
            cursor::MoveTo(self.pos.x as u16, self.pos.y as u16),
            style::PrintStyledContent(self.icon.as_str().red())
        )?;
        Ok(())
    }
}

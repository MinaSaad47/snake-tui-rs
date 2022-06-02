use std::{
    io::{self, Stdout, Write},
    ops,
};

use crossterm::{terminal::{self, ClearType}, style::{StyledContent, self}, queue};
use crossterm::execute;
use crossterm::cursor;

use crate::math::Vec2;

pub trait Render<'a> {
    fn render(&'a self) -> Box<dyn std::iter::Iterator<Item = (&'a Vec2, &'a StyledContent<String>)> + 'a>;
}

pub struct Renderer {
    stdout: Stdout,
}

impl Renderer {
    pub fn new() -> Self {
        let mut stdout = io::stdout();

        terminal::enable_raw_mode().unwrap();

        execute!(
            stdout,
            terminal::EnterAlternateScreen,
            cursor::Hide,
            terminal::SetTitle("Snake Game")
        )
        .unwrap();

        Self { stdout }
    }
    pub fn render<'a>(&mut self, objects: &[&'a (dyn Render<'a> + 'a)]) -> crossterm::Result<()> {
        for obj in objects.iter() {
            for (pos, c) in obj.render() {
                queue!(self.stdout,
                    cursor::MoveTo(pos.x as u16, pos.y as u16),
                    style::PrintStyledContent(c.clone())
                )?;
            }
        }
        self.stdout.flush()?;
        Ok(())
    }
    pub fn clear<'a>(&mut self, objects: &[&'a (dyn Render<'a> + 'a)]) -> crossterm::Result<()> {
        for obj in objects {
            for (pos, _) in obj.render() {
                queue!(self.stdout,
                    cursor::MoveTo(pos.x as u16, pos.y as u16),
                    style::Print(" "),
                )?;
            }
        }
        self.stdout.flush()?;
        Ok(())
    }
    pub fn clear_all(&mut self) -> crossterm::Result<()> {
        queue!(self.stdout,
            terminal::Clear(ClearType::All),
        )?;
        self.stdout.flush()?;
        Ok(())
    }
}

impl ops::Drop for Renderer {
    fn drop(&mut self) {
        execute!(self.stdout, terminal::LeaveAlternateScreen).unwrap();
        terminal::disable_raw_mode().unwrap();
    }
}

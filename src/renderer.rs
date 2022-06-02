use std::{
    io::{self, Stdout, Write},
    ops,
};

use crossterm::{
    cursor, execute, queue, style,
    terminal,
};

use crate::math::Vec2;

pub trait Render<'a> {
    fn render(&self, stdout: &mut Stdout) -> crossterm::Result<()>;
    fn to_clear(&'a self) -> Box<dyn std::iter::Iterator<Item = &Vec2> + 'a> { // I do not know what I'am doing here, Seriously !!!
        Box::new(std::iter::empty())
    }
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
    pub fn render(&mut self, objects: &[&dyn Render]) -> crossterm::Result<()> {
        for obj in objects.iter() {
            obj.render(&mut self.stdout)?;
        }
        self.stdout.flush()?;
        Ok(())
    }
    pub fn clear<'a>(&mut self, objects: &[&'a (dyn Render<'a> + 'a)]) -> crossterm::Result<()> {
        for obj in objects {
            for i in obj.to_clear() {
                queue!(self.stdout, cursor::MoveTo(i.x as u16, i.y as u16), style::Print(" "))?;
            };
        }
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

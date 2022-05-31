use std::{
    io::{self, Stdout, Write},
    ops,
};

use crossterm::{
    cursor, execute, queue, style,
    terminal,
};

pub trait Display {
    fn display(&self, stdout: &mut Stdout) -> crossterm::Result<()>;
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
    pub fn display(&mut self, objects: &[&dyn Display]) -> crossterm::Result<()> {
        self.clear()?;
        for obj in objects.iter() {
            obj.display(&mut self.stdout)?;
        }
        self.stdout.flush()?;
        Ok(())
    }
    fn clear(&mut self) -> crossterm::Result<()> {
        let (x_max, y_max) = terminal::size()?;
        for y in 1..(y_max - 1) {
            queue!(
                self.stdout,
                cursor::MoveTo(1, y),
                style::Print(" ".repeat(x_max as usize - 2))
            )?;
        }
        Ok(())
    }
}

impl ops::Drop for Renderer {
    fn drop(&mut self) {
        execute!(self.stdout, terminal::LeaveAlternateScreen).unwrap();
        terminal::disable_raw_mode().unwrap();
    }
}

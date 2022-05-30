use std::{
    io::{self, Stdout, Write},
    ops,
};

use crossterm::{
    cursor, execute, queue,
    terminal::{self, ClearType},
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
    pub fn display_all(&mut self, objects: &[&dyn Display]) -> crossterm::Result<()> {
        queue!(self.stdout, terminal::Clear(ClearType::All))?;
        for obj in objects.iter() {
            obj.display(&mut self.stdout)?;
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

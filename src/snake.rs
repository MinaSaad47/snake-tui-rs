use std::{collections::LinkedList, io::Stdout};

use crossterm::{
    cursor, queue,
    style::{self, Stylize},
};

use crate::{math::Vec2, renderer};

pub struct Snake {
    pub head_icon: String,
    pub body_icon: String,
    pub body: LinkedList<Vec2>,
    pub dir: Vec2,
    pub alive: bool,
}

impl Snake {
    pub fn new(head_icon: &str, body_icon: &str, (x_max, y_max): (u16, u16)) -> Self {
        Self {
            head_icon: head_icon.to_string(),
            body_icon: body_icon.to_string(),
            body: {
                let mut body = LinkedList::new();
                body.push_front(Vec2::new(x_max as i16 / 2, y_max as i16 / 2));
                body
            },
            dir: Vec2::new(1, 0),
            alive: true,
        }
    }
}

impl renderer::Display for Snake {
    fn display(&self, stdout: &mut Stdout) -> crossterm::Result<()> {
        if let Some(&head) = self.body.front() {
            queue!(
                stdout,
                cursor::MoveTo(head.x as u16, head.y as u16),
                style::PrintStyledContent(self.head_icon.as_str().green())
            )?;
        }
        for part in self.body.iter().skip(1) {
            queue!(
                stdout,
                cursor::MoveTo(part.x as u16, part.y as u16),
                style::PrintStyledContent(self.body_icon.as_str().dark_yellow())
            )?;
        }
        Ok(())
    }
}

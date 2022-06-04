use std::collections::LinkedList;

use crossterm::style::{self, StyledContent, Stylize};

use crate::{math::Vec2, renderer};

pub struct Snake {
    pub head_icon: StyledContent<String>,
    pub body_icon: StyledContent<String>,
    pub body: LinkedList<Vec2>,
    pub dir: Vec2,
    pub alive: bool,
}

impl Snake {
    pub fn new(head_icon: &str, body_icon: &str, (x_max, y_max): (u16, u16)) -> Self {
        Self {
            head_icon: head_icon.to_string().green(),
            body_icon: body_icon.to_string().yellow(),
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

impl<'a> renderer::Render<'a> for Snake {
    fn render(
        &'a self,
    ) -> Box<dyn std::iter::Iterator<Item = (&'a Vec2, &'a style::StyledContent<String>)> + 'a>
    {
        Box::new(self.body.iter().enumerate().map(|(index, part)| {
            if index == 0 {
                (part, &self.head_icon)
            } else {
                (part, &self.body_icon)
            }
        }))
    }
}

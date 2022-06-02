



use crossterm::style::{self,Stylize, StyledContent};
use crate::{math::Vec2, renderer};

pub struct Food {
    pub pos: Vec2,
    pub icon: StyledContent<String>,
}

impl Food {
    pub fn new(icon: &str, pos: &Vec2) -> Self {
        Self {
            icon: icon.to_string().red(),
            pos: *pos,
        }
    }
}

use std::iter;
impl<'a> renderer::Render<'a> for Food {
    fn render(&'a self) -> Box<dyn std::iter::Iterator<Item = (&'a Vec2, &style::StyledContent<String>)> + 'a> {
        Box::new(
            iter::once((&self.pos, &self.icon))
        )
    }
}

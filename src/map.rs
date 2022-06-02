use std::{collections::LinkedList, iter};

use crossterm::{
    style::{Stylize, StyledContent},
    terminal,
};

use crate::{renderer, math::Vec2};

pub struct Map {
    top: (LinkedList<Vec2>, StyledContent<String>),
    bottom: (LinkedList<Vec2>, StyledContent<String>),
    left: (LinkedList<Vec2>, StyledContent<String>),
    right: (LinkedList<Vec2>, StyledContent<String>),
    corners: (LinkedList<Vec2>, StyledContent<String>),
}

impl Map {
    pub fn new(top: &str, bottom: &str, left: &str, right: &str, corners: &str) -> Self {
        let mut map = Self {
            top: (Default::default(), top.to_string().green()),
            bottom: (Default::default(), bottom.to_string().green()),
            left: (Default::default(), left.to_string().green()),
            right: (Default::default(), right.to_string().green()),
            corners: (Default::default(), corners.to_string().green()),
        };
        map.generate_borders();
        map
    }
    pub fn generate_borders(&mut self) {
        let (x_max, y_max) = terminal::size().unwrap();
        self.top.0      = (1..(x_max - 1)).map(|x| Vec2::new(x as i16, 0)).collect();
        self.bottom.0   = (1..(x_max - 1)).map(|x| Vec2::new(x as i16, y_max as i16)).collect();
        self.left.0     = (1..(y_max - 1)).map(|y| Vec2::new(0, y as i16)).collect();
        self.right.0    = (1..(y_max - 1)).map(|y| Vec2::new(x_max as i16, y as i16)).collect();
        self.corners.0    = [(0, 0), (x_max, 0), (0, y_max), (x_max, y_max)].into_iter().map(|(x, y)| Vec2::new(x as i16, y as i16)).collect();
    }
}

impl<'a> renderer::Render<'a> for Map {
    fn render(&'a self) -> Box<dyn std::iter::Iterator<Item = (&'a Vec2, &'a StyledContent<String>)> + 'a> {
        Box::new(
            self.top.0.iter().zip(iter::repeat(&self.top.1))
            .chain(self.bottom.0.iter().zip(iter::repeat(&self.bottom.1)))
            .chain(self.left.0.iter().zip(iter::repeat(&self.left.1)))
            .chain(self.right.0.iter().zip(iter::repeat(&self.right.1)))
            .chain(self.corners.0.iter().zip(iter::repeat(&self.corners.1)))
        )
    }
}

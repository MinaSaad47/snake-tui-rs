use std::ops;

#[derive(Clone, Copy, PartialEq, Default)]
pub struct Vec2 {
    pub x: i16,
    pub y: i16,
}

impl Vec2 {
    pub fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }
}

impl ops::Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

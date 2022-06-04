use crossterm::{event::KeyCode, terminal};
use rand::Rng;

use crate::{food::Food, math::Vec2, snake::Snake};

pub fn update_objects(snake: &mut Snake, food: &mut Food) -> crossterm::Result<()> {
    let mut prev_part = Vec2::default();
    let mut skip = 0;
    if let Some(head) = snake.body.front_mut() {
        prev_part = *head;
        if *head + snake.dir == food.pos {
            snake.body.push_front(food.pos);
            let (x_max, y_max) = terminal::size()?;
            let mut rng = rand::thread_rng();
            let (new_x, new_y) = (rng.gen_range(1..x_max), rng.gen_range(1..y_max));
            food.pos = Vec2::new(new_x as i16, new_y as i16);
            skip = snake.body.len();
        } else {
            *head = *head + snake.dir;
            skip = 1;
        }
    }

    let head = prev_part; // to check if killed itself
    for part in snake.body.iter_mut().skip(skip) {
        if head == *part {
            snake.alive = false;
            return Ok(());
        }
        let dir = prev_part - *part;
        prev_part = *part;
        *part = *part + dir;
    }
    // map colision
    let (x_max, y_max) = terminal::size()?;
    for part in snake.body.iter_mut() {
        // left colision
        if part.x == 0 && snake.dir == Vec2::new(-1, 0) {
            part.x = x_max as i16 - 2;
        }
        // right colision
        if part.x == x_max as i16 - 1 && snake.dir == Vec2::new(1, 0) {
            part.x = 1;
        }
        // top colision
        if part.y == 0 && snake.dir == Vec2::new(0, -1) {
            part.y = y_max as i16 - 2;
        }
        // bottom colision
        if part.y == y_max as i16 - 1 && snake.dir == Vec2::new(0, 1) {
            part.y = 1;
        }
    }
    Ok(())
}

pub fn handle_keyboard(code: KeyCode, snake: &mut Snake) -> bool {
    let new_dir = match code {
        KeyCode::Char('q') => return false,
        KeyCode::Up => Vec2::new(0, -1),
        KeyCode::Down => Vec2::new(0, 1),
        KeyCode::Left => Vec2::new(-1, 0),
        KeyCode::Right => Vec2::new(1, 0),
        _ => snake.dir,
    };
    if snake.dir + new_dir != Vec2::new(0, 0) || snake.body.len() == 1 {
        snake.dir = new_dir
    }
    true
}

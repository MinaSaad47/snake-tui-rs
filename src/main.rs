#![allow(unused_imports)]

const SNAKE_HEAD: &str = "#";
const SNAKE_BODY: &str = "O";
const FOOD: &str = "O";

use std::{
    borrow::BorrowMut,
    cell::RefCell,
    collections::LinkedList,
    io::{self, Stdout, StdoutLock, Write},
    iter::Skip,
    ops, thread,
    time::Duration,
};

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute, queue,
    style::{self, StyledContent, Stylize},
    terminal::{self, ClearType},
};

use rand::{self, Rng};
use snake_tui_rs::{food::Food, map::Map, math::Vec2, renderer::Renderer, snake::Snake};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // game object
    let map = Map::new("-", "-", "|", "|", "+");
    let mut snake = Snake::new(SNAKE_HEAD, SNAKE_BODY, terminal::size()?);
    let mut food = Food::new(FOOD, &Vec2::new(16, 16));

    let mut renderer = Renderer::new();

    // game loop
    while snake.alive {
        // event handling
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => {
                    if !handle_keyboard(key.code, &mut snake) {
                        break;
                    }
                }
                _ => {}
            }
        }
        // update game objects
        update_objects(&mut snake, &mut food)?;
        // display game object
        renderer.display_all(&[&map, &snake, &food])?;
    }

    Ok(())
}

fn update_objects(snake: &mut Snake, food: &mut Food) -> crossterm::Result<()> {
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
    Ok(())
}

fn handle_keyboard(code: KeyCode, snake: &mut Snake) -> bool {
    match code {
        KeyCode::Char('q') => return false,
        KeyCode::Up => snake.dir = Vec2::new(0, -1),
        KeyCode::Down => snake.dir = Vec2::new(0, 1),
        KeyCode::Left => snake.dir = Vec2::new(-1, 0),
        KeyCode::Right => snake.dir = Vec2::new(1, 0),
        _ => {}
    }
    true
}

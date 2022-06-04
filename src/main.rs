const SNAKE_HEAD_CHAR: &str = "#";
const SNAKE_BODY_CHAR: &str = "O";
const FOOD_CHAR: &str = "O";

use std::time::Duration;

use crossterm::{
    event::{self, Event},
    terminal,
};

use snake_tui_rs::{food::Food, game, map::Map, math::Vec2, renderer::Renderer, snake::Snake};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // game object
    let mut map = Map::new("-", "-", "|", "|", "+");
    let mut food = Food::new(FOOD_CHAR, &Vec2::new(16, 16));
    let mut snake = Snake::new(SNAKE_HEAD_CHAR, SNAKE_BODY_CHAR, terminal::size()?);

    let mut renderer = Renderer::new();

    renderer.render(&[&map])?;

    // game loop
    while snake.alive {
        // event handling
        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => {
                    if !game::handle_keyboard(key.code, &mut snake) {
                        break;
                    }
                }
                Event::Resize(_, _) => {
                    renderer.clear_all()?;
                    map.generate_borders();
                    renderer.render(&[&map])?;
                }
                _ => {}
            }
        }
        // clear game objects
        renderer.clear(&[&snake, &food])?;
        // update game objects
        game::update_objects(&mut snake, &mut food)?;
        // display game object
        renderer.render(&[&snake, &food])?;
    }

    Ok(())
}

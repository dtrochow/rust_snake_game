use crate::utils::has_duplicate_coordinates;
use crate::snake::Snake;
use crate::food::Food;

use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;
use piston::input::*;
use std::borrow::Borrow;


#[derive(Clone, PartialEq)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down
}

pub struct Game {
    pub gl: GlGraphics,
    pub snake: Snake,
    pub food: Food,
    pub game_over: bool,
    pub points: u64,
    pub increase_speed: bool
}

impl Game  {
    pub fn render(&mut self, args: &RenderArgs) { 
        let green: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |_c, gl| {
            graphics::clear(green, gl);
        });

        self.snake.render(&mut self.gl, args);
        self.food.render(&mut self.gl, args);
    }

    fn check_snake_food_collision(&self) -> bool {
        let snake_head = self.snake.body.first().unwrap();
        (self.food.x_pos / (self.food.size as f64), self.food.y_pos / (self.food.size as f64)) == (snake_head.0 as f64, snake_head.1 as f64)
    }

    fn check_snake_self_collision(&self) -> bool {
        has_duplicate_coordinates(self.snake.body.borrow())
    }

    pub fn update(&mut self) {
        self.snake.update();
        if self.check_snake_food_collision() {
            self.food.update();
            self.snake.grow();
            self.points += 1;
            if self.points % 3 == 0 {
                self.increase_speed = true;
            }
        }
        if self.check_snake_self_collision() {
            self.end_the_game();
        }
    }

    pub fn pressed(&mut self, btn: &Button) {
        let last_direction = self.snake.direction.clone();

        self.snake.direction = match btn {
            &Button::Keyboard(Key::Up) if last_direction != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::Down) if last_direction != Direction::Up => Direction::Down,
            &Button::Keyboard(Key::Left) if last_direction != Direction::Right => Direction::Left,
            &Button::Keyboard(Key::Right) if last_direction != Direction::Left => Direction::Right,
            _ => last_direction
        };
    }

    fn end_the_game(&mut self) {
        self.game_over = true;
    }
}

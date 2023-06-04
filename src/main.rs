mod utils;
mod game;
mod snake;
mod food;

use crate::game::Direction;

extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

use std::collections::LinkedList;

const SNAKE_SIZE: u32  = 20;
const WINDOW_WIDTH:  u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

fn main() {
    let opengl = OpenGL::V3_2;
    let mut frames: u64 = 10;

    let mut window: GlutinWindow = WindowSettings::new(
        "Snake Game",
        [WINDOW_WIDTH, WINDOW_HEIGHT]
    ).graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut events = Events::new(EventSettings::new()).ups(frames);

    let mut game = game::Game {
        gl: GlGraphics::new(opengl),
        snake: snake::Snake {
            body: LinkedList::from_iter((vec![(0, 0), (0, 1), (1,1)]).into_iter()),  
            direction: Direction::Right,
            window_size: (WINDOW_WIDTH, WINDOW_HEIGHT),
            snake_size: SNAKE_SIZE
        },
        food: food::Food::new(SNAKE_SIZE, (WINDOW_WIDTH, WINDOW_HEIGHT)),
        game_over: false,
        points: 0,
        increase_speed: false
    };

    while let Some(e) = events.next(&mut window) {
        if game.game_over {
            println!("Game Over!");
            break;
        }

        if game.increase_speed {
            frames += 2;
            events.set_ups(frames);
            game.increase_speed = false;
        }

        if let Some(r) = e.render_args() {
            game.render(&r);
        }

        if let Some(_u) = e.update_args() {
            game.update();
        }

        if let Some(k) = e.button_args() {
            if k.state == ButtonState::Press {
                game.pressed(&k.button);
            }
        }
    }
}

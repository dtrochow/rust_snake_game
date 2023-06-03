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
use rand::Rng;

const SNAKE_SIZE: u32  = 15;
const WINDOW_WIDTH:  u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

#[derive(Clone, PartialEq)]
enum Direction {
    Right,
    Left,
    Up,
    Down
}

struct Game {
    gl: GlGraphics,     // OpenGL drawing backend.  
    snake: Snake,       // Snake object
    food: Food          // Food object
}

impl Game  {
    fn render(&mut self, args: &RenderArgs) { 
        let green: [f32; 4] = [0.0, 1.0, 0.0, 1.0];

        self.gl.draw(args.viewport(), |_c, gl| {
            graphics::clear(green, gl);
        });

        self.snake.render(&mut self.gl, args);
        self.food.render(&mut self.gl, args);
    }

    fn update(&mut self) {
        self.snake.update();
        self.food.update();
    }

    fn pressed(&mut self, btn: &Button) {
        let last_direction = self.snake.direction.clone();

        self.snake.direction = match btn {
            &Button::Keyboard(Key::Up) if last_direction != Direction::Down => Direction::Up,
            &Button::Keyboard(Key::Down) if last_direction != Direction::Up => Direction::Down,
            &Button::Keyboard(Key::Left) if last_direction != Direction::Right => Direction::Left,
            &Button::Keyboard(Key::Right) if last_direction != Direction::Left => Direction::Right,
            _ => last_direction
        };
    }
}

struct Snake {
    body: LinkedList<(i32, i32)>,
    direction: Direction
}

impl Snake {
    fn render(&self,  gl: &mut GlGraphics,  args: &RenderArgs) {
        let red: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let squares: Vec<graphics::types::Rectangle> = self.body
            .iter()
            .map(|&(x, y)| { graphics::rectangle::square(
                (x * (SNAKE_SIZE as i32)) as f64,
                (y * (SNAKE_SIZE as i32)) as f64,
                SNAKE_SIZE as f64)
        })
        .collect();
        
        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            squares.into_iter()
                .for_each(|square| graphics::rectangle(red, square, transform, gl));
        });
    }

    fn update(&mut self) {
        let mut new_head = (*self.body.front().expect("Snake has no body")).clone();

        match self.direction {
            Direction::Left => new_head.0 -= 1,
            Direction::Right => new_head.0 += 1,
            Direction::Down => new_head.1 += 1,
            Direction::Up => new_head.1 -= 1
        }

        self.body.push_front(new_head);

        self.body.pop_back().unwrap();
    }
}

fn generate_initial_random_position() -> (f64, f64) {
    let mut rng = rand::thread_rng();

    // Calculate the number of rows and columns in the grid
    let num_rows = WINDOW_HEIGHT / SNAKE_SIZE;
    let num_cols = WINDOW_WIDTH / SNAKE_SIZE;

    // Generate random row and column indices
    let row_index = rng.gen_range(0..num_rows);
    let col_index = rng.gen_range(0..num_cols);

    // Calculate the actual position by multiplying the indices with the square size
    let x = col_index * SNAKE_SIZE;
    let y = row_index * SNAKE_SIZE;

    (x as f64, y as f64)
}

struct Food {
    x_pos: f64,
    y_pos: f64,
    size: f64
}

impl Food {
    fn new(size: f64) -> Food {
        let (x_pos, y_pos) = generate_initial_random_position();
        Food {
            x_pos,
            y_pos,
            size
        }
    }

    fn render(&self,  gl: &mut GlGraphics,  args: &RenderArgs) {
        let red: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square: graphics::types::Rectangle = graphics::rectangle::square(self.x_pos, self.y_pos, self.size);
        
        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(red, square, transform, gl);
        });
    }

    fn update(&mut self) {
        // self.x_pos += 1.0 * SNAKE_SIZE as f64;
    }
}

fn main() {
    let opengl = OpenGL::V3_2;

    let mut window: GlutinWindow = WindowSettings::new(
        "Snake Game",
        [WINDOW_WIDTH, WINDOW_HEIGHT]
    ).graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        snake: Snake {
            body: LinkedList::from_iter((vec![(0, 0), (0, 1), (1,1)]).into_iter()),  
            direction: Direction::Right
        },
        food: Food::new(SNAKE_SIZE as f64)
    };

    let mut events = Events::new(EventSettings::new()).ups(8);
    while let Some(e) = events.next(&mut window) {
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

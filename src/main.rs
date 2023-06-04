extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

use std::borrow::Borrow;
use std::collections::{LinkedList, HashSet};
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
    food: Food,         // Food object
    game_over: bool,
    points: u64,
    increase_speed: bool
}

fn has_duplicate_coordinates(list: &LinkedList<(i32, i32)>) -> bool {
    let mut seen_coordinates: HashSet<&(i32, i32)> = HashSet::new();

    for coordinate in list.iter() {
        if !seen_coordinates.insert(coordinate) {
            return true;
        }
    }

    false
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

    fn check_snake_food_collision(&self) -> bool {
        let snake_head = self.snake.body.front().unwrap();
        (self.food.x_pos / self.food.size, self.food.y_pos / self.food.size) == (snake_head.0 as f64, snake_head.1 as f64)
    }

    fn check_snake_self_collision(&self) -> bool {
        has_duplicate_coordinates(self.snake.body.borrow())
    }

    fn update(&mut self) {
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

    fn end_the_game(&mut self) {
        self.game_over = true;
    }
}

struct Snake {
    body: LinkedList<(i32, i32)>,
    direction: Direction
}

fn penultimate_element<T>(list: &LinkedList<T>) -> Option<&T> {
    let mut iter = list.iter();
    let mut prev = None;
    let mut current = None;

    while let Some(item) = iter.next() {
        prev = current;
        current = Some(item);
    }

    prev
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

        let right_edge =  (WINDOW_WIDTH / SNAKE_SIZE) as i32;
        let left_edge = 0;
        let top_edge = 0;
        let bottom_edge = (WINDOW_HEIGHT / SNAKE_SIZE) as i32;

        match self.direction {
            Direction::Left => {
                if new_head.0 == 0 {
                    new_head.0 = right_edge;
                } else {
                    new_head.0 -= 1
                }   
            }
            Direction::Right => {
                if new_head.0 == right_edge {
                    new_head.0 = left_edge;
                } else {
                    new_head.0 += 1
                }
            }
            Direction::Down => {
                if new_head.1 == bottom_edge {
                    new_head.1 = top_edge;
                } else {
                    new_head.1 += 1
                }
            }
            Direction::Up => {
                if new_head.1 == top_edge {
                    new_head.1 = bottom_edge;
                } else {
                    new_head.1 -= 1
                }
            }
        }

        self.body.push_front(new_head);

        self.body.pop_back().unwrap();
    }

    fn grow(&mut self) {
        let last_tail_segment = self.body.back().unwrap().clone();
        let penultimate_tail_segment = penultimate_element(self.body.borrow()).unwrap().clone();

        let grow_segment = (2 * last_tail_segment.0 - penultimate_tail_segment.0, 2 * last_tail_segment.1 - penultimate_tail_segment.1);

        self.body.push_back(grow_segment);
    }
}

fn generate_random_food_position() -> (f64, f64) {
    let mut rng = rand::thread_rng();

    let num_rows = WINDOW_HEIGHT / SNAKE_SIZE;
    let num_cols = WINDOW_WIDTH / SNAKE_SIZE;

    let row_index = rng.gen_range(0..num_rows);
    let col_index = rng.gen_range(0..num_cols);

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
        let (x_pos, y_pos) = generate_random_food_position();
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
        (self.x_pos, self.y_pos) = generate_random_food_position();
    }
}

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

    let mut game = Game {
        gl: GlGraphics::new(opengl),
        snake: Snake {
            body: LinkedList::from_iter((vec![(0, 0), (0, 1), (1,1)]).into_iter()),  
            direction: Direction::Right
        },
        food: Food::new(SNAKE_SIZE as f64),
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

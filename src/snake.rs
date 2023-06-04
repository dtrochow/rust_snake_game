use crate::game::Direction;
use crate::utils::penultimate_element;

use opengl_graphics::GlGraphics;
use std::collections::LinkedList;
use piston::input::RenderArgs;
use std::borrow::Borrow;

pub struct Snake {
    pub body: LinkedList<(i32, i32)>,
    pub direction: Direction,
    pub window_size: (u32, u32),
    pub snake_size: u32
}

impl Snake {
    pub fn render(&self,  gl: &mut GlGraphics,  args: &RenderArgs) {
        let red: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let squares: Vec<graphics::types::Rectangle> = self.body
            .iter()
            .map(|&(x, y)| { graphics::rectangle::square(
                (x * (self.snake_size as i32)) as f64,
                (y * (self.snake_size as i32)) as f64,
                self.snake_size as f64)
        })
        .collect();
        
        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            squares.into_iter()
                .for_each(|square| graphics::rectangle(red, square, transform, gl));
        });
    }

    pub fn update(&mut self) {
        let mut new_head = (*self.body.front().expect("Snake has no body")).clone();

        let window_width: u32 =  self.window_size.0;
        let window_height: u32 =  self.window_size.1;

        let right_edge =  (window_width / self.snake_size) as i32;
        let left_edge = 0;
        let top_edge = 0;
        let bottom_edge = (window_height / self.snake_size) as i32;

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

    pub fn grow(&mut self) {
        let last_tail_segment = self.body.back().unwrap().clone();
        let penultimate_tail_segment = penultimate_element(self.body.borrow()).unwrap().clone();

        let grow_segment = (2 * last_tail_segment.0 - penultimate_tail_segment.0, 2 * last_tail_segment.1 - penultimate_tail_segment.1);

        self.body.push_back(grow_segment);
    }
}

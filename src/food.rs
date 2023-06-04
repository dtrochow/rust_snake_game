use rand::Rng;
use opengl_graphics::GlGraphics;
use piston::input::RenderArgs;

pub struct Food {
    pub x_pos: f64,
    pub y_pos: f64,
    pub size: u32,
    window_size: (u32, u32),
}

fn generate_random_food_position(window_size: (u32, u32), snake_size: u32) -> (f64, f64) {
    let window_width = window_size.0;
    let window_height = window_size.1;
    let mut rng = rand::thread_rng();

    let num_rows = window_height / snake_size;
    let num_cols = window_width / snake_size;

    let row_index = rng.gen_range(0..num_rows);
    let col_index = rng.gen_range(0..num_cols);

    let x = col_index * snake_size;
    let y = row_index * snake_size;

    (x as f64, y as f64)
}

impl Food {
    pub fn new(size: u32, window_size: (u32, u32)) -> Food {
        let (x_pos, y_pos) = generate_random_food_position(window_size, size);
        Food {
            x_pos,
            y_pos,
            size,
            window_size
        }
    }

    pub fn render(&self,  gl: &mut GlGraphics,  args: &RenderArgs) {
        let red: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let square: graphics::types::Rectangle = graphics::rectangle::square(self.x_pos, self.y_pos, self.size as f64);
        
        gl.draw(args.viewport(), |c, gl| {
            let transform = c.transform;

            graphics::rectangle(red, square, transform, gl);
        });
    }

    pub fn update(&mut self) {
        (self.x_pos, self.y_pos) = generate_random_food_position(self.window_size, self.size);
    }
}

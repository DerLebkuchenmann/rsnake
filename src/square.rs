#![allow(dead_code)]

use graphics::*;
use opengl_graphics::GlGraphics;
use piston::input::{RenderArgs, UpdateArgs};

#[path = "./settings.rs"]
pub mod settings;

pub struct Square {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub rotation: f64,
    pub rot_speed: f64,
    pub mov_speed_x: f64,
    pub mov_speed_y: f64,
    pub color: [f32; 4],
    square: graphics::types::Rectangle,
}

impl Square {
    pub fn new(
        x: f64,
        y: f64,
        width: f64,
        rotation: f64,
        rot_speed: f64,
        mov_speed_x: f64,
        mov_speed_y: f64,
        color: [f32; 4],
    ) -> Square {
        Square {
            x,
            y,
            width,
            rotation,
            rot_speed,
            mov_speed_x,
            mov_speed_y,
            color,
            square: rectangle::square(0.0, 0.0, width),
        }
    }
    pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs) {
        gl.draw(args.viewport(), |c, gl| {
            let transform = c
                .transform
                .trans(self.x, self.y)
                .rot_rad(self.rotation)
                // transform rotating point to center of square d
                .trans(-0.5 * self.width, -0.5 * self.width);

            // println!("{:?}", self.color);
            // Draw a box rotating around the middle of the screen.
            rectangle(self.color, self.square, transform, gl);
        });
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.x += self.mov_speed_x * args.dt;
        self.y += self.mov_speed_y * args.dt;

        if self.x - self.width / 2.0 <= 0.0 || self.x + self.width / 2.0 >= settings::WINDOWSIZE[0]
        {
            self.mov_speed_x = -self.mov_speed_x;
        }
        if self.y - self.width / 2.0 <= 0.0 || self.y + self.width / 2.0 >= settings::WINDOWSIZE[1]
        {
            self.mov_speed_y = -self.mov_speed_y;
        }
    }
}

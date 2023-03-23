use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, Window};

use crate::Canvas;

pub struct MovingObject {
    pub color: Vec<f32>,
    pub x: f64,
    pub y: f64,
    dx: f64,
    dy: f64,
}

impl MovingObject {
    pub fn new(x: f64, y: f64, dx: f64, dy: f64) -> MovingObject {
        let mut rng = rand::thread_rng();
        let color = vec!(
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
            1.0,
        );

        MovingObject {
            color,
            x,
            y,
            dx,
            dy,
        }
    }

    pub fn render(&mut self, canvas: &mut Canvas) {
        // canvas.draw(self.x, self.y);
    }

    pub fn move_frame(&mut self, dt: f64, width: f64, height: f64) {
        let top = 0.0;
        let left = 0.0;
        let bottom = height;
        let right = width;

        let dx = self.dx * dt;
        let dy = self.dy * dt;

        if self.y + dy < top { self.dy = -self.dy; }
        if self.x + dy < left { self.dx = -self.dx; }
        if bottom < self.y + dy { self.dy = -self.dy; }
        if right < self.x + dx { self.dx = -self.dx; }

        self.x += self.dx * dt;
        self.y += self.dy * dt;
    }
}

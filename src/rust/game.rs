use js_sys::Array;
use rand::Rng;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, Window};

use crate::Canvas;
use crate::MovingObject;

pub struct Game {
    canvas: Canvas,
    shapes: Vec<MovingObject>,
}

impl Game {
    pub fn new(canvas: Canvas) -> Game {
        let mut rng = rand::thread_rng();
        let shapes = (0..100000) // Change this number to generate more or fewer shapes
            .map(|_| {
                let x = rng.gen_range(0.0..canvas.width);
                let y = rng.gen_range(0.0..canvas.height);
                let dx = rng.gen_range(-100.0..100.0);
                let dy = rng.gen_range(-100.0..100.0);
                MovingObject::new(x, y, dx, dy)
            })
            .collect::<Vec<MovingObject>>();

        Game {
            canvas,
            shapes,
        }
    }

    fn render(&mut self) {
        // let positions = self.shapes.iter().map(|s| (s.x, s.y)).collect::<Vec<_>>();
        // let positions_js_array = positions
        //     .iter()
        //     .map(|(x, y)| {
        //         let position = Array::new();
        //         position.push(&JsValue::from_f64(*x as f64));
        //         position.push(&JsValue::from_f64(*y as f64));
        //         position
        //     })
        //     .collect::<Array>();

        self.canvas.render(&self.shapes);
    }

    fn move_frame(&mut self, dt: f64) {
        for shape in &mut self.shapes {
            shape.move_frame(dt, self.canvas.width, self.canvas.height);
        }
    }

    pub fn tick(&mut self, dt: f64) {
        self.move_frame(dt);
        self.render();
    }
}

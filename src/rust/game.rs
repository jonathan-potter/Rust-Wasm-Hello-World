use js_sys::Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, Window};

use crate::Canvas;
use crate::MovingObject;

#[wasm_bindgen]
pub struct Game {
    canvas: Canvas,
    shapes: Vec<MovingObject>,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: Canvas) -> Game {
        let shapes =

        Game {
            canvas: canvas,
            shapes: vec![
                MovingObject::new(100.0, 100.0, 100.0, 100.0),
                MovingObject::new(100.0, 200.0, 100.0, 100.0),
                MovingObject::new(100.0, 300.0, 100.0, 100.0),
            ],
        }
    }

    fn render(&mut self) {
        let positions = self.shapes.iter().map(|s| (s.x, s.y)).collect::<Vec<_>>();
        let positions_js_array = positions
            .iter()
            .map(|(x, y)| {
                let position = Array::new();
                position.push(&JsValue::from_f64(*x as f64));
                position.push(&JsValue::from_f64(*y as f64));
                position
            })
            .collect::<Array>();

        self.canvas.render(positions_js_array);
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

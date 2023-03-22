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
    shape: MovingObject,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new(canvas: Canvas) -> Game {
        Game {
            canvas: canvas,
            shape: MovingObject::new(),
        }
    }

    fn render(&mut self) {
        let positions = Array::new();

        let position1 = Array::new();
        position1.push(&JsValue::from_f64(self.shape.x));
        position1.push(&JsValue::from_f64(self.shape.y));

        positions.push(&position1);

        self.canvas.render(positions);
    }

    fn move_frame(&mut self, dt: f64) {
        self.shape.move_frame(dt, self.canvas.width, self.canvas.height);
    }

    pub fn tick(&mut self, dt: f64) {
        self.move_frame(dt);
        self.render();
    }
}

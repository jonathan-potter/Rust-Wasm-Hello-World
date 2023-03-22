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
        self.shape.render(&mut self.canvas);
    }

    fn move_frame(&mut self) {
        self.shape.move_frame();
    }

    pub fn tick(&mut self) {
        self.canvas.clear();

        self.move_frame();
        self.render();
    }
}

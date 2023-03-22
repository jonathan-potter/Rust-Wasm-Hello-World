use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, Window};

use crate::Canvas;

#[wasm_bindgen]
pub struct MovingObject {
    x: f64,
    y: f64,
    dx: f64,
    dy: f64,
}

#[wasm_bindgen]
impl MovingObject {
    #[wasm_bindgen(constructor)]
    pub fn new() -> MovingObject {
        MovingObject {
            x: 100.0,
            y: 100.0,
            dx: 2.0,
            dy: 2.0,
        }
    }

    pub fn render(&mut self, canvas: &mut Canvas) {
        canvas.drawCircle(self.x, self.y)
    }

    pub fn move_frame(&mut self) {
        self.x += self.dx;
        self.y += self.dy;
    }
}
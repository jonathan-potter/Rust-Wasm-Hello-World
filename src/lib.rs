use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, Window};

#[path = "rust/shared.rs"] pub mod shared;
pub use shared::*;

use canvas::Canvas;
use game::Game;
use movingObject::MovingObject;

#[wasm_bindgen]
pub struct Integration {
    game: Game,
}

#[wasm_bindgen]
impl Integration {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Integration, JsValue> {
        let canvas = Canvas::new()?;
        let game = Game::new(canvas);

        Ok(Integration {
            game: game,
        })
    }

    pub fn tick(&mut self, time: f64) {
        self.game.tick(time);
    }
}

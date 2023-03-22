use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, Window};

#[wasm_bindgen]
pub struct Canvas {
    window: Window,
    document: Document,
    context: CanvasRenderingContext2d,
    pub width: f64,
    pub height: f64,
}

#[wasm_bindgen]
impl Canvas {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Canvas, JsValue> {
        let window = web_sys::window().ok_or("no window")?;
        let document = window.document().ok_or("no document")?;
        let canvas: HtmlCanvasElement = document.create_element("canvas")?.dyn_into()?;
        let context: CanvasRenderingContext2d = canvas.get_context("2d")?.unwrap().dyn_into()?;

        document.body().unwrap().append_child(&canvas)?;

        canvas.set_width(640);
        canvas.set_height(480);

        Ok(Canvas {
            window,
            document,
            context,
            width: 640.0,
            height: 480.0,
        })
    }

    pub fn clear(&mut self) {
        self.context.clear_rect(0.0, 0.0, self.width, self.height);
    }

    pub fn drawCircle(&mut self, x: f64, y: f64) {
        self.context.set_fill_style(&JsValue::from_str("red"));
        self.context.begin_path();
        self.context.arc(x, y, 20.0, 0.0, std::f64::consts::PI * 2.0).unwrap();
        self.context.fill();
    }
}

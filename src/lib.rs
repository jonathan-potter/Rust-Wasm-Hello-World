use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::closure::Closure;
use web_sys::{CanvasRenderingContext2d, Document, HtmlCanvasElement, Window};

#[wasm_bindgen]
pub struct MovingShape {
    window: Window,
    document: Document,
    context: CanvasRenderingContext2d,
    x: f64,
    y: f64,
    dx: f64,
    dy: f64,
    closure: Rc<RefCell<Option<Closure<dyn FnMut()>>>>,
}

#[wasm_bindgen]
impl MovingShape {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<MovingShape, JsValue> {
        let window = web_sys::window().ok_or("no window")?;
        let document = window.document().ok_or("no document")?;
        let canvas: HtmlCanvasElement = document.create_element("canvas")?.dyn_into()?;
        let context: CanvasRenderingContext2d = canvas.get_context("2d")?.unwrap().dyn_into()?;

        document.body().unwrap().append_child(&canvas)?;

        canvas.set_width(640);
        canvas.set_height(480);

        Ok(MovingShape {
            window,
            document,
            context,
            x: 100.0,
            y: 100.0,
            dx: 2.0,
            dy: 2.0,
            closure: Rc::new(RefCell::new(None)),
        })
    }

    pub fn render(&mut self) {
        self.animate();
    }

    fn animate(&mut self) {
        let window = self.window.clone();
        let context = self.context.clone();
        let mut x = self.x;
        let mut y = self.y;
        let mut dx = self.dx;
        let mut dy = self.dy;
        let closure_cell = self.closure.clone();

        let closure = Closure::wrap(Box::new(move || {
            context.clear_rect(0.0, 0.0, 640.0, 480.0);

            context.set_fill_style(&JsValue::from_str("red"));
            context.begin_path();
            context.arc(x, y, 20.0, 0.0, std::f64::consts::PI * 2.0).unwrap();
            context.fill();

            x += dx;
            y += dy;

            dx = if x > 620.0 || x < 20.0 { -dx } else { dx };
            dy = if y > 460.0 || y < 20.0 { -dy } else { dy };

            window
                .request_animation_frame(closure_cell.borrow().as_ref().unwrap().as_ref().unchecked_ref())
                .unwrap();
        }) as Box<dyn FnMut()>);

        *self.closure.borrow_mut() = Some(closure);
        self.window
            .request_animation_frame(self.closure.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .unwrap();
    }
}

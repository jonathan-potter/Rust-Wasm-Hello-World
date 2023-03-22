use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext, WebGlShader};

use crate::utility::geometry_utils::{create_indices_buffer, create_vertex_buffer};
use crate::utility::shader_utils::{compile_shader, link_program};

#[wasm_bindgen]
pub struct Canvas {
    context: WebGlRenderingContext,
    pub width: f64,
    pub height: f64,
}

#[wasm_bindgen]
impl Canvas {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Result<Canvas, JsValue> {
        let window = web_sys::window().ok_or("no window")?;
        let document = window.document().ok_or("no document")?;
        let canvas = document
            .create_element("canvas")?
            .dyn_into::<web_sys::HtmlCanvasElement>()?;

        document.body().unwrap().append_child(&canvas)?;

        canvas.set_width(640);
        canvas.set_height(480);

        let context = canvas
            .get_context("webgl")?
            .ok_or("no webgl context")?
            .dyn_into::<WebGlRenderingContext>()?;

        Ok(Canvas {
            context,
            width: 640.0,
            height: 480.0,
        })
    }

    pub fn draw(&mut self) -> Result<(), JsValue> {
        let vertex_shader = compile_shader(
            &self.context,
            WebGlRenderingContext::VERTEX_SHADER,
            r#"attribute vec4 position;
            void main() {
                gl_Position = position;
            }"#,
        )?;
        let fragment_shader = compile_shader(
            &self.context,
            WebGlRenderingContext::FRAGMENT_SHADER,
            r#"precision mediump float;
            void main() {
                gl_FragColor = vec4(1, 0, 0, 1);
            }"#,
        )?;

        let program = link_program(&self.context, &vertex_shader, &fragment_shader)?;
        self.context.use_program(Some(&program));

        let vertex_positions = vec![-0.5, -0.5, 0.0, 0.5, -0.5, 0.0, 0.0, 0.5, 0.0];
        let vertex_buffer = create_vertex_buffer(&self.context, &vertex_positions)?;

        let indices = vec![0, 1, 2];
        let index_buffer = create_indices_buffer(&self.context, &indices)?;

        self.context.bind_buffer(
            WebGlRenderingContext::ARRAY_BUFFER,
            Some(&vertex_buffer),
        );
        self.context.bind_buffer(
            WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            Some(&index_buffer),
        );

        let position_attrib_location = self
            .context
            .get_attrib_location(&program, "position") as u32;
        self.context.enable_vertex_attrib_array(position_attrib_location);

        self.context.vertex_attrib_pointer_with_i32(
            position_attrib_location,
            3,
            WebGlRenderingContext::FLOAT,
            false,
            0,
            0,
        );

        self.context.clear_color(0.0, 0.0, 0.0, 1.0);
        self.context.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        self.context.draw_elements_with_i32(
            WebGlRenderingContext::TRIANGLES,
            indices.len() as i32,
            WebGlRenderingContext::UNSIGNED_SHORT,
            0,
        );

        Ok(())
    }
}

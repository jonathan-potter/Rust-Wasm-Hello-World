use js_sys::Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext};

use crate::utility::shader_utils::{compile_shader, link_program};

#[wasm_bindgen]
pub struct Canvas {
    context: WebGlRenderingContext,
    program: WebGlProgram,
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

        let vertex_shader = compile_shader(
            &context,
            WebGlRenderingContext::VERTEX_SHADER,
            r#"
            attribute vec2 position;
            uniform vec2 u_resolution;

            void main() {
                vec2 zeroToOne = position / u_resolution;
                vec2 zeroToTwo = zeroToOne * 2.0;
                vec2 clipSpace = zeroToTwo - 1.0;

                gl_Position = vec4(clipSpace * vec2(1, -1), 0, 1);
            }
            "#,
        )?;

        let fragment_shader = compile_shader(
            &context,
            WebGlRenderingContext::FRAGMENT_SHADER,
            r#"precision mediump float;
            void main() {
                gl_FragColor = vec4(1, 0, 0, 1);
            }"#,
        )?;

        let program = link_program(&context, &vertex_shader, &fragment_shader)?;
        context.use_program(Some(&program));

        Ok(Canvas {
            context,
            program,
            width: 640.0,
            height: 480.0,
        })
    }

    pub fn render(&mut self, positions: Array) {
        // Set the u_resolution uniform
        let resolution_location = self
            .context
            .get_uniform_location(&self.program, "u_resolution")
            .unwrap();
        self.context.uniform2f(
            Some(&resolution_location),
            self.width as f32,
            self.height as f32,
        );

        for i in 0..positions.length() {
            let position = positions.get(i).unchecked_into::<js_sys::Array>();
            let x = position.get(0).as_f64().unwrap() as f32;
            let y = position.get(1).as_f64().unwrap() as f32;
            self.draw_triangle(x, y);
        }
    }

    fn draw_triangle(&mut self, x: f32, y: f32) {
        let vertices: [f32; 6] = [
            x, y,
            x + 10.0, y + 10.0,
            x + 20.0, y,
        ];

        let buffer = self.context.create_buffer().unwrap();
        self.context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

        // Convert the vertices array into a Float32Array
        let vertices_js_array = js_sys::Float32Array::from(&vertices[..]);

        self.context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vertices_js_array,
            WebGlRenderingContext::STATIC_DRAW,
        );

        self.context.vertex_attrib_pointer_with_i32(0, 2, WebGlRenderingContext::FLOAT, false, 0, 0);
        self.context.enable_vertex_attrib_array(0);

        self.context.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, (vertices.len() / 2) as i32);
    }
}

use js_sys::Array;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGlProgram, WebGlRenderingContext};

use crate::utility::shader_utils::{compile_shader, link_program};
use crate::MovingObject;

pub struct Canvas {
    context: WebGlRenderingContext,
    program: WebGlProgram,
    pub width: f64,
    pub height: f64,
}

impl Canvas {
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
            attribute vec4 color;
            uniform vec2 u_resolution;
            varying vec4 v_color;

            void main() {
                vec2 zeroToOne = position / u_resolution;
                vec2 zeroToTwo = zeroToOne * 2.0;
                vec2 clipSpace = zeroToTwo - 1.0;

                gl_Position = vec4(clipSpace * vec2(1, -1), 0, 1);
                v_color = color;
            }
            "#,
        )?;

        let fragment_shader = compile_shader(
            &context,
            WebGlRenderingContext::FRAGMENT_SHADER,
            r#"precision mediump float;
            varying vec4 v_color;
            void main() {
                gl_FragColor = v_color;
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

    pub fn render(&mut self, moving_objects: &Vec<MovingObject>) {
        let resolution_location = self
            .context
            .get_uniform_location(&self.program, "u_resolution")
            .unwrap();
        self.context.uniform2f(
            Some(&resolution_location),
            self.width as f32,
            self.height as f32,
        );

        let mut vertices = Vec::new();
        let mut colors = Vec::new();

        for obj in moving_objects {
            let x = obj.x as f32;
            let y = obj.y as f32;

            vertices.extend_from_slice(&[
                x, y,
                x + 10.0, y + 10.0,
                x + 20.0, y,
            ]);

            for _ in 0..3 {
                colors.extend_from_slice(&obj.color);
            }
        }

        self.draw_triangles(&vertices, &colors);
    }

    fn draw_triangles(&mut self, vertices: &[f32], colors: &[f32]) {
        let buffer = self.context.create_buffer().unwrap();
        self.context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

        // Convert the vertices array into a Float32Array
        let vertices_js_array = js_sys::Float32Array::from(vertices);

        self.context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vertices_js_array,
            WebGlRenderingContext::STATIC_DRAW,
        );

        self.context.vertex_attrib_pointer_with_i32(0, 2, WebGlRenderingContext::FLOAT, false, 0, 0);
        self.context.enable_vertex_attrib_array(0);

        let color_buffer = self.context.create_buffer().unwrap();
        self.context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&color_buffer));

        // Convert the colors array into a Float32Array
        let colors_js_array = js_sys::Float32Array::from(colors);

        self.context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &colors_js_array,
            WebGlRenderingContext::STATIC_DRAW,
        );

        self.context.vertex_attrib_pointer_with_i32(1, 4, WebGlRenderingContext::FLOAT, false, 0, 0);
        self.context.enable_vertex_attrib_array(1);

        self.context.draw_arrays(WebGlRenderingContext::TRIANGLES, 0, (vertices.len() / 2) as i32);
    }
}

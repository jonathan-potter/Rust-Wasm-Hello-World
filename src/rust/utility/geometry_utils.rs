use wasm_bindgen::JsValue;
use web_sys::{WebGlBuffer, WebGlRenderingContext};

pub fn create_vertex_buffer(
    context: &WebGlRenderingContext,
    vertices: &[f32],
) -> Result<WebGlBuffer, JsValue> {
    let buffer = context.create_buffer().ok_or("Failed to create buffer")?;
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));
    unsafe {
        let vert_array = js_sys::Float32Array::view(vertices);
        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ARRAY_BUFFER,
            &vert_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }
    context.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, None);

    Ok(buffer)
}

pub fn create_indices_buffer(
    context: &WebGlRenderingContext,
    indices: &[u16],
) -> Result<WebGlBuffer, JsValue> {
    let buffer = context.create_buffer().ok_or("Failed to create buffer")?;
    context.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, Some(&buffer));
    unsafe {
        let ind_array = js_sys::Uint16Array::view(indices);
        context.buffer_data_with_array_buffer_view(
            WebGlRenderingContext::ELEMENT_ARRAY_BUFFER,
            &ind_array,
            WebGlRenderingContext::STATIC_DRAW,
        );
    }
    context.bind_buffer(WebGlRenderingContext::ELEMENT_ARRAY_BUFFER, None);

    Ok(buffer)
}

use web_sys::WebGlRenderingContext as Gl;
use web_sys::{WebGlShader, WebGlProgram};

pub fn compile_shader(
    gl: &Gl,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = gl.create_shader(shader_type).unwrap();
    gl.shader_source(&shader, source);
    gl.compile_shader(&shader);

    if gl.get_shader_parameter(&shader, Gl::COMPILE_STATUS).as_bool().unwrap_or(false) {
        Ok(shader)
    } else {
        Err(gl.get_shader_info_log(&shader).unwrap_or_else(|| String::from("Unknown error")))
    }
}

pub fn link_program(
    gl: &Gl,
    vertex_shader: &WebGlShader,
    fragment_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = gl.create_program().unwrap();
    gl.attach_shader(&program, vertex_shader);
    gl.attach_shader(&program, fragment_shader);
    gl.link_program(&program);

    if gl.get_program_parameter(&program, Gl::LINK_STATUS).as_bool().unwrap_or(false) {
        Ok(program)
    } else {
        Err(gl.get_program_info_log(&program).unwrap_or_else(|| String::from("Unknown error")))
    }
}

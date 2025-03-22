use std::ffi::CString;
use std::str;

use gl::types::{GLchar, GLint, GLuint};
use nalgebra::{Matrix2, Matrix2x3, Matrix2x4, Matrix3, Matrix3x2, Matrix3x4, Matrix4, Matrix4x2, Matrix4x3, Vector2, Vector3, Vector4};

/// A simple OpenGL shader program ```program: GLuint``` wrapper.
pub struct Shader {
    program: GLuint,
}

impl Shader {
    fn load_shader(source: &str, path: &str, typename: &str, type_: u32) -> GLuint {
        unsafe {
            let shader = gl::CreateShader(type_);
            gl::ShaderSource(shader, 1, &CString::new(source.as_bytes()).unwrap().as_ptr(), std::ptr::null());
            gl::CompileShader(shader);

            let mut log_length: GLint = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut log_length);

            let mut log: Vec<u8> = vec![0; log_length as usize];
            gl::GetShaderInfoLog(shader, log_length, std::ptr::null_mut(), log.as_mut_ptr() as *mut GLchar);

            let log = std::str::from_utf8(&log).unwrap();

            let mut success: GLint = 0;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);

            if success == gl::FALSE as GLint {
                gl::DeleteShader(shader);

                panic!(
                    "Failed to compile {} shader at: {}. Error: {}.",
                    typename,
                    path,
                    log
                );
            }

            shader
        }
    }
    fn delete_shaders(vertex_shader: GLuint, fragment_shader: GLuint) {
        unsafe {
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        }
    }

    /// Loads vertex and fragment shaders from ```vertex_path``` and ```fragment_path```.
    pub fn new(vertex_path: &str, fragment_path: &str) -> Self {
        let vertex_source = std::fs::read_to_string(vertex_path);
        if let Err(error) = vertex_source {
            panic!("Failed to read vertex shader source at: {}. Error: {}", vertex_path, error);
        }

        let fragment_source = std::fs::read_to_string(fragment_path);
        if let Err(error) = fragment_source {
            panic!("Failed to read fragment shader source at: {}. Error: {}", fragment_path, error);
        }

        unsafe {
            let vertex_shader = Self::load_shader(
                vertex_source.unwrap().as_str(),
                vertex_path,
                "vertex",
                gl::VERTEX_SHADER
            );
            let fragment_shader = Self::load_shader(
                fragment_source.unwrap().as_str(),
                fragment_path,
                "fragment",
                gl::FRAGMENT_SHADER
            );

            let program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader);
            gl::AttachShader(program, fragment_shader);
            gl::LinkProgram(program);

            let mut log_length: GLint = 0;
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut log_length);

            let mut log: Vec<u8> = vec![0; log_length as usize];
            gl::GetProgramInfoLog(program, log_length, std::ptr::null_mut(), log.as_mut_ptr() as *mut GLchar);

            let log = std::str::from_utf8(&log).unwrap();

            let mut success: GLint = 0;
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);

            if success == gl::FALSE as GLint {
                Self::delete_shaders(vertex_shader, fragment_shader);
                panic!(
                    "Failed to link program with shaders: Vertex({}), Fragment({}). Error: {}.",
                    vertex_path,
                    fragment_path,
                    log,
                );
            }

            Self::delete_shaders(vertex_shader, fragment_shader);
            Self { program }
        }
    }

    /// Makes OpenGL use current shader program.
    pub fn bind(&self) {
        unsafe { gl::UseProgram(self.program); }
    }
    /// Unbinds any shader programs from OpenGL's state.
    pub fn unbind() {
        unsafe { gl::UseProgram(0); }
    }

    fn get_uniform_location(&self, name: &str) -> GLint {
        unsafe { gl::GetUniformLocation(self.program, CString::new(name).unwrap().as_ptr() as *const GLchar) }
    }

    /// Sets boolean uniform at ```name``` location (aka. ```gl::Uniform1i```).  
    /// It's doesn't exist in gl crate, but using this function is just useful instead of converting bool to int manually.
    pub fn set_bool(&self, name: &str, value: bool) {
        unsafe { gl::Uniform1i(self.get_uniform_location(name), if value { 1 } else { 0 }); }
    }
    /// Sets integer uniform at ```name``` location (aka. ```gl::Uniform1i```).
    pub fn set_int(&self, name: &str, value: i32) {
        unsafe { gl::Uniform1i(self.get_uniform_location(name), value); }
    }
    /// Sets float uniform at ```name``` location (aka. ```gl::Uniform1f```).
    pub fn set_float(&self, name: &str, value: f32) {
        unsafe { gl::Uniform1f(self.get_uniform_location(name), value); }
    }

    /// Sets float 2D vector uniform at ```name``` location (aka. ```gl::Uniform2f```).
    pub fn set_vec2(&self, name: &str, value: &Vector2<f32>) {
        unsafe { gl::Uniform2f(self.get_uniform_location(name), value.x, value.y); }
    }
    /// Sets double 2D vector uniform at ```name``` location (aka. ```gl::Uniform2d```).
    pub fn set_dvec2(&self, name: &str, value: &Vector2<f64>) {
        unsafe { gl::Uniform2d(self.get_uniform_location(name), value.x, value.y); }
    }
    /// Sets integer 2D vector uniform at ```name``` location (aka. ```gl::Uniform2i```).
    pub fn set_ivec2(&self, name: &str, value: &Vector2<i32>) {
        unsafe { gl::Uniform2i(self.get_uniform_location(name), value.x, value.y); }
    }
    /// Sets unsigned int 2D vector uniform at ```name``` location (aka. ```gl::Uniform2ui```).
    pub fn set_uvec2(&self, name: &str, value: &Vector2<u32>) {
        unsafe { gl::Uniform2ui(self.get_uniform_location(name), value.x, value.y); }
    }

    /// Sets float 3D vector uniform at ```name``` location (aka. ```gl::Uniform3f```).
    pub fn set_vec3(&self, name: &str, value: &Vector3<f32>) {
        unsafe { gl::Uniform3f(self.get_uniform_location(name), value.x, value.y, value.z); }
    }
    /// Sets double 3D vector uniform at ```name``` location (aka. ```gl::Uniform3d```).
    pub fn set_dvec3(&self, name: &str, value: &Vector3<f64>) {
        unsafe { gl::Uniform3d(self.get_uniform_location(name), value.x, value.y, value.z); }
    }
    /// Sets integer 3D vector uniform at ```name``` location (aka. ```gl::Uniform3i```).
    pub fn set_ivec3(&self, name: &str, value: &Vector3<i32>) {
        unsafe { gl::Uniform3i(self.get_uniform_location(name), value.x, value.y, value.z); }
    }
    /// Sets unsigned int 3D vector uniform at ```name``` location (aka. ```gl::Uniform3ui```).
    pub fn set_uvec3(&self, name: &str, value: &Vector3<u32>) {
        unsafe { gl::Uniform3ui(self.get_uniform_location(name), value.x, value.y, value.z); }
    }

    /// Sets float 4D vector uniform at ```name``` location (aka. ```gl::Uniform4f```).
    pub fn set_vec4(&self, name: &str, value: &Vector4<f32>) {
        unsafe { gl::Uniform4f(self.get_uniform_location(name), value.x, value.y, value.z, value.w); }
    }
    /// Sets double 4D vector uniform at ```name``` location (aka. ```gl::Uniform4d```).
    pub fn set_dvec4(&self, name: &str, value: &Vector4<f64>) {
        unsafe { gl::Uniform4d(self.get_uniform_location(name), value.x, value.y, value.z, value.w); }
    }
    /// Sets integer 4D vector uniform at ```name``` location (aka. ```gl::Uniform4i```).
    pub fn set_ivec4(&self, name: &str, value: &Vector4<i32>) {
        unsafe { gl::Uniform4i(self.get_uniform_location(name), value.x, value.y, value.z, value.w); }
    }
    /// Sets unsigned int 4D vector uniform at ```name``` location (aka. ```gl::Uniform4ui```).
    pub fn set_uvec4(&self, name: &str, value: &Vector4<u32>) {
        unsafe { gl::Uniform4ui(self.get_uniform_location(name), value.x, value.y, value.z, value.w); }
    }

    /// Sets float 2x2 matrix uniform at ```name``` location (aka. ```gl::UniformMatrix2fv```).
    pub fn set_mat2(&self, name: &str, value: &Matrix2<f32>) {
        unsafe { gl::UniformMatrix2fv(self.get_uniform_location(name), 1, gl::FALSE, value.as_ptr()); }
    }
    /// Sets double 2x2 matrix uniform at ```name``` location (aka. ```gl::UniformMatrix2dv```).
    pub fn set_dmat2(&self, name: &str, value: &Matrix2<f64>) {
        unsafe { gl::UniformMatrix2dv(self.get_uniform_location(name), 1, gl::FALSE, value.as_ptr()); }
    }
    /// Sets float 2x3 matrix uniform at ```name``` location (aka. ```gl::UniformMatrix2x3fv```).
    pub fn set_mat2x3(&self, name: &str, value: &Matrix2x3<f32>) {
        unsafe { gl::UniformMatrix2x3fv(self.get_uniform_location(name), 1, gl::FALSE, value.as_ptr()); }
    }
    /// Sets double 2x3 matrix uniform at ```name``` location (aka. ```gl::UniformMatrix2x3dv```).
    pub fn set_dmat2x3(&self, name: &str, value: &Matrix2x3<f64>) {
        unsafe { gl::UniformMatrix2x3dv(self.get_uniform_location(name), 1, gl::FALSE, value.as_ptr()); }
    }
    /// Sets float 2x4 matrix uniform at ```name``` location (aka. ```gl::UniformMatrix2x4fv```).
    pub fn set_mat2x4(&self, name: &str, value: &Matrix2x4<f32>) {
        unsafe { gl::UniformMatrix2x4fv(self.get_uniform_location(name), 1, gl::FALSE, value.as_ptr()); }
    }
    /// Sets double 2x4 matrix uniform at ```name``` location (aka. ```gl::UniformMatrix2x4dv```).
    pub fn set_dmat2x4(&self, name: &str, value: &Matrix2x4<f64>) {
        unsafe { gl::UniformMatrix2x4dv(self.get_uniform_location(name), 1, gl::FALSE, value.as_ptr()); }
    }

    /// Sets float 3x3 matrix uniform at ```name``` location (aka. ```gl::UniformMatrix3fv```).
    pub fn set_mat3(&self, name: &str, value: &Matrix3<f32>) {
        unsafe { gl::UniformMatrix3fv(self.get_uniform_location(name), 1, gl::FALSE, value.as_ptr()); }
    }
    /// Sets double 3x3 matrix uniform at ```name``` location (aka. ```gl::UniformMatrix3dv```).
    pub fn set_dmat3(&self, name: &str, value: &Matrix3<f64>) {
        unsafe { gl::UniformMatrix3dv(self.get_uniform_location(name), 1, gl::FALSE, value.as_ptr()); }
    }
    /// Sets float 3x2 matrix uniform at ```name``` location (aka. ```gl::UniformMatrix3x2fv```).
    pub fn set_mat3x2(&self, name: &str, value: &Matrix3x2<f32>) {
        unsafe { gl::UniformMatrix3x2fv(self.get_uniform_location(name), 1, gl::FALSE, value.as_ptr()); }
    }
    /// Sets double 3x2 matrix uniform at ```name``` location (aka. ```gl::UniformMatrix3x2dv```).
    pub fn set_dmat3x2(&self, name: &str, value: &Matrix3x2<f64>) {
        unsafe { gl::UniformMatrix3x2dv(self.get_uniform_location(name), 1, gl::FALSE, value.as_ptr()); }
    }
    /// Sets float 3x4 matrix uniform at ```name``` location (aka. ```gl::UniformMatrix3x4fv```).
    pub fn set_mat3x4(&self, name: &str, value: &Matrix3x4<f32>) {
        unsafe { gl::UniformMatrix3x4fv(self.get_uniform_location(name), 1, gl::FALSE, value.as_ptr()); }
    }
    /// Sets double 3x4 matrix uniform at ```name``` location (aka. ```gl::UniformMatrix3x4dv```).
    pub fn set_dmat3x4(&self, name: &str, value: &Matrix3x4<f64>) {
        unsafe { gl::UniformMatrix3x4dv(self.get_uniform_location(name), 1, gl::FALSE, value.as_ptr()); }
    }

    /// Sets float 4x4 matrix uniform at ```name``` location (aka. ```gl::UniformMatrix4fv```).
    pub fn set_mat4(&self, name: &str, value: &Matrix4<f32>) {
        unsafe { gl::UniformMatrix4fv(self.get_uniform_location(name), 1, gl::FALSE, value.as_ptr()); }
    }
    /// Sets double 4x4 matrix uniform at ```name``` location (aka. ```gl::UniformMatrix4dv```).
    pub fn set_dmat4(&self, name: &str, value: &Matrix4<f64>) {
        unsafe { gl::UniformMatrix4dv(self.get_uniform_location(name), 1, gl::FALSE, value.as_ptr()); }
    }
    /// Sets float 4x2 matrix uniform at ```name``` location (aka. ```gl::UniformMatrix4x2fv```).
    pub fn set_mat4x2(&self, name: &str, value: &Matrix4x2<f32>) {
        unsafe { gl::UniformMatrix4x2fv(self.get_uniform_location(name), 1, gl::FALSE, value.as_ptr()); }
    }
    /// Sets double 4x2 matrix uniform at ```name``` location (aka. ```gl::UniformMatrix4x2dv```).
    pub fn set_dmat4x2(&self, name: &str, value: &Matrix4x2<f64>) {
        unsafe { gl::UniformMatrix4x2dv(self.get_uniform_location(name), 1, gl::FALSE, value.as_ptr()); }
    }
    /// Sets float 4x3 matrix uniform at ```name``` location (aka. ```gl::UniformMatrix4x3fv```).
    pub fn set_mat4x3(&self, name: &str, value: &Matrix4x3<f32>) {
        unsafe { gl::UniformMatrix4x3fv(self.get_uniform_location(name), 1, gl::FALSE, value.as_ptr()); }
    }
    /// Sets double 4x3 matrix uniform at ```name``` location (aka. ```gl::UniformMatrix4x3dv```).
    pub fn set_dmat4x3(&self, name: &str, value: &Matrix4x3<f64>) {
        unsafe { gl::UniformMatrix4x3dv(self.get_uniform_location(name), 1, gl::FALSE, value.as_ptr()); }
    }
}
impl Drop for Shader {
    fn drop(&mut self) {
        unsafe { gl::DeleteProgram(self.program); }
    }
}
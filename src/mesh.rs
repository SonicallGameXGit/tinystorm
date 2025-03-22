use std::f32::consts::PI;
use gl::types::{GLenum, GLint, GLsizei, GLsizeiptr, GLuint};

/// Just a vertex attribute types enum. Float, Vec2, etc.
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Attribute {
    Float,
    Vec2,
    Vec3,
    Vec4,

    Double,
    DVec2,
    DVec3,
    DVec4,

    Int,
    IVec2,
    IVec3,
    IVec4,

    UInt,
    UVec2,
    UVec3,
    UVec4,
}
impl Attribute {
    /// Returns size in bytes of current attribute.
    /// # Example
    /// **[Attribute::Float] = 4 bytes**  
    /// **[Attribute::DVec2] = 16 bytes**
    pub const fn size_in_bytes(&self) -> usize {
        match self {
            Attribute::Float => std::mem::size_of::<f32>(),
            Attribute::Vec2 => 2 * std::mem::size_of::<f32>(),
            Attribute::Vec3 => 3 * std::mem::size_of::<f32>(),
            Attribute::Vec4 => 4 * std::mem::size_of::<f32>(),

            Attribute::Double => std::mem::size_of::<f64>(),
            Attribute::DVec2 => 2 * std::mem::size_of::<f64>(),
            Attribute::DVec3 => 3 * std::mem::size_of::<f64>(),
            Attribute::DVec4 => 4 * std::mem::size_of::<f64>(),

            Attribute::Int => std::mem::size_of::<i32>(),
            Attribute::IVec2 => 2 * std::mem::size_of::<i32>(),
            Attribute::IVec3 => 3 * std::mem::size_of::<i32>(),
            Attribute::IVec4 => 4 * std::mem::size_of::<i32>(),

            Attribute::UInt => std::mem::size_of::<u32>(),
            Attribute::UVec2 => 2 * std::mem::size_of::<u32>(),
            Attribute::UVec3 => 3 * std::mem::size_of::<u32>(),
            Attribute::UVec4 => 4 * std::mem::size_of::<u32>(),
        }
    }
}

/// A system for creating custom layouts for meshes.
#[derive(Default)]
pub struct Layout {
    attributes: Vec<Attribute>
}
impl Layout {
    /// Best for 3D games with more improved graphics.
    /// # Layout
    /// position: [Attribute::Vec3]  
    /// uv: [Attribute::Vec2]  
    /// normal: [Attribute::Vec3]
    pub fn default_3d() -> Self {
        Self { attributes: vec![Attribute::Vec3, Attribute::Vec2, Attribute::Vec3] }
    }
    /// Best for 3D games with workbench graphics.
    /// # Layout
    /// position: [Attribute::Vec3]  
    /// normal: [Attribute::Vec3]
    pub fn simple_3d() -> Self {
        Self { attributes: vec![Attribute::Vec3, Attribute::Vec3] }
    }

    /// Best for 2D games with simple graphics.
    /// # Layout
    /// position: [Attribute::Vec2]  
    /// uv: [Attribute::Vec2]
    pub fn default_2d() -> Self {
        Self { attributes: vec![Attribute::Vec2, Attribute::Vec2] }
    }
    /// Best for 2D games with workbench graphics.
    /// position: [Attribute::Vec2]
    pub fn basic_2d() -> Self {
        Self { attributes: vec![Attribute::Vec2] }
    }
    
    /// Set next vertex attribute.
    /// # Example
    /// ```
    /// Layout::default()
    ///     .next_attribute(Attribute::Vec3) // Position [layout(location=0)]
    ///     .next_attribute(Attribute::Vec2) // UV [layout(location=1)]
    /// ```
    pub fn next_attribute(mut self, attribute: Attribute) -> Self {
        self.attributes.push(attribute);
        self
    }
    /// Returns all attributes built using [Layout::next_attribute()]
    pub fn attributes(&self) -> &[Attribute] {
        &self.attributes
    }
}

/// Just a mesh you can render on your screen.
/// # Example
/// ```rust
/// use tinystorm::{window::WindowBuilder, mesh::{Layout, Mesh}, gl};
/// 
/// let mut window = WindowBuilder::default().build();
/// let mesh = Mesh::new::<f32>(&[
///     -0.5, -0.5,
///      0.5, -0.5,
///      0.5,  0.5,
///     -0.5,  0.5,
/// ], &Layout::basic_2d(), gl::TRIANGLE_FAN);
/// 
/// while window.is_running() {
///     window.poll_events();
///     unsafe { gl::Clear(gl::COLOR_BUFFER_BIT); }
/// 
///     mesh.draw();
///     window.swap_buffers();
/// }
/// ```
#[derive(Clone)]
pub struct Mesh {
    vao: GLuint,
    vbo: GLuint,

    num_vertices: GLsizei,
    render_mode: GLenum,
}
impl Mesh {
    /// Returns a sphere with certain number of horizontal and vertical divisions in [Layout::simple_3d] layout.  
    /// Origin is located at it's center. Radius is 1.0
    pub fn simple_sphere(x_divisions: usize, y_divisions: usize) -> Self {
        let mut vertices = Vec::new();
        
        for i in 0..=y_divisions {
            let latitude = PI * (i as f32 / y_divisions as f32);
            let sin_latitude = latitude.sin();
            let cos_latitude = latitude.cos();
    
            for j in 0..=x_divisions {
                let longitude = 2.0 * PI * (j as f32 / x_divisions as f32);
                let sin_longitude = longitude.sin();
                let cos_longitude = longitude.cos();
    
                let x = sin_latitude * cos_longitude;
                let y = sin_latitude * sin_longitude;
                let z = cos_latitude;
    
                let nx = x;
                let ny = y;
                let nz = z;
    
                vertices.push(x);
                vertices.push(y);
                vertices.push(z);
                vertices.push(nx);
                vertices.push(ny);
                vertices.push(nz);
            }
        }
    
        let mut indices = Vec::new();
        for i in 0..y_divisions {
            for j in 0..x_divisions {
                let current = i * (x_divisions + 1) + j;
                let next = current + x_divisions + 1;
    
                indices.push(current);
                indices.push(next);
                indices.push(current + 1);
    
                indices.push(next);
                indices.push(next + 1);
                indices.push(current + 1);
            }
        }
    
        let mut result = Vec::new();
        for index in indices {
            let base = index * 6;
            result.extend_from_slice(&vertices[base..base + 6]);
        }
    
        Self::new::<f32>(&result, &Layout::simple_3d(), gl::TRIANGLES)
    }
    /// Returns a sphere with certain number of horizontal and vertical divisions in [Layout::default_3d] layout.  
    /// Origin is located at it's center. Radius is 1.0
    pub fn default_sphere(x_divisions: usize, y_divisions: usize) -> Self {
        let mut vertices = Vec::new();
        
        for i in 0..=y_divisions {
            let latitude = PI * (i as f32 / y_divisions as f32);
            let sin_latitude = latitude.sin();
            let cos_latitude = latitude.cos();
    
            for j in 0..=x_divisions {
                let longitude = 2.0 * PI * (j as f32 / x_divisions as f32);
                let sin_longitude = longitude.sin();
                let cos_longitude = longitude.cos();
    
                let x = sin_latitude * cos_longitude;
                let y = sin_latitude * sin_longitude;
                let z = cos_latitude;

                let u = j as f32 / x_divisions as f32;
                let v = i as f32 / y_divisions as f32;
    
                let nx = x;
                let ny = y;
                let nz = z;
    
                vertices.push(x);
                vertices.push(y);
                vertices.push(z);
                vertices.push(u);
                vertices.push(v);
                vertices.push(nx);
                vertices.push(ny);
                vertices.push(nz);
            }
        }
    
        let mut indices = Vec::new();
        for i in 0..y_divisions {
            for j in 0..x_divisions {
                let current = i * (x_divisions + 1) + j;
                let next = current + x_divisions + 1;
    
                indices.push(current);
                indices.push(next);
                indices.push(current + 1);
    
                indices.push(next);
                indices.push(next + 1);
                indices.push(current + 1);
            }
        }
    
        let mut result = Vec::new();
        for index in indices {
            let base = index * 8;
            result.extend_from_slice(&vertices[base..base + 8]);
        }
    
        Self::new::<f32>(&result, &Layout::default_3d(), gl::TRIANGLES)
    }
    /// Returns a cube in [Layout::default_3d] layout.  
    /// Origin is located at it's center. Half-Size is 1.0
    pub fn default_cube() -> Self {
        Self::new::<f32>(&[
            // Back face
            1.0, -1.0, -1.0, 0.0, 0.0, 0.0, 0.0, -1.0,
            -1.0, -1.0, -1.0, 1.0, 0.0, 0.0, 0.0, -1.0,
            1.0, 1.0, -1.0, 0.0, 1.0, 0.0, 0.0, -1.0,

            -1.0, 1.0, -1.0, 1.0, 1.0, 0.0, 0.0, -1.0,
            1.0, 1.0, -1.0, 0.0, 1.0, 0.0, 0.0, -1.0,
            -1.0, -1.0, -1.0, 1.0, 0.0, 0.0, 0.0, -1.0,

            // Front face
            -1.0, -1.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
            1.0, -1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0,
            -1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0,

            1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0,
            -1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0, 1.0,
            1.0, -1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0,

            // Left face
            -1.0, 1.0, 1.0, 0.0, 0.0, -1.0, 0.0, 0.0,
            -1.0, 1.0, -1.0, 1.0, 0.0, -1.0, 0.0, 0.0,
            -1.0, -1.0, 1.0, 0.0, 1.0, -1.0, 0.0, 0.0,

            -1.0, -1.0, -1.0, 1.0, 1.0, -1.0, 0.0, 0.0,
            -1.0, -1.0, 1.0, 0.0, 1.0, -1.0, 0.0, 0.0,
            -1.0, 1.0, -1.0, 1.0, 0.0, -1.0, 0.0, 0.0,

            // Right face
            1.0, 1.0, -1.0, 0.0, 0.0, 1.0, 0.0, 0.0,
            1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0,
            1.0, -1.0, -1.0, 0.0, 1.0, 1.0, 0.0, 0.0,

            1.0, -1.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0,
            1.0, -1.0, -1.0, 0.0, 1.0, 1.0, 0.0, 0.0,
            1.0, 1.0, 1.0, 1.0, 0.0, 1.0, 0.0, 0.0,

            // Bottom face
            -1.0, -1.0, -1.0, 0.0, 0.0, 0.0, -1.0, 0.0,
            1.0, -1.0, -1.0, 1.0, 0.0, 0.0, -1.0, 0.0,
            -1.0, -1.0, 1.0, 0.0, 1.0, 0.0, -1.0, 0.0,

            1.0, -1.0, 1.0, 1.0, 1.0, 0.0, -1.0, 0.0,
            -1.0, -1.0, 1.0, 0.0, 1.0, 0.0, -1.0, 0.0,
            1.0, -1.0, -1.0, 1.0, 0.0, 0.0, -1.0, 0.0,

            // Top face
            -1.0, 1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0,
            1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0,
            -1.0, 1.0, -1.0, 0.0, 1.0, 0.0, 1.0, 0.0,

            1.0, 1.0, -1.0, 1.0, 1.0, 0.0, 1.0, 0.0,
            -1.0, 1.0, -1.0, 0.0, 1.0, 0.0, 1.0, 0.0,
            1.0, 1.0, 1.0, 1.0, 0.0, 0.0, 1.0, 0.0,
        ], &Layout::default_3d(), gl::TRIANGLES)
    }
    /// Returns a cube in [Layout::simple_3d] layout.  
    /// Origin is located at it's center. Half-Size is 1.0
    pub fn simple_cube() -> Self {
        Self::new::<f32>(&[
            // Back face
            1.0, -1.0, -1.0, 0.0, 0.0, -1.0,
            -1.0, -1.0, -1.0, 0.0, 0.0, -1.0,
            1.0, 1.0, -1.0, 0.0, 0.0, -1.0,

            -1.0, 1.0, -1.0, 0.0, 0.0, -1.0,
            1.0, 1.0, -1.0, 0.0, 0.0, -1.0,
            -1.0, -1.0, -1.0, 0.0, 0.0, -1.0,

            // Front face
            -1.0, -1.0, 1.0, 0.0, 0.0, 1.0,
            1.0, -1.0, 1.0, 0.0, 0.0, 1.0,
            -1.0, 1.0, 1.0, 0.0, 0.0, 1.0,

            1.0, 1.0, 1.0, 0.0, 0.0, 1.0,
            -1.0, 1.0, 1.0, 0.0, 0.0, 1.0,
            1.0, -1.0, 1.0, 0.0, 0.0, 1.0,

            // Left face
            -1.0, 1.0, 1.0, -1.0, 0.0, 0.0,
            -1.0, 1.0, -1.0, -1.0, 0.0, 0.0,
            -1.0, -1.0, 1.0, -1.0, 0.0, 0.0,

            -1.0, -1.0, -1.0, -1.0, 0.0, 0.0,
            -1.0, -1.0, 1.0, -1.0, 0.0, 0.0,
            -1.0, 1.0, -1.0, -1.0, 0.0, 0.0,

            // Right face
            1.0, 1.0, -1.0, 1.0, 0.0, 0.0,
            1.0, 1.0, 1.0, 1.0, 0.0, 0.0,
            1.0, -1.0, -1.0, 1.0, 0.0, 0.0,

            1.0, -1.0, 1.0, 1.0, 0.0, 0.0,
            1.0, -1.0, -1.0, 1.0, 0.0, 0.0,
            1.0, 1.0, 1.0, 1.0, 0.0, 0.0,

            // Bottom face
            -1.0, -1.0, -1.0, 0.0, -1.0, 0.0,
            1.0, -1.0, -1.0, 0.0, -1.0, 0.0,
            -1.0, -1.0, 1.0, 0.0, -1.0, 0.0,

            1.0, -1.0, 1.0, 0.0, -1.0, 0.0,
            -1.0, -1.0, 1.0, 0.0, -1.0, 0.0,
            1.0, -1.0, -1.0, 0.0, -1.0, 0.0,

            // Top face
            -1.0, 1.0, 1.0, 0.0, 1.0, 0.0,
            1.0, 1.0, 1.0, 0.0, 1.0, 0.0,
            -1.0, 1.0, -1.0, 0.0, 1.0, 0.0,

            1.0, 1.0, -1.0, 0.0, 1.0, 0.0,
            -1.0, 1.0, -1.0, 0.0, 1.0, 0.0,
            1.0, 1.0, 1.0, 0.0, 1.0, 0.0,
        ], &Layout::simple_3d(), gl::TRIANGLES)
    }

    /// Creates a mesh with your vertices, custom vertex layout and render mode.
    /// # Example
    /// ```
    /// let mesh = Mesh::new::<f32>(&[
    ///     -0.5, -0.5,
    ///      0.5, -0.5,
    ///      0.5,  0.5,
    ///     -0.5,  0.5,
    /// ], &Layout::basic_2d(), gl::TRIANGLE_FAN);
    /// ```
    pub fn new<T>(vertices: &[T], layout: &Layout, render_mode: GLenum) -> Self {
        let mut vao: GLuint = 0;
        let mut vbo: GLuint = 0;

        let mut stride = 0;
        for attribute in layout.attributes() {
            stride += attribute.size_in_bytes();
        }

        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);

            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, std::mem::size_of_val(vertices) as GLsizeiptr, vertices.as_ptr() as *const _, gl::STATIC_DRAW);

            let mut offset: GLuint = 0;
            for (i, attribute) in layout.attributes().iter().enumerate() {
                let index = i as GLuint;
                gl::EnableVertexAttribArray(index);

                match attribute {
                    Attribute::Float | Attribute::Vec2 | Attribute::Vec3 | Attribute::Vec4 => {
                        gl::VertexAttribPointer(
                            i as GLuint,
                            *attribute as GLint + 1,
                            gl::FLOAT,
                            gl::FALSE,
                            stride as GLsizei,
                            offset as *const _,
                        );
                    }
                    Attribute::Double | Attribute::DVec2 | Attribute::DVec3 | Attribute::DVec4 => {
                        gl::VertexAttribLPointer(
                            i as GLuint,
                            *attribute as GLint - Attribute::Double as GLint + 1,
                            gl::FLOAT,
                            stride as GLsizei,
                            offset as *const _,
                        );
                    }
                    Attribute::Int | Attribute::IVec2 | Attribute::IVec3 | Attribute::IVec4 => {
                        gl::VertexAttribIPointer(
                            i as GLuint,
                            *attribute as GLint - Attribute::Int as GLint + 1,
                            gl::INT,
                            stride as GLsizei,
                            offset as *const _,
                        );
                    }
                    Attribute::UInt | Attribute::UVec2 | Attribute::UVec3 | Attribute::UVec4 => {
                        gl::VertexAttribIPointer(
                            i as GLuint,
                            *attribute as GLint - Attribute::UInt as GLint + 1,
                            gl::UNSIGNED_INT,
                            stride as GLsizei,
                            offset as *const _,
                        );
                    }
                }

                offset += attribute.size_in_bytes() as GLuint;
            }
        }

        Self { vao, vbo, num_vertices: (std::mem::size_of_val(vertices) / stride) as GLsizei, render_mode }
    }

    /// Draws the mesh itself.
    /// # Example
    /// ```
    /// // You must clear the framebuffer before rendering meshes on it,
    /// // else your mesh won't appear on screen.
    /// unsafe { gl::Clear(gl::COLOR_BUFFER_BIT); }
    /// ...
    /// mesh.draw();
    /// other_mesh.draw();
    /// ...
    /// // Swap buffers only after all meshes are drawn to see them on your screen.
    /// window.swap_buffers();
    /// ```
    pub fn draw(&self) {
        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(self.render_mode, 0, self.num_vertices);
        }
    }
}
impl Drop for Mesh {
    /// You don't need to manually free OpenGL resources, it's done automatically.
    fn drop(&mut self) {
        unsafe {
            gl::DeleteVertexArrays(1, &self.vao);
            gl::DeleteBuffers(1, &self.vbo);
        }
    }
}
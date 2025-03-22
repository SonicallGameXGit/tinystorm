//! # Introduction
//! ## Window
//! ### Creating the Window
//! To create a simple window you can do:
//! 
//! ```rust
//! use tinystorm::window::{Window, WindowBuilder};
//! 
//! let mut window: Window = WindowBuilder::default().build();
//! while window.is_running() {
//!     window.poll_events();
//!     window.swap_buffers();
//! }
//! ```
//! 
//! But if you want more percise control over the window creation, you can do:
//! 
//! ```rust
//! let mut window: Window = WindowBuilder::default()
//!     .with_size(720, 480) // Default: 960, 540
//!     .with_title("Ne chitay eto :}") // Default: "Titled window, lol"
//!     .with_vsync(false) // Default: true
//!     .with_max_fps(144 * 5) // Default: WindowBuilder::NO_MAX_FPS
//!     .with_msaa(4) // (aka. 4 samples per pixel) Default: WindowBuilder::NO_MSAA
//!     .build();
//! ```
//! 
//! Let's breakdown trough each setting:  
//!  - ``with_size(width: u32, height: u32)`` - sets size of the window.  
//!  - ``with_title(title: String)`` - sets the title of the window.  
//!  - ``with_vsync(vsync: bool)`` - enables/disables vertical synchronization  
//!    (if your display refresh rate is 60, FPS would clamp to it).  
//!  - ``with_max_fps(max_fps: u32)`` - sets maximum FPS.  
//!    But if vsync enabled and display refresh rate is lower than max FPS - it would clamp to vsync instead.  
//!    You can use ``WindowBuilder::NO_MAX_FPS`` for better readability.  
//!  - ``with_msaa(msaa_quality: u32)`` - if greater than 0, enables msaa with ``msaa_quality`` samples.  
//!    You can use ``WindowBuilder::NO_MSAA`` for better readability.
//! ### Working with the Window
//! Here's a simple example of working with certain parts of the Window
//! 
//! ```rust
//! use tinystorm::{window::{Window, WindowBuilder}, glfw::Key, glfw::MouseButton};
//! 
//! let mut window: Window = WindowBuilder::default().build();
//! while window.is_running() {
//!     window.poll_events();
//! 
//!     // Keyboard input handling
//!     if window.is_key_pressed(Key::Space) {
//!         println!("It would print this message each frame while Space key is pressed.");
//!     }
//!     if window.is_key_just_pressed(Key::Escape) {
//!         println!("It would print this message only at the frame when Escape key was pressed.");
//!     }
//! 
//!     // Mouse button input handling
//!     if window.is_mouse_button_pressed(MouseButton::Left) {
//!         println!("The same thing as window.is_key_pressed but with mouse buttons instead.");
//!     }
//!     if window.is_mouse_button_just_pressed(MouseButton::Middle) {
//!         println!("The same thing as window.is_key_just_pressed but with mouse buttons instead.");
//!     }
//! 
//!     // Mouse cursor input handling
//!     println!(
//!         "Mouse position in pixels from top-left corner of the window: X {}, Y {}.",
//!         window.get_mouse_x(),
//!         window.get_mouse_y(),
//!     );
//!     println!(
//!         "Mouse movement on this frame: X {}, Y {}.",
//!         window.get_mouse_dx(),
//!         window.get_mouse_dy(),
//!     );
//!     println!("Is mouse grabbed: {}.", window.is_mouse_grabbed());
//! 
//!     // Working with time
//!     // ps. You can also do window.get_delta_raw().as_secs_f32()
//!     println!("Current frame delta time in seconds: {}.", window.get_delta());
//! 
//!     // Getting window info
//!     println!("Window size in pixels: Width {}, Height {}.", window.get_width(), window.get_height());
//!     println!(
//!         "Window position in pixels from top-left corner: X {}, Y {}.",
//!         window.get_x(),
//!         window.get_y(),
//!     );
//! 
//!     println!("Window title: {}.", window.get_title());
//!     println!("Window aspect: {}.", window.get_aspect());
//! 
//!     window.swap_buffers();
//! }
//! ```
//! 
//! Example usage
//! 
//! ```rust
//! if window.is_key_just_pressed(Key::E) {
//!     window.toggle_mouse(); // If mouse is grabbed - it would be released, else - it would be grabbed back.
//! }
//! if window.is_key_just_pressed(Key::Q) {
//!     window.grab_mouse();
//! }
//! if window.is_key_just_pressed(Key::W) {
//!     window.release_mouse();
//! }
//! if window.is_key_just_pressed(Key::R) {
//!     window.set_title(String::from("Oh no! You've just changed the window title!"));
//! }
//! if window.is_key_pressed(Key::T) {
//!     // ps. sets the window position from top-left corner.
//!     window.set_position(50, 100); // You can set x/y separately using window.set_x/y(value);
//! }
//! if window.is_key_pressed(Key::Y) {
//!     // ps. sets the window size.
//!     window.set_size(100, 300); // You can set width/height separately using window.set_width/height(value);
//! }
//! ```
//! 
//! ## Mesh
//! ### Creating the Mesh
//! To create a simple mesh you can do:
//! 
//! ```rust
//! use tinystorm::{mesh::{Layout, Mesh}, gl};
//! 
//! // Create the window first
//! let mesh = Mesh::new::<f32>(&[
//!     -0.5, -0.5,
//!      0.5, -0.5,
//!      0.0,  0.5,
//! ], &Layout::basic_2d(), gl::TRIANGLES); // A simple triangle mesh
//! ```
//! 
//! You can also define your own layout:
//! 
//! ```rust
//! use tinystorm::{mesh::{Attribute, Layout, Mesh}, gl};
//! 
//! // Create the window first
//! //     X,    Y,      U,   V,      R,   G,   B,
//! let mesh = Mesh::new::<f32>(&[
//!     -0.5, -0.5,    0.0, 0.0,    1.0, 0.0, 0.0,
//!      0.5, -0.5,    1.0, 0.0,    0.0, 1.0, 0.0,
//!      0.5,  0.5,    1.0, 1.0,    0.0, 0.0, 1.0,
//!     -0.5,  0.5,    0.0, 1.0,    1.0, 1.0, 0.0,
//! ], &Layout::default()
//!     .next_attribute(Attribute::Vec2) // XY. In GLSL: layout(location=0) in vec2 a_Position;
//!     .next_attribute(Attribute::Vec2) // UV. In GLSL: layout(location=1) in vec2 a_TexCoord;
//!     .next_attribute(Attribute::Vec3) // RGB. In GLSL: layout(location=2) in vec3 a_Color;
//! , gl::TRIANGLE_FAN);
//! ```
//! 
//! Also you're allowed to send custom vertex structs!
//! 
//! ```rust
//! #[repr(C)]
//! struct Vertex {
//!     position: (f32, f32),
//!     texcoord: (f32, f32),
//!     color: (f32, f32, f32),
//! }
//! impl Vertex {
//!     pub fn new(position: (f32, f32), texcoord: (f32, f32), color: (f32, f32, f32)) -> Self {
//!         Self { position, texcoord, color }
//!     }
//! }
//! 
//! let mesh = Mesh::new::<Vertex>(&[ // IMPORTANT! You need to declare ::<Vertex> for proper data packing
//!     Vertex::new((-0.5, -0.5), (0.0, 0.0), (1.0, 0.0, 0.0)),
//!     Vertex::new(( 0.5, -0.5), (1.0, 0.0), (0.0, 1.0, 0.0)),
//!     Vertex::new(( 0.5,  0.5), (1.0, 1.0), (0.0, 0.0, 1.0)),
//!     Vertex::new((-0.5,  0.5), (0.0, 1.0), (1.0, 1.0, 0.0)),
//! ], &Layout::default()
//!     .next_attribute(Attribute::Vec2) // XY. In GLSL: layout(location=0) in vec2 a_Position;
//!     .next_attribute(Attribute::Vec2) // UV. In GLSL: layout(location=1) in vec2 a_TexCoord;
//!     .next_attribute(Attribute::Vec3) // RGB. In GLSL: layout(location=2) in vec3 a_Color;
//! , gl::TRIANGLE_FAN);
//! ```
//! 
//! ### Rendering the mesh
//! To render the mesh you can just call ``yourmesh.draw();``  
//!   
//! But it's not enough, to see anything on your screen you need to call:  
//! ``unsafe { gl::Clear(gl::COLOR_BUFFER_BIT); }``  
//! Or, if you want to have a 3D game:  
//! ``unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT); }``
//! #### Example:
//! ```rust
//! while window.is_running() {
//!     window.poll_events();
//!     unsafe { gl::Clear(gl::COLOR_BUFFER_BIT); }
//! 
//!     mesh1.draw();
//!     mesh2.draw();
//!     // etc. (It's allowed to draw the same mesh multiple times with no issues!)
//! 
//!     window.swap_buffers();
//! }
//! ```
//! 
//! ## Shader
//! ### Loading shaders
//! To load vertex and fragment shaders from files you can do:
//! ```rust
//! use tinystorm::shader::Shader;
//! 
//! // Create the window first.
//! //                       Vertex shader path            Fragment shader path
//! let shader = Shader::new("./assets/shaders/test.vert", "./assets/shaders/test.frag");
//! ```
//! 
//! ### Using shaders
//! ```rust
//! while window.is_running() {
//!     window.poll_events();
//!     unsafe { gl::Clear(gl::COLOR_BUFFER_BIT); }
//! 
//!     shader.bind();
//!     shader.set_float("u_Aspect", window.get_aspect()); // There's many other uniform types you can use.
//! 
//!     // If you'll draw mesh while certain shader is bound, this shader would apply to the mesh.
//!     mesh.draw(); // In this case there's "shader" shader bound.
//!     Shader::unbind(); // You can use that if you really need to have no shaders bound at all.
//! 
//!     mesh.draw(); // In this case there's no shader bound.
//!     window.swap_buffers();
//! }
//! ```
//! 
//! ## Texture
//! ### Loading textures
//! To load texture from file you can do:
//! ```rust
//! use tinystorm::{texture::Texture, gl};
//! 
//! // Create the window first.
//! let texture = Texture::load_from_file("./assets/textures/super_mario.png", gl::NEAREST, gl::CLAMP_TO_EDGE);
//! ```
//! 
//! ### Using textures
//! To use texture you need to bind it before rendering the mesh to a certain texture slot.
//! #### Example
//! 
//! ```rust
//! while window.is_running() {
//!     window.poll_events();
//!     unsafe { gl::Clear(gl::COLOR_BUFFER_BIT); }
//! 
//!     shader.bind();
//!     shader.set_int("u_ColorSampler", 0); // Just bind uniform sampler2D u_ColorSampler; to texture slot 0.
//!     
//!     texture.bind(0); // Binding texture to a slot 0.
//!     mesh.draw();
//!     Texture::unbind(); // You can use that if you really need to have no textures bound at all.
//! }
//! ```

pub mod window;
pub mod shader;
pub mod mesh;
pub mod texture;

pub use glfw;
pub use gl;

use std::time::{Duration, Instant};

use glfw::{self, Context};
use spin_sleep::SpinSleeper;

/// It's just a simple GLFW window holder with custom basic input system.
///
/// # Example
/// ```rust
/// use tinystorm::window::{WindowBuilder};
///
/// let mut window = WindowBuilder::default()
///     .with_size(800, 600)
///     .with_title("My Window")
///     .with_vsync(false)
///     .with_max_fps(144 * 5)
///     .with_msaa(4)
///     .build();
///
/// while window.is_running() {
///     window.poll_events();
///     // Render your scene here
///     window.swap_buffers();
/// }
/// ```
pub struct Window {
    glfw: glfw::Glfw,

    /// GLFW window handle you can use for more precise control of your application.
    pub handle: glfw::PWindow,
    /// Current frame events you can use like a custom event handler. Updated only after [Self::poll_events] is called.
    pub events: glfw::GlfwReceiver<(f64, glfw::WindowEvent)>,

    title: String,

    keys: [u64; glfw::ffi::KEY_LAST as usize + 1],
    mouse_buttons: [u64; glfw::ffi::MOUSE_BUTTON_LAST as usize + 1],

    current_frame: u64,

    frame_duration: Duration,
    last_time: Instant,
    sleeper: spin_sleep::SpinSleeper,

    width: u32,
    height: u32,

    aspect: f32,

    mouse_x: f32,
    mouse_y: f32,

    last_mouse_x: f32,
    last_mouse_y: f32,

    mouse_dx: f32,
    mouse_dy: f32,

    frame_time: Instant,
    delta_time: Duration,
}

impl Window {
    /// Is window still running. Primarily used in the main game loop.
    /// # Example
    /// ```rust
    /// while window.is_running() { ... }
    /// ```
    pub fn is_running(&self) -> bool {
        !self.handle.should_close()
    }

    /// Handles events in current frame. **Please call it at the frame start to avoid input lag.**
    /// # Example
    /// ```rust
    /// while window.is_running() {
    ///     window.poll_events();
    ///     ...
    /// }
    /// ```
    pub fn poll_events(&mut self) {
        self.delta_time = self.frame_time.elapsed();
        self.frame_time = Instant::now();

        let elapsed = self.last_time.elapsed();
        if elapsed < self.frame_duration {
            self.sleeper.sleep(self.frame_duration - elapsed);
        }

        self.last_time = Instant::now();

        self.glfw.poll_events();
        self.current_frame += 1;

        for (_, event) in glfw::flush_messages(&self.events) {
            match event {
                glfw::WindowEvent::FramebufferSize(width, height) => {
                    self.width = width as u32;
                    self.height = height as u32;
                    self.aspect = width as f32 / height as f32;

                    unsafe { gl::Viewport(0, 0, width, height) }
                }
                glfw::WindowEvent::Key(key, _, action, _) => {
                    match action {
                        glfw::Action::Press => {
                            self.keys[key as usize] = self.current_frame;
                        }
                        glfw::Action::Release => {
                            self.keys[key as usize] = 0;
                        }
                        _ => {}
                    }
                }
                glfw::WindowEvent::MouseButton(button, action, _) => {
                    match action {
                        glfw::Action::Press => {
                            self.mouse_buttons[button as usize] = self.current_frame;
                        }
                        glfw::Action::Release => {
                            self.mouse_buttons[button as usize] = 0;
                        }
                        _ => {}
                    }
                }
                
                _ => {}
            }
        }

        let cursor_pos = self.handle.get_cursor_pos();

        self.mouse_x = cursor_pos.0 as f32;
        self.mouse_y = cursor_pos.1 as f32;

        self.mouse_dx = self.mouse_x - self.last_mouse_x;
        self.mouse_dy = self.mouse_y - self.last_mouse_y;

        self.last_mouse_x = self.mouse_x;
        self.last_mouse_y = self.mouse_y;
    }

    /// Swaps front framebuffer with back that scene was rendered on. **Please call it at the frame end to avoid input lag.**
    /// # Example
    /// ```rust
    /// while window.is_running() {
    ///     ...
    ///     window.swap_buffers();
    /// }
    /// ```
    pub fn swap_buffers(&mut self) {
        self.handle.swap_buffers();
    }

    /// Sets window X position in pixels from top-left corner
    pub fn set_x(&mut self, value: i32) {
        let y = self.handle.get_pos().1;
        self.handle.set_pos(value, y);
    }
    /// Sets window Y position in pixels from top-left corner
    pub fn set_y(&mut self, value: i32) {
        let x = self.handle.get_pos().0;
        self.handle.set_pos(x, value);
    }
    /// Sets window position in pixels from top-left corner
    pub fn set_position(&mut self, x: i32, y: i32) {
        self.handle.set_pos(x, y);
    }

    /// Sets window width in pixels
    pub fn set_width(&mut self, value: i32) {
        self.handle.set_size(value, self.height as i32);
    }
    /// Sets window height in pixels
    pub fn set_height(&mut self, value: i32) {
        self.handle.set_size(self.width as i32, value);
    }
    /// Sets window size in pixels
    pub fn set_size(&mut self, width: i32, height: i32) {
        self.handle.set_size(width, height);
    }

    /// Sets window title to a new one.
    pub fn set_title(&mut self, value: String) {
        self.title = value;
        self.handle.set_title(&self.title);
    }
    /// Gets current window title.
    pub fn get_title(&self) -> &str {
        &self.title
    }

    /// Hides mouse and keeps it at the window center.
    /// Used primarily for first-person games where you don't want to see the cursor.
    pub fn grab_mouse(&mut self) {
        self.handle.set_cursor_mode(glfw::CursorMode::Disabled);
    }
    /// Shows mouse back and releases it from the window center.
    /// Used primarily for first-person games. For example, when you want to show a menu and release the mouse back.
    pub fn release_mouse(&mut self) {
        self.handle.set_cursor_mode(glfw::CursorMode::Normal);

        let cursor_pos = self.handle.get_cursor_pos();

        self.mouse_x = cursor_pos.0 as f32;
        self.mouse_y = cursor_pos.1 as f32;

        self.last_mouse_x = self.mouse_x;
        self.last_mouse_y = self.mouse_y;
    }
    /// Changes mouse state to grabbed/released.
    /// If mouse is grabbed - it would be released, else - it would be grabbed.
    pub fn toggle_mouse(&mut self) {
        if self.is_mouse_grabbed() {
            self.release_mouse();
        } else {
            self.grab_mouse();
        }
    }

    /// Checks if specific key is pressed.
    /// # Example
    /// ```rust
    /// use tinystorm::{window::WindowBuilder, glfw::Key};
    /// 
    /// let mut window = WindowBuilder::default().build();
    /// while window.is_running() {
    ///     window.poll_events();
    /// 
    ///     // If key is pressed it would print "Key A is pressed!",
    ///     // and it would print that each frame until key is released.
    ///     if window.is_key_pressed(Key::A) {
    ///         println!("Key A is pressed!");
    ///     }
    /// 
    ///     window.swap_buffers();
    /// }
    /// ```
    pub fn is_key_pressed(&self, key: glfw::Key) -> bool {
        self.keys[key as usize] > 0
    }
    /// Checks if specific key is just pressed.
    /// # Example
    /// ```rust
    /// use tinystorm::{window::WindowBuilder, glfw::Key};
    /// 
    /// let mut window = WindowBuilder::default().build();
    /// while window.is_running() {
    ///     window.poll_events();
    /// 
    ///     // If key was just pressed at this frame it would print "Key B is just pressed!",
    ///     // but on next frame it wouldn't trigger.
    ///     // 
    ///     // Used primarily when you want to check if key is just clicked,
    ///     // but not pressed for a certain time.
    ///     if window.is_key_just_pressed(Key::B) {
    ///         println!("Key B is just pressed!");
    ///     }
    /// 
    ///     window.swap_buffers();
    /// }
    /// ```
    pub fn is_key_just_pressed(&self, key: glfw::Key) -> bool {
        self.keys[key as usize] == self.current_frame
    }

    /// Checks if specific mouse button is pressed.
    /// # Example
    /// ```rust
    /// use tinystorm::{window::WindowBuilder, glfw::MouseButton};
    /// 
    /// let mut window = WindowBuilder::default().build();
    /// while window.is_running() {
    ///     window.poll_events();
    /// 
    ///     // If mouse button is pressed it would print "Left mouse button is pressed!",
    ///     // and it would print that each frame until mouse button is released.
    ///     if window.is_mouse_button_pressed(MouseButton::Left) {
    ///         println!("Left mouse button is pressed!");
    ///     }
    /// 
    ///     window.swap_buffers();
    /// }
    /// ```
    pub fn is_mouse_button_pressed(&self, button: glfw::MouseButton) -> bool {
        self.mouse_buttons[button as usize] > 0
    }
    /// Checks if specific mouse button is just pressed.
    /// # Example
    /// ```rust
    /// use tinystorm::{window::WindowBuilder, glfw::MouseButton};
    /// 
    /// let mut window = WindowBuilder::default().build();
    /// while window.is_running() {
    ///     window.poll_events();
    /// 
    ///     // If mouse button was just pressed at this frame it would print "Middle mouse button is just pressed!",
    ///     // but on next frame it wouldn't trigger.
    ///     // 
    ///     // Used primarily when you want to check if mouse button is just clicked,
    ///     // but not pressed for a certain time.
    ///     if window.is_mouse_button_just_pressed(MouseButton::Middle) {
    ///         println!("Middle mouse button is just pressed!");
    ///     }
    /// 
    ///     window.swap_buffers();
    /// }
    /// ```
    pub fn is_mouse_button_just_pressed(&self, button: glfw::MouseButton) -> bool {
        self.mouse_buttons[button as usize] == self.current_frame
    }

    /// Gets mouse cursor X position in pixels from top-left corner relative to window.
    pub fn get_mouse_x(&self) -> f32 {
        self.mouse_x
    }
    /// Gets mouse cursor Y position in pixels from top-left corner relative to window.
    pub fn get_mouse_y(&self) -> f32 {
        self.mouse_y
    }

    /// Gets mouse cursor delta X position in pixels from top-left corner.
    /// It represents a horizontal mouse cursor movement in current frame.
    pub fn get_mouse_dx(&self) -> f32 {
        self.mouse_dx
    }
    /// Gets mouse cursor delta Y position in pixels from top-left corner.
    /// It represents a vertical mouse cursor movement in current frame.
    pub fn get_mouse_dy(&self) -> f32 {
        self.mouse_dy
    }

    /// Gets window X position in pixels from top-left corner.
    pub fn get_x(&self) -> i32 {
        self.handle.get_pos().0
    }
    /// Gets window Y position in pixels from top-left corner.
    pub fn get_y(&self) -> i32 {
        self.handle.get_pos().1
    }

    /// Gets window width in pixels.
    pub fn get_width(&self) -> u32 {
        self.width
    }
    /// Gets window height in pixels.
    pub fn get_height(&self) -> u32 {
        self.height
    }

    /// Gets window aspect ratio. Formula: ```window.get_width() as f32 / window.get_height() as f32```
    pub fn get_aspect(&self) -> f32 {
        self.aspect
    }

    /// Gets delta time between last and current frames as [Duration] so you can get it in any format you want.
    /// It's used primarily for physics calculation, player movement or animations that are time-related.
    pub fn get_delta_raw(&self) -> Duration {
        self.delta_time
    }
    /// Gets delta time between last and current frames in seconds.
    /// It's used primarily for physics calculation, player movement or animations that are time-related.
    pub fn get_delta(&self) -> f32 {
        self.delta_time.as_secs_f32()
    }

    /// Returns if mouse is grabbed (it means it's hidden and moved to window center, primarily used for first-person games) or released.
    pub fn is_mouse_grabbed(&self) -> bool {
        self.handle.get_cursor_mode() == glfw::CursorMode::Disabled
    }

    /// Turn off the window prematurely. (It would just make [Window::is_running()] false)
    pub fn close(&mut self) {
        self.handle.set_should_close(true);
    }
}

/// A simple window builder, use it to create a window without headache and simple settings.
pub struct WindowBuilder {
    width: u32,
    height: u32,
    title: String,
    vsync: bool,
    max_fps: u32,
    msaa: u32,
}

impl WindowBuilder {
    /// # Example
    /// ```rust
    /// use tinystorm::WindowBuilder;
    /// 
    /// let window = WindowBuilder::default()
    ///     .with_max_fps(WindowBuilder::NO_MAX_FPS) // It would remove FPS bounds (except vsync is enabled).
    ///     .build();
    /// ```
    pub const NO_MAX_FPS: u32 = 0;
    /// # Example
    /// ```rust
    /// use tinystorm::WindowBuilder;
    /// 
    /// let window = WindowBuilder::default()
    ///     .with_msaa(WindowBuilder::NO_MSAA) // It would disable MSAA for an OpenGL context.
    ///     .build();
    /// ```
    pub const NO_MSAA: u32 = 0;
    
    /// Sets window default size in pixels.
    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.width = width;
        self.height = height;

        self
    }
    /// Sets window default title.
    pub fn with_title(mut self, title: String) -> Self {
        self.title = title;
        self
    }
    /// Enables/disables vsync for the window.
    pub fn with_vsync(mut self, vsync: bool) -> Self {
        self.vsync = vsync;
        self
    }
    /// Sets window FPS cap to max_fps.
    /// # No max FPS cap example
    /// ```rust
    /// use tinystorm::WindowBuilder;
    /// 
    /// let window = WindowBuilder::default()
    ///     .with_max_fps(WindowBuilder::NO_MAX_FPS) // It would remove FPS bounds (except vsync is enabled).
    ///     .build();
    /// ```
    /// # 144 FPS cap example
    /// ```rust
    /// use tinystorm::WindowBuilder;
    /// 
    /// let window = WindowBuilder::default()
    ///     .with_max_fps(144) // If vsync is enabled and it's, for example, 60hz - fps would be clamped to 60, not to 144.
    ///     .build();
    /// ```
    pub fn with_max_fps(mut self, max_fps: u32) -> Self {
        self.max_fps = max_fps;
        self
    }
    /// Enables multisampling for an OpenGL context. ([WindowBuilder::NO_MSAA] = no MSAA).
    pub fn with_msaa(mut self, msaa_quality: u32) -> Self {
        self.msaa = msaa_quality;
        self
    }

    /// Builds the window itself from settings declared before.
    /// # Example
    /// ```rust
    /// use tinystorm::WindowBuilder;
    /// 
    /// let window = WindowBuilder::default()
    ///     .with_size(960, 540)
    ///     // Other settings...
    ///     .build();
    /// ```
    pub fn build(&self) -> Window {
        let mut glfw = match glfw::init(glfw::fail_on_errors) {
            Ok(value) => value,
            Err(error) => panic!("Error! You're trying to make multiple windows. Unfortunately, that's an unsupported feature. Result: {}", error),
        };

        // glfw.window_hint(glfw::WindowHint::ContextVersion(4, 6));
        // glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
        // glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Compat));

        if self.msaa > 0 {
            glfw.window_hint(glfw::WindowHint::Samples(Some(self.msaa)));
        }
    
        let (mut handle, events) = glfw.create_window(
            self.width, self.height,
            &self.title,
            glfw::WindowMode::Windowed
        ).expect("Failed to create a window.");

        handle.make_current();
        handle.set_raw_mouse_motion(true);
        handle.set_key_polling(true);
        handle.set_mouse_button_polling(true);
        handle.set_framebuffer_size_polling(true);

        glfw.set_swap_interval(if self.vsync { glfw::SwapInterval::Sync(1) } else { glfw::SwapInterval::None });

        let framebuffer_size: (i32, i32) = handle.get_framebuffer_size();
        gl::load_with(|procname| handle.get_proc_address(procname));
        
        unsafe { gl::Viewport(0, 0, framebuffer_size.0, framebuffer_size.1); }
        if self.msaa > 0 {
            unsafe { gl::Enable(gl::MULTISAMPLE); }
        }

        Window {
            glfw,
            handle,
            events,

            title: self.title.clone(),

            keys: [0; glfw::ffi::KEY_LAST as usize + 1],
            mouse_buttons: [0; glfw::ffi::MOUSE_BUTTON_LAST as usize + 1],

            current_frame: 0,

            frame_duration: if self.max_fps == Self::NO_MAX_FPS { Duration::ZERO } else { Duration::from_secs_f32(1.0 / self.max_fps as f32) },
            last_time: Instant::now(),
            sleeper: SpinSleeper::default(),

            width: framebuffer_size.0 as u32,
            height: framebuffer_size.1 as u32,

            aspect: framebuffer_size.0 as f32 / framebuffer_size.1 as f32,

            mouse_x: 0.0,
            mouse_y: 0.0,

            last_mouse_x: 0.0,
            last_mouse_y: 0.0,

            mouse_dx: 0.0,
            mouse_dy: 0.0,

            frame_time: Instant::now(),
            delta_time: Duration::ZERO,
        }
    }
}

/// # Default values
/// ```
/// width: 960  
/// height: 540  
/// title: "Titled window, lol"  
/// vsync: true  
/// max_fps: [WindowBuilder::NO_MAX_FPS]  
/// msaa: [WindowBuilder::NO_MSAA] 
/// ```
impl Default for WindowBuilder {
    fn default() -> Self {
        Self {
            width: 960,
            height: 540,
            title: String::from("Titled window, lol"),
            vsync: true,
            max_fps: Self::NO_MAX_FPS,
            msaa: Self::NO_MSAA,
        }
    }
}
use crate::engine::{Engine, Game};
use glfw::{Context, GlfwReceiver, PWindow, WindowEvent};

pub struct DesktopPlatform {
    pub glfw: glfw::Glfw,
    pub window: PWindow,
    pub events: GlfwReceiver<(f64, WindowEvent)>,
}

impl DesktopPlatform {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        let mut glfw = glfw::init(glfw::fail_on_errors).expect("glfw init");

        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 1));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("window");

        window.make_current();

        window.set_key_polling(true);
        window.set_mouse_button_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_framebuffer_size_polling(true);

        Self {
            glfw,
            window,
            events,
        }
    }

    pub fn poll(&mut self) {
        self.glfw.poll_events();
    }

    pub fn swap_buffers(&mut self) {
        self.window.swap_buffers();
    }

    pub fn should_close(&self) -> bool {
        self.window.should_close()
    }
}

impl DesktopPlatform {
    pub fn run<G: Game>(&mut self, engine: &mut Engine, mut game: G) {
        game.init(engine);

        let mut last = self.glfw.get_time() as f32;

        while !self.should_close() {
            let time = self.glfw.get_time() as f32;
            let dt = time - last;
            last = time;

            engine.begin_frame();

            engine.poll_framebuffer_events(self);

            engine.update_camera_controls(self, dt);
            game.update(engine, time, dt);

            engine.render(time);

            self.swap_buffers();

            self.poll();
        }
    }
}

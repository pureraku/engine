use std::rc::Rc;

use glfw::{Action, Context, GlfwReceiver, Key, MouseButton, PWindow, WindowEvent};

use crate::assets::asset_manager::AssetManager;
use crate::assets::material::Material;
use crate::assets::mesh::Mesh;
use crate::camera::{Camera, FlyCamera, PlayerCamera};
use crate::renderer::{Lighting, Renderer};
use crate::scene::{EntityId, Scene};
use crate::transform::Transform;

pub trait Game {
    fn init(&mut self, engine: &mut Engine);
    fn update(&mut self, _engine: &mut Engine, _time: f32, _dt: f32) {}
}

pub struct Engine {
    glfw: glfw::Glfw,
    window: PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>,
    _gl: Rc<glow::Context>,
    renderer: Renderer,
    scene: Scene,
    assets_manager: AssetManager,
    camera: Camera,
    fly_camera: FlyCamera,
    player_camera: PlayerCamera,
    lighting: Lighting,
    mouse_locked: bool,
    use_player_camera: bool,
    toggle: bool,
}

impl Engine {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        let mut glfw = glfw::init(glfw::fail_on_errors).expect("glfw init");
        glfw.window_hint(glfw::WindowHint::ContextVersion(4, 1));
        glfw.window_hint(glfw::WindowHint::OpenGlProfile(
            glfw::OpenGlProfileHint::Core,
        ));
        glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("glfw window");

        window.make_current();
        window.set_key_polling(true);
        window.set_mouse_button_polling(true);
        window.set_framebuffer_size_polling(true);
        window.set_cursor_pos_polling(true);

        let gl = Rc::new(unsafe {
            glow::Context::from_loader_function(|s| window.get_proc_address(s) as *const _)
        });

        let (fb_w, fb_h) = window.get_framebuffer_size();
        let aspect = fb_w as f32 / fb_h.max(1) as f32;

        let camera = Camera::new(aspect);
        let renderer = Renderer::new(&gl);
        renderer.resize(fb_w as u32, fb_h as u32);
        let assets_manager = AssetManager::new(&gl);

        let mut engine = Self {
            glfw,
            window,
            events,
            _gl: gl,
            renderer,
            scene: Scene::default(),
            assets_manager,
            camera,
            fly_camera: FlyCamera::default(),
            player_camera: PlayerCamera::default(),
            lighting: Lighting::default(),
            mouse_locked: false,
            use_player_camera: true,
            toggle: false,
        };

        engine.window.set_cursor_mode(glfw::CursorMode::Normal);
        engine
    }

    pub fn assets(&mut self) -> &mut AssetManager {
        &mut self.assets_manager
    }

    pub fn lighting(&mut self) -> &mut Lighting {
        &mut self.lighting
    }

    pub fn spawn(
        &mut self,
        mesh: Rc<Mesh>,
        material: Rc<Material>,
        transform: Transform,
    ) -> EntityId {
        self.scene.spawn(mesh, material, transform)
    }
    pub fn transform_mut(&mut self, id: EntityId) -> &mut crate::transform::Transform {
        &mut self.scene.object_mut(id).transform
    }

    pub fn run<G: Game>(&mut self, mut game: G) {
        game.init(self);

        let mut last_time = self.glfw.get_time() as f32;

        while !self.window.should_close() {
            self.renderer.begin_frame();

            let time = self.glfw.get_time() as f32;
            let dt = (time - last_time).max(0.0);
            last_time = time;

            self.poll_framebuffer_events();
            self.update_camera_controls(dt);
            game.update(self, time, dt);
            self.renderer
                .draw_scene(&self.scene, &self.camera, &self.lighting);

            self.window.swap_buffers();
            self.glfw.poll_events();
        }
    }

    fn poll_framebuffer_events(&mut self) {
        for (_, event) in glfw::flush_messages(&self.events) {
            if let WindowEvent::FramebufferSize(w, h) = event {
                let w = w.max(1) as u32;
                let h = h.max(1) as u32;
                self.renderer.resize(w, h);
                self.camera.set_aspect(w as f32 / h as f32);
            }
        }
    }

    fn update_camera_controls(&mut self, dt: f32) {
        let toggle_pressed = self.window.get_key(Key::C) == Action::Press;

        if toggle_pressed && !self.toggle {
            self.use_player_camera = !self.use_player_camera;

            self.fly_camera.reset_mouse();
            self.player_camera.reset_mouse();
        }

        self.toggle = toggle_pressed;
        if self.window.get_key(Key::Escape) == Action::Press {
            self.window.set_cursor_mode(glfw::CursorMode::Normal);
            self.mouse_locked = false;

            self.fly_camera.set_enabled(false);
            self.player_camera.set_enabled(false);
        } else if !self.mouse_locked
            && self.window.get_mouse_button(MouseButton::Button1) == Action::Press
        {
            self.window.set_cursor_mode(glfw::CursorMode::Disabled);

            self.fly_camera.reset_mouse();
            self.player_camera.reset_mouse();

            self.mouse_locked = true;
        }

        if self.mouse_locked {
            self.fly_camera.set_enabled(!self.use_player_camera);
            self.player_camera.set_enabled(self.use_player_camera);
            if self.use_player_camera {
                self.player_camera
                    .update(&mut self.camera, &self.window, dt);
            } else {
                self.fly_camera.update(&mut self.camera, &self.window, dt);
            }
        }
    }
}

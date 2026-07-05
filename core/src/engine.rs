use crate::platform::desktop::DesktopPlatform;
use glfw::{Action, Key, MouseButton, WindowEvent};
use std::rc::Rc;

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
    pub fn new(platform: &mut DesktopPlatform) -> Self {
        let gl = Rc::new(unsafe {
            glow::Context::from_loader_function(|s| platform.window.get_proc_address(s) as *const _)
        });
        let (fb_w, fb_h) = platform.window.get_framebuffer_size();
        let aspect = fb_w as f32 / fb_h.max(1) as f32;

        let camera = Camera::new(aspect);
        let renderer = Renderer::new(&gl);
        renderer.resize(fb_w as u32, fb_h as u32);
        let assets_manager = AssetManager::new(&gl);

        let engine = Self {
            _gl: gl,
            renderer,
            scene: Scene::default(),
            assets_manager,
            camera,
            fly_camera: FlyCamera::default(),
            player_camera: PlayerCamera::default(),
            lighting: Lighting::default(),
            mouse_locked: false,
            use_player_camera: false,
            toggle: false,
        };

        platform.window.set_cursor_mode(glfw::CursorMode::Normal);
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

    pub(crate) fn poll_framebuffer_events(&mut self, platform: &mut DesktopPlatform) {
        for (_, event) in glfw::flush_messages(&platform.events) {
            if let WindowEvent::FramebufferSize(w, h) = event {
                let w = w.max(1) as u32;
                let h = h.max(1) as u32;
                self.renderer.resize(w, h);
                self.camera.set_aspect(w as f32 / h as f32);
            }
        }
    }
    pub fn begin_frame(&mut self) {
        self.renderer.begin_frame();
    }

    pub fn render(&mut self, time: f32) {
        self.renderer
            .draw_scene(&self.scene, &self.camera, &self.lighting, time);
    }
    pub(crate) fn update_camera_controls(&mut self, platform: &mut DesktopPlatform, dt: f32) {
        let toggle_pressed = platform.window.get_key(Key::C) == Action::Press;

        if toggle_pressed && !self.toggle {
            self.use_player_camera = !self.use_player_camera;

            self.fly_camera.reset_mouse();
            self.player_camera.reset_mouse();
        }

        self.toggle = toggle_pressed;
        if platform.window.get_key(Key::Escape) == Action::Press {
            platform.window.set_cursor_mode(glfw::CursorMode::Normal);
            self.mouse_locked = false;

            self.fly_camera.set_enabled(false);
            self.player_camera.set_enabled(false);
        } else if !self.mouse_locked
            && platform.window.get_mouse_button(MouseButton::Button1) == Action::Press
        {
            platform.window.set_cursor_mode(glfw::CursorMode::Disabled);

            self.fly_camera.reset_mouse();
            self.player_camera.reset_mouse();

            self.mouse_locked = true;
        }

        if self.mouse_locked {
            self.fly_camera.set_enabled(!self.use_player_camera);
            self.player_camera.set_enabled(self.use_player_camera);
            if self.use_player_camera {
                self.player_camera
                    .update(&mut self.camera, &platform.window, dt);
            } else {
                self.fly_camera
                    .update(&mut self.camera, &platform.window, dt);
            }
        }
    }
}

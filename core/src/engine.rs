use crate::platform::desktop::DesktopPlatform;
use glfw::{Action, Key, MouseButton, WindowEvent};
use std::rc::Rc;

use crate::assets::asset_manager::AssetManager;
use crate::assets::material::Material;
use crate::assets::mesh::Mesh;
use crate::camera::{Camera, FlyCamera, PlayerCamera};
use crate::input::Input;
use crate::renderer::{Lighting, Renderer};
use crate::scene::{EntityId, Scene};
use crate::transform::Transform;
use crate::ui::UiRenderer;

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
    input: Input,
    ui: UiRenderer,
    mouse_locked: bool,
    use_player_camera: bool,
    custom_camera_controller: bool,
    toggle: bool,
    screen_size: (u32, u32),
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
        let ui = UiRenderer::new(&gl, fb_w as f32, fb_h as f32);

        let engine = Self {
            _gl: gl,
            renderer,
            scene: Scene::default(),
            assets_manager,
            camera,
            fly_camera: FlyCamera::default(),
            player_camera: PlayerCamera::default(),
            lighting: Lighting::default(),
            input: Input::new(),
            ui,
            mouse_locked: false,
            use_player_camera: false,
            custom_camera_controller: false,
            toggle: false,
            screen_size: (fb_w as u32, fb_h as u32),
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

    pub fn camera(&self) -> &Camera {
        &self.camera
    }

    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }

    pub fn input(&self) -> &Input {
        &self.input
    }

    pub fn input_mut(&mut self) -> &mut Input {
        &mut self.input
    }

    pub fn ui(&mut self) -> &mut UiRenderer {
        &mut self.ui
    }

    pub fn screen_size(&self) -> (u32, u32) {
        self.screen_size
    }

    pub fn scene(&self) -> &Scene {
        &self.scene
    }

    pub fn scene_mut(&mut self) -> &mut Scene {
        &mut self.scene
    }

    pub fn is_cursor_locked(&self) -> bool {
        self.mouse_locked
    }

    pub fn set_cursor_locked(&mut self, locked: bool) {
        self.mouse_locked = locked;
    }

    pub fn set_use_player_camera(&mut self, use_player: bool) {
        self.use_player_camera = use_player;
    }

    pub fn set_custom_camera(&mut self, custom: bool) {
        self.custom_camera_controller = custom;
    }

    pub fn player_camera_mut(&mut self) -> &mut PlayerCamera {
        &mut self.player_camera
    }

    pub fn fly_camera_mut(&mut self) -> &mut FlyCamera {
        &mut self.fly_camera
    }

    pub fn spawn(
        &mut self,
        mesh: Rc<Mesh>,
        material: Rc<Material>,
        transform: Transform,
    ) -> EntityId {
        self.scene.spawn(mesh, material, transform)
    }

    pub fn despawn(&mut self, id: EntityId) {
        self.scene.despawn(id);
    }

    pub fn is_alive(&self, id: EntityId) -> bool {
        self.scene.is_alive(id)
    }

    pub fn set_visible(&mut self, id: EntityId, visible: bool) {
        self.scene.set_visible(id, visible);
    }

    pub fn transform_mut(&mut self, id: EntityId) -> &mut crate::transform::Transform {
        &mut self.scene.object_mut(id).transform
    }

    pub fn object_mut(&mut self, id: EntityId) -> &mut crate::scene::Object {
        self.scene.object_mut(id)
    }

    pub fn process_events(&mut self, platform: &mut DesktopPlatform) {
        for (_, event) in glfw::flush_messages(&platform.events) {
            match event {
                WindowEvent::FramebufferSize(w, h) => {
                    let w = w.max(1) as u32;
                    let h = h.max(1) as u32;
                    self.screen_size = (w, h);
                    self.renderer.resize(w, h);
                    self.camera.set_aspect(w as f32 / h as f32);
                    self.ui.resize(w as f32, h as f32);
                }
                WindowEvent::Key(key, _scancode, action, _mods) => {
                    self.input.on_key(key, action);
                }
                WindowEvent::MouseButton(button, action, _mods) => {
                    self.input.on_mouse_button(button, action);
                }
                WindowEvent::CursorPos(x, y) => {
                    self.input.on_cursor_pos(x, y);
                }
                WindowEvent::Scroll(x, y) => {
                    self.input.on_scroll(x, y);
                }
                _ => {}
            }
        }
    }

    pub fn poll_framebuffer_events(&mut self, platform: &mut DesktopPlatform) {
        self.process_events(platform);
    }

    pub fn begin_frame(&mut self) {
        self.renderer.begin_frame();
        self.ui.begin();
    }

    pub fn render(&mut self, time: f32) {
        self.renderer
            .draw_scene(&self.scene, &self.camera, &self.lighting, time);
    }

    pub fn flush_ui(&mut self) {
        self.ui.flush();
    }

    pub fn update_camera_controls(&mut self, platform: &mut DesktopPlatform, dt: f32) {
        if platform.window.get_key(Key::Escape) == Action::Press {
            platform.window.set_cursor_mode(glfw::CursorMode::Normal);
            self.mouse_locked = false;

            self.fly_camera.set_enabled(false);
            self.player_camera.set_enabled(false);
        } else if !self.mouse_locked
            && platform.window.get_mouse_button(MouseButton::Button1) == Action::Press
        {
            platform.window.set_cursor_mode(glfw::CursorMode::Disabled);

            self.input.reset_mouse();
            self.fly_camera.reset_mouse();
            self.player_camera.reset_mouse();

            self.mouse_locked = true;
        }

        if self.custom_camera_controller {
            return;
        }

        let toggle_pressed = platform.window.get_key(Key::C) == Action::Press;

        if toggle_pressed && !self.toggle {
            self.use_player_camera = !self.use_player_camera;

            self.fly_camera.reset_mouse();
            self.player_camera.reset_mouse();
        }

        self.toggle = toggle_pressed;

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


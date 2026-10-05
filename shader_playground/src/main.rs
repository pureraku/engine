mod app;

use app::App;
use engine::Engine;
use engine::platform::desktop::DesktopPlatform;

fn main() {
    let mut platform = DesktopPlatform::new(1280, 720, "Shader Playground");

    let mut engine = Engine::new(&mut platform);

    platform.run(&mut engine, App::default());
}

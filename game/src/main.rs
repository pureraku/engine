mod objects;
mod world;

use engine::Engine;
use engine::platform::desktop::DesktopPlatform;
use world::World;

fn main() {
    let mut platform = DesktopPlatform::new(1280, 720, "Game");

    let mut engine = Engine::new(&mut platform);

    platform.run(&mut engine, World::default());
}

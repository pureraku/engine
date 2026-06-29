mod objects;
mod world;

use engine::Engine;
use world::World;

fn main() {
    Engine::new(800, 600, "Press 'C' to toggle FlyCamera").run(World::default());
}

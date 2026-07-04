mod app;

use app::App;
use engine::Engine;

fn main() {
    Engine::new(800, 600, "Press 'C' to toggle FlyCamera").run(App::default());
}

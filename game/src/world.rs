use rand::Rng;

use crate::objects::cube::Cube;
use crate::objects::ground::Ground;
use crate::objects::sphere::Sphere;
use crate::objects::sun::Sun;
use engine::{Engine, Game};

pub struct World {
    cubes: Vec<Cube>,
    spheres: Vec<Sphere>,
    ground: Option<Ground>,
    sun: Option<Sun>,
}

impl Default for World {
    fn default() -> Self {
        Self {
            cubes: Vec::new(),
            spheres: Vec::new(),
            ground: None,
            sun: None,
        }
    }
}

impl Game for World {
    fn init(&mut self, engine: &mut Engine) {
        engine.lighting().light_intensity = 4.0;

        let mut rng = rand::thread_rng();

        self.create_cubes(engine, &mut rng);
        self.create_spheres(engine, &mut rng);
        self.ground = Some(Ground::new(engine));
        self.sun = Some(Sun::new(engine));
    }

    fn update(&mut self, engine: &mut Engine, time: f32, _dt: f32) {
        for cube in &self.cubes {
            cube.update(engine, time);
        }
        for sphere in &self.spheres {
            sphere.update(engine, time);
        }
    }
}

impl World {
    fn create_cubes(&mut self, engine: &mut Engine, rng: &mut impl Rng) {
        for _ in 0..8 {
            self.cubes.push(Cube::new(engine, rng));
        }
    }

    fn create_spheres(&mut self, engine: &mut Engine, rng: &mut impl Rng) {
        for _ in 0..12 {
            self.spheres.push(Sphere::new(engine, rng));
        }
    }
}

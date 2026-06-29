use engine::assets::assets_manager::MeshType;
use engine::{Engine, EntityId, Vec3, transform::Transform};
use std::rc::Rc;

pub struct Sun {
    pub _id: EntityId,
}

impl Sun {
    pub fn new(engine: &mut Engine) -> Self {
        let mesh = engine.assets().mesh(MeshType::UvSphere {
            stacks: 64,
            slices: 128,
        });

        let material = Rc::new(
            engine
                .assets()
                .new_material("basic")
                .with_color(Vec3::new(1.0, 1.0, 0.0)),
        );

        let mut transform = Transform::default();
        transform.scale = Vec3::splat(2.0);

        Self {
            _id: engine.spawn(mesh, material, transform),
        }
    }

    pub fn _update(&self, engine: &mut Engine, time: f32) {
        engine.transform_mut(self._id).rotation.y = time * 0.25;
    }
}

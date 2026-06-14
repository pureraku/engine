use glam::Vec3;
use rand::Rng;
use std::rc::Rc;

use crate::engine::{Engine, Game};
use crate::geometry;
use crate::material::Material;

pub struct DemoGame;

fn random_position(rng: &mut impl Rng) -> Vec3 {
    Vec3::new(
        rng.gen_range(-5..=4) as f32,
        rng.gen_range(-5..=4) as f32,
        rng.gen_range(-5..=4) as f32,
    )
}

impl Game for DemoGame {
    fn init(&mut self, engine: &mut Engine) {
        let assets = engine.assets();
        let (cube_mesh, sphere_mesh, cube_mat, sphere_mat) = {
            let cube_mesh = assets.mesh("cube", &geometry::cube());
            let sphere_mesh = assets.mesh("sphere", &geometry::uv_sphere(16, 32));

            let basic = assets.shader("basic");
            let toon = assets.shader("toon");
            let cat = assets.texture("cat", "assets/image.JPG");

            let cube_mat = Rc::new(Material::new(basic).with_texture(cat));
            let sphere_mat = Rc::new(
                Material::new(toon)
                    .with_color(Vec3::new(0.35, 0.7, 1.0))
                    .with_toon_steps(4.0),
            );

            (cube_mesh, sphere_mesh, cube_mat, sphere_mat)
        };

        // Creating a new cube at the origin ... 
        let vertex_shader = r#"
            #version 410 core
            layout(location = 0) in vec3 position;
            layout(location = 1) in vec3 normal;

            uniform mat4 model;
            uniform mat4 view;
            uniform mat4 projection;

            out VS_OUT {
                vec3 world_pos;
                vec3 normal;
            } vs_out;

            void main() {
                vec4 world_pos = model * vec4(position, 1.0);
                vs_out.world_pos = world_pos.xyz;
                vs_out.normal = normalize(mat3(model) * normal);
                gl_Position = projection * view * world_pos;
            }
        "#;

        let fragment_shader = r#"
            #version 410 core
            in VS_OUT {
                vec3 world_pos;
                vec3 normal;
            } fs_in;

            out vec4 FragColor;

            uniform vec3 lightPos;
            uniform vec3 lightColor;

            // Hash function for pseudo-random numbers
            float hash(float n) {
                return fract(sin(n) * 43758.5453123);
            }

            float hash3(vec3 p) {
                return fract(sin(dot(p, vec3(12.9898, 78.233, 45.164))) * 43758.5453);
            }

            // Perlin noise approximation
            float noise(vec3 p) {
                vec3 i = floor(p);
                vec3 f = fract(p);
                f = f * f * (3.0 - 2.0 * f);

                float n = mix(
                    mix(mix(hash3(i + vec3(0, 0, 0)), hash3(i + vec3(1, 0, 0)), f.x),
                        mix(hash3(i + vec3(0, 1, 0)), hash3(i + vec3(1, 1, 0)), f.x), f.y),
                    mix(mix(hash3(i + vec3(0, 0, 1)), hash3(i + vec3(1, 0, 1)), f.x),
                        mix(hash3(i + vec3(0, 1, 1)), hash3(i + vec3(1, 1, 1)), f.x), f.y), f.z);
                
                return n;
            }

            float fbm(vec3 p) {
                float value = 0.0;
                float amplitude = 1.0;
                float frequency = 1.0;
                
                for (int i = 0; i < 6; i++) {
                    value += amplitude * abs(noise(p * frequency) - 0.5);
                    frequency *= 2.0;
                    amplitude *= 0.5;
                }
                return value;
            }

            void main() {
                vec3 pos = fs_in.world_pos;
                vec3 normal = normalize(fs_in.normal);
                
                // Generate fractal brownian motion pattern
                float pattern = fbm(pos * 3.0);
                float detail = fbm(pos * 10.0 + pattern);
                float fine = fbm(pos * 30.0 + detail);
                
                // Create complex color mapping
                vec3 color1 = vec3(0.1, 0.3, 0.8);  // Deep blue
                vec3 color2 = vec3(0.9, 0.4, 0.1);  // Orange
                vec3 color3 = vec3(0.2, 0.9, 0.4);  // Green
                
                // Mix colors based on noise patterns
                vec3 base_color = mix(color1, color2, sin(pattern * 6.28) * 0.5 + 0.5);
                base_color = mix(base_color, color3, sin(detail * 6.28) * 0.5 + 0.5);
                
                // Add fine detail variation
                base_color += fine * 0.3 * vec3(0.3, 0.5, 0.8);
                
                // Lighting
                vec3 light_dir = normalize(lightPos - pos);
                float diffuse = max(dot(normal, light_dir), 0.3);
                
                // Rim lighting for extra pop
                vec3 view_dir = normalize(-pos);
                float rim = pow(1.0 - abs(dot(normal, view_dir)), 2.0);
                
                // Final composition
                vec3 color = base_color * diffuse + rim * 0.5 * vec3(1.0, 0.8, 1.0);
                
                FragColor = vec4(color, 1.0);
            }
        "#;

        
        let custom_shader = assets.shader_from_sources("test", vertex_shader, fragment_shader);
        let cool_mat = Rc::new(Material::new(custom_shader));

        // Spawn items in world
        {
            engine.spawn(
                cube_mesh.clone(),
                cool_mat,
                Vec3::ZERO,
                |t, time, _dt| {
                    t.scale = Vec3::new(2.0, 2.0, 4.0);
                    t.rotation.x = time * 0.3;
                    t.rotation.y = time * 0.5;
                    t.rotation.z = time * 0.2;
                }
            );


            let mut rng = rand::thread_rng();

            for _ in 0..12 {
                engine.spawn(
                    cube_mesh.clone(),
                    cube_mat.clone(),
                    random_position(&mut rng),
                    |t, time, _dt| t.rotation.y = time,
                );
            }

            for _ in 0..8 {
                engine.spawn(
                    sphere_mesh.clone(),
                    sphere_mat.clone(),
                    random_position(&mut rng),
                    |t, time, _dt| {
                        t.position.y += time.sin() * 0.0025;
                        t.rotation.x = time * 0.5;
                    },
                );
            }
        }
    }
}

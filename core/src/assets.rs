use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::geometry;
use crate::material::Material;
use crate::mesh::Mesh;
use crate::shader::Shader;
use crate::texture::Texture;

fn project_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

pub struct Assets {
    gl: Rc<glow::Context>,
    shaders: HashMap<String, Rc<Shader>>,
    textures: HashMap<String, Rc<Texture>>,
    meshes: HashMap<String, Rc<Mesh>>,
}
pub enum MeshType {
    Cube,
    UvSphere { stacks: i32, slices: i32 },
}

impl Assets {
    pub fn new(gl: &Rc<glow::Context>) -> Self {
        Self {
            gl: gl.clone(),
            shaders: HashMap::new(),
            textures: HashMap::new(),
            meshes: HashMap::new(),
        }
    }

    /// Load `shaders/{name}.vert` and `shaders/{name}.frag` (cached).
    pub fn shader(&mut self, name: &str) -> Rc<Shader> {
        if let Some(s) = self.shaders.get(name) {
            return s.clone();
        }
        let root = project_root().join("shaders");
        let vert = root.join(format!("{name}.vert"));
        let frag = root.join(format!("{name}.frag"));
        let s = Rc::new(Shader::from_files(
            &self.gl,
            vert.to_str().unwrap(),
            frag.to_str().unwrap(),
        ));
        self.shaders.insert(name.to_string(), s.clone());
        s
    }

    /// Load a texture relative to the project root (cached).
    pub fn texture(&mut self, key: &str, path: impl AsRef<Path>) -> Rc<Texture> {
        if let Some(t) = self.textures.get(key) {
            return t.clone();
        }
        let path = project_root().join(path);
        let t = Rc::new(Texture::from_path(&self.gl, &path));
        self.textures.insert(key.to_string(), t.clone());
        t
    }
    // Create new shader material
    pub fn new_material(&mut self, shader: &str) -> Material {
        Material::new(self.shader(shader))
    }

    /// Choose procedural geometry (cached).
    pub fn mesh(&mut self, kind: MeshType) -> Rc<Mesh> {
        let key = match &kind {
            MeshType::Cube => "cube".to_string(),
            MeshType::UvSphere { stacks, slices } => {
                format!("uv_sphere:{stacks}:{slices}")
            }
        };

        if let Some(mesh) = self.meshes.get(&key) {
            return mesh.clone();
        }

        let geometry = match kind {
            MeshType::Cube => geometry::cube(),
            MeshType::UvSphere { stacks, slices } => geometry::uv_sphere(stacks, slices),
        };

        let mesh = Rc::new(Mesh::new(&self.gl, &geometry.vertices, geometry.stride));

        self.meshes.insert(key, mesh.clone());
        mesh
    }
    /// Compile shader from source strings with caching
    #[allow(unused)]
    pub fn create_shader(&mut self, key: &str, vertex_src: &str, fragment_src: &str) -> Rc<Shader> {
        if let Some(s) = self.shaders.get(key) {
            return s.clone();
        }
        let s = Rc::new(Shader::from_sources(&self.gl, vertex_src, fragment_src));
        self.shaders.insert(key.to_string(), s.clone());
        s
    }
}

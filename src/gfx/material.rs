// external refs
use std::collections::HashMap;
use std::path::{Path, PathBuf};

// local refs
use crate::gfx::*;

///
/// Data needed to load a material
///
pub struct MaterialInfo {
    pub shader_file: PathBuf,
    pub vert_format: VertexFormat,
    pub topology: PrimitiveTopology,
    pub textures: HashMap<String, PathBuf>
}

///
/// Data for rendering a graphical object
///
pub struct Material {
    vs: Shader,
    ps: Shader,
    layout: InputLayout,
    topology: PrimitiveTopology,
    textures: Vec<Texture>,
    ps_inputs: Vec<(u32, ShaderInput)>
}

impl Material {
    ///
    /// Creates a new material
    ///
    pub fn create(vs: Shader, ps: Shader, topology: PrimitiveTopology, layout: InputLayout,
        textures: Vec<Texture>, ps_inputs: Vec<(u32, ShaderInput)>) -> Self {

        Self { vs, ps, layout, topology, textures, ps_inputs }
    }

    ///
    /// Loads a new material from a shader file
    ///
    pub fn load(gfx: &Graphics, mat_info: &MaterialInfo) -> Result<Self, ()> {
        let sc = ShaderCompiler::from_file(&mat_info.shader_file);
        let vs_code = sc.compile("VSMain", ShaderType::Vertex)?;
        let ps_code = sc.compile("PSMain", ShaderType::Pixel)?;
        let vs = gfx.create_vertex_shader(&vs_code)?;
        let ps = gfx.create_pixel_shader(&ps_code)?;
        let layout = gfx.create_input_layout(&mat_info.vert_format, &vs_code)?;

        let mut textures = Vec::with_capacity(mat_info.textures.len());
        let mut lookup = HashMap::<&Path, usize>::with_capacity(mat_info.textures.len());
        for (name, path) in mat_info.textures.iter() {
            if lookup.contains_key::<Path>(path) {
                continue;
            }
            if let Ok(tex) = gfx.load_texture_from_file(path) {
                lookup.insert(path, textures.len());
                textures.push(tex);
            }
        }

        let mut ps_inputs = Vec::<(u32, ShaderInput)>::new();

        // TODO: REFLECT THE SHADER!
        if let Some(path) = mat_info.textures.get("albedo_map") {
            let idx = lookup.get::<Path>(path).unwrap();
            if let Ok(input) = gfx.create_texture_shader_input(&textures[*idx]) {
                ps_inputs.push((0, input));
            }
        }
        // TODO: REFLECT THE SHADER!

        Ok(Self::create(vs, ps, mat_info.topology, layout, textures, ps_inputs))
    }

    ///
    /// Sets up the material for the graphics pipeline
    ///
    pub fn select(&self, gfx: &Graphics) {
        gfx.set_input_layout(&self.layout);
        gfx.set_primitive_topology(self.topology);
        gfx.set_vertex_shader(&self.vs);
        gfx.set_pixel_shader(&self.ps);

        for (slot, input) in &self.ps_inputs {
            gfx.set_ps_shader_input(*slot, input);
        }
    }

    ///
    /// Changes the primitive topology of the material
    ///
    pub fn set_primitive_topology(&mut self, topology: PrimitiveTopology) {
        self.topology = topology;
    }
}
// external refs
use tobj;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

// local refs
use crate::gfx::*;
use crate::numerics::*;

///
/// Utility to build or load a model
///
pub struct ModelBuilder {
    verts: Vec<MeshVertex>,
    indices: Vec<u32>,
    draws: Vec<DrawData>,
    materials: Vec<MaterialInfo>
}

impl ModelBuilder {
    ///
    /// Loads a model from an obj file
    ///
    pub fn load_from_obj(path: &Path) -> Result<Self, ()> {
        let (models, materials_in) = tobj::load_obj(path)
            .map_err(|e|{ println!("Failed to load {:?}. Error: {}", path, e); })?;

        let mut verts = Vec::new();
        let mut indices = Vec::new();
        let mut draws = Vec::with_capacity(models.len());
        let mut materials = Vec::with_capacity(materials_in.len());

        for mat in materials_in {
            let mut textures = HashMap::with_capacity(1);
            let tex_path: PathBuf = ["data\\objects", &mat.diffuse_texture].iter().collect();
            textures.insert("albedo_map".to_string(), tex_path);
            let mat_info = MaterialInfo {
                shader_file: PathBuf::from("data\\shaders\\object.hlsl"),
                vert_format: MeshVertex::get_format(),
                topology: PrimitiveTopology::TriangleList,
                textures,
            };
            materials.push(mat_info);
        }

        for model in models {
            let idx_offs = indices.len() as u32;
            let num_verts = model.mesh.positions.len() / 3;
            let num_tris = model.mesh.indices.len() / 3;

            indices.reserve(model.mesh.indices.len());
            verts.reserve(num_verts);

            for i in 0..num_verts {
                let mut v = MeshVertex::default();
                v.pos = Point3::new(
                    model.mesh.positions[i * 3 + 0],
                    model.mesh.positions[i * 3 + 1],
                    model.mesh.positions[i * 3 + 2]
                );
                if !model.mesh.normals.is_empty() {
                    v.norm = Vector3::new(
                        model.mesh.normals[i * 3 + 0],
                        model.mesh.normals[i * 3 + 1],
                        model.mesh.normals[i * 3 + 2]
                    );
                }
                if !model.mesh.texcoords.is_empty() {
                    v.uv = Vector2::new(
                        model.mesh.texcoords[i * 2 + 0],
                        model.mesh.texcoords[i * 2 + 1]
                    );
                }

                verts.push(v);
            }

            // wind the triangles differently to support our front-facing winding
            for i in 0..num_tris {
                let tri = &model.mesh.indices[(i * 3)..(i * 3 + 3)];
                indices.push(tri[0]);
                indices.push(tri[2]);
                indices.push(tri[1]);
            }

            let draw = DrawData {
                start_index: idx_offs,
                num_tris: (model.mesh.indices.len() / 3) as u32,
                material_idx: model.mesh.material_id.unwrap_or(0xFFFFFFFF) as u32
            };
            draws.push(draw);
        }

        Ok(Self { verts, indices, draws, materials })
    }

    ///
    /// Finalizes and builds the model
    ///
    pub fn build(self, gfx: &Graphics) -> Result<Model, ()> {
        // TODO: stop hardcoding material shader & get from obj data
        let vfmt = MeshVertex::get_format();
        let mut mats = Vec::with_capacity(self.materials.len());
        for mat in self.materials {
            mats.push(Material::load(gfx, &mat)?);
        }
        Model::new(gfx, &self.verts, &self.indices, self.draws, mats)
    }
}
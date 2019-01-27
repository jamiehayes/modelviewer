// local refs
use crate::gfx::*;
use crate::numerics::*;

///
/// Vertex format used by meshes
///
#[derive(Debug, Clone, Copy)]
pub struct MeshVertex {
    pub pos: Point3F,
    pub norm: Vector3F,
    pub tangent: Vector3F,
    pub uv: Vector2F,
    pub color: Color
}

impl MeshVertex {
    ///
    /// Gets the vertex format for this vertex type
    ///
    pub fn get_format() -> VertexFormat {
        VertexFormat::new()
            .add_new_element(FormatType::R32G32B32Float, 12, SemanticType::Position, 0, 0,
                InputClass::PerVertex, 0)
            .add_new_element(FormatType::R32G32B32Float, 12, SemanticType::Normal, 0, 0,
                InputClass::PerVertex, 0)
            .add_new_element(FormatType::R32G32B32Float, 12, SemanticType::Tangent, 0, 0,
                InputClass::PerVertex, 0)
            .add_new_element(FormatType::R32G32Float, 8, SemanticType::TexCoord, 0, 0,
                InputClass::PerVertex, 0)
            .add_new_element(FormatType::R8G8B8A8UNorm, 4, SemanticType::TexCoord, 1, 0,
                InputClass::PerVertex, 0)

    }
}

impl Default for MeshVertex {
    fn default() -> Self {
        Self {
            pos: Point3F::origin(),
            norm: zero(),
            tangent: zero(),
            uv: zero(),
            color: Color::white()
        }
    }
}

///
/// Represents data for a single draw call within a model
///
#[derive(Default, Copy, Clone)]
pub struct DrawData {
    pub start_index: u32,
    pub num_tris: u32,
    pub material_idx: u32
}

///
/// Handles the rendering of a model (clustering of meshes + materials)
///
pub struct Model {
    vb: VertexBuffer,
    ib: IndexBuffer,
    draws: Vec<DrawData>,
    mats: Vec<Material>
}

impl Model {
    ///
    /// Constructs a model from the specified data
    ///
    pub fn new(gfx: &Graphics, verts: &[MeshVertex], indices: &[u32], draws: Vec<DrawData>,
               mats: Vec<Material>) -> Result<Self, ()> {

        let vfmt = MeshVertex::get_format();
        let vb = gfx.create_vertex_buffer(&vfmt, verts)?;
        let ib = gfx.create_index_buffer(indices)?;

        Ok(Self { vb, ib, draws: draws, mats })
    }

    ///
    /// Draws the model
    ///
    pub fn draw(&self, gfx: &Graphics) {
        gfx.set_vertex_buffer(&self.vb, 0);
        gfx.set_index_buffer(&self.ib);

        for draw in &self.draws {
            let mat = &self.mats[draw.material_idx as usize];
            mat.select(gfx);

            gfx.draw_indexed(draw.num_tris * 3, draw.start_index);
        }
    }
}
// local refs
use crate::numerics::*;

///
/// Constants
///
pub const MAX_INPUTS: usize = 8;

#[derive(Debug, Copy, Clone)]
pub enum FormatType {
    Invalid = -1,

    // 8-bit unorm formats
    R8UNorm,
    R8G8UNorm,
    R8G8B8A8UNorm,

    // 8-bit snorm formats
    R8SNorm,
    R8G8SNorm,
    R8G8B8A8SNorm,

    // 16-bit unorm formats
    R16UNorm,
    R16G16UNorm,
    R16G16B16A16UNorm,

    // 16-bit snorm formats
    R16SNorm,
    R16G16SNorm,
    R16G16B16A16SNorm,

    // 16-bit float formats
    R16Float,
    R16G16Float,
    R16G16B16A16Float,

    // 32-bit float formats
    R32Float,
    R32G32Float,
    R32G32B32Float,
    R32G32B32A32Float,

    NumFormatTypes
}

#[derive(Debug, Copy, Clone)]
pub enum SemanticType {
    Invalid = -1,

    Position,
    Normal,
    Tangent,
    Bitangent,
    TexCoord,

    NumSemanticTypes
}

#[derive(Debug, Copy, Clone)]
pub enum InputClass {
    PerVertex,
    PerInstance
}

///
/// Describes a single attribute of a vertex buffer
///
#[derive(Debug, Copy, Clone)]
pub struct InputElement {
    pub semantic: SemanticType,
    pub semantic_index: u32,
    pub format: FormatType,
    pub input_slot: u32,
    pub byte_offset: u32,
    pub slot_class: InputClass,
    pub instance_step_rate: u32
}

impl Default for InputElement {
    ///
    /// Creates a default input element
    ///
    fn default() -> InputElement {
        InputElement {
            semantic: SemanticType::Invalid,
            semantic_index: 0,
            format: FormatType::Invalid,
            input_slot: 0,
            byte_offset: 0,
            slot_class: InputClass::PerVertex,
            instance_step_rate: 0
        }
    }
}

///
/// Describes the format of a vertex buffer
///
#[derive(Debug, Copy, Clone)]
pub struct VertexFormat {
    pub num_inputs: u32,
    pub stride: u32,
    pub inputs: [InputElement; MAX_INPUTS]
}

impl VertexFormat {
    ///
    /// Builder method to create an empty vertex format
    ///
    pub fn new() -> VertexFormat {
        VertexFormat { num_inputs: 0, stride: 0, inputs: [InputElement::default(); MAX_INPUTS] }
    }

    ///
    /// Builder method to add a vertex buffer element
    ///
    pub fn add_element(mut self, elem: &InputElement, size_bytes: u32) -> VertexFormat {
        assert!((self.num_inputs as usize) < MAX_INPUTS);

        self.inputs[self.num_inputs as usize] = *elem;
        self.num_inputs += 1;
        self.stride += size_bytes;

        self
    }

    ///
    /// Builder method to add a vertex buffer element
    ///
    pub fn add_new_element(self, format: FormatType, size_bytes: u32, semantic: SemanticType,
        semantic_index: u32, input_slot: u32, slot_class: InputClass, instance_step_rate: u32)
        -> VertexFormat {

        let byte_offset = if self.num_inputs == 0 { 0 } else { self.stride };
        let elem = InputElement {
            semantic,
            semantic_index,
            format,
            input_slot,
            byte_offset,
            slot_class,
            instance_step_rate,
        };

        self.add_element(&elem, size_bytes)
    }

    ///
    /// Builder method to add padding to the vertex format
    ///
    pub fn add_padding(mut self, size_bytes: u32) -> VertexFormat {
        self.stride += size_bytes;

        self
    }

    ///
    /// Clears the vertex format to empty
    ///
    pub fn reset(&mut self) {
        self.num_inputs = 0;
        self.stride = 0;
    }
}

///
/// Common vertex format containing position + color
///
#[derive(Debug, Clone, Copy)]
pub struct VertPosColor {
    pub pos: Point3F,
    pub color: Color4
}

impl VertPosColor {
    ///
    /// Gets the vertex format for this vertex type
    ///
    pub fn get_format() -> VertexFormat {
        VertexFormat::new()
            .add_new_element(FormatType::R32G32B32Float, 12, SemanticType::Position, 0, 0, InputClass::PerVertex, 0)
            .add_new_element(FormatType::R8G8B8A8UNorm, 4, SemanticType::TexCoord, 0, 0, InputClass::PerVertex, 0)
    }
}

impl Default for VertPosColor {
    fn default() -> Self {
        Self { pos: Point3F::origin(), color: Color4::white() }
    }
}

///
/// Common vertex format containing position + color
///
#[derive(Debug, Clone, Copy)]
pub struct VertPosNormUVColor {
    pub pos: Point3F,
    pub norm: Vector3F,
    pub uv: Vector2F,
    pub color: Color4
}

impl VertPosNormUVColor {
    ///
    /// Gets the vertex format for this vertex type
    ///
    pub fn get_format() -> VertexFormat {
        VertexFormat::new()
            .add_new_element(FormatType::R32G32B32Float, 12, SemanticType::Position, 0, 0,
                InputClass::PerVertex, 0)
            .add_new_element(FormatType::R32G32B32Float, 12, SemanticType::Normal, 0, 0,
                InputClass::PerVertex, 0)
            .add_new_element(FormatType::R32G32Float, 8, SemanticType::TexCoord, 0, 0,
                InputClass::PerVertex, 0)
            .add_new_element(FormatType::R8G8B8A8UNorm, 4, SemanticType::TexCoord, 1, 0,
                InputClass::PerVertex, 0)

    }
}

impl Default for VertPosNormUVColor {
    fn default() -> Self {
        Self {
            pos: Point3F::origin(),
            norm: zero(),
            uv: zero(),
            color: Color4::white()
        }
    }
}
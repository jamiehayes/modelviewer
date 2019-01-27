// external refs
use winapi::um::d3d11;

///
/// The type of the shader
///
#[derive(Debug, Clone, Copy)]
pub enum ShaderType {
    Vertex,
    Pixel
}

///
/// Shader struct
///
pub enum Shader {
    Vertex(*mut d3d11::ID3D11VertexShader),
    Pixel(*mut d3d11::ID3D11PixelShader)
}
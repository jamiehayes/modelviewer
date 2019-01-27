// external refs
use winapi::um::d3d11;

///
/// Holds the input layout that bridges the gap between a vertex buffer and a vertex shader
///
pub struct InputLayout {
    pub data: *mut d3d11::ID3D11InputLayout
}

impl Drop for InputLayout {
    fn drop(&mut self) {
        if !self.data.is_null() {
            unsafe { (*self.data).Release(); }
        }
    }
}
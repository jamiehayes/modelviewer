// external refs
use winapi::um::d3d11;
use winapi::shared::winerror;

///
/// Texture or buffer input for a shader
///
pub struct ShaderInput {
    pub srv: *mut d3d11::ID3D11ShaderResourceView
}

impl ShaderInput {
    ///
    /// Creates a new shader input from the specified texture
    ///
    pub fn new(device: *mut d3d11::ID3D11Device, res: *mut d3d11::ID3D11Resource)
        -> Result<ShaderInput, ()> {

        let mut srv = std::ptr::null_mut::<d3d11::ID3D11ShaderResourceView>();
        let hr = unsafe {
            (*device).CreateShaderResourceView(res, std::ptr::null(), &mut srv as *mut *mut _)
        };

        if hr != winerror::S_OK {
            println!("Failed to create a shader resource view: {:#x}", hr);
            Err(())
        }
        else {
            Ok(ShaderInput { srv })
        }
    }
}

impl Drop for ShaderInput {
    fn drop(&mut self) {
        if !self.srv.is_null() {
            unsafe { (*self.srv).Release(); }
        }
    }
}
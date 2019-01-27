// external refs
use winapi::um::d3d11;
use winapi::shared::winerror;

///
/// A structure that wraps index buffer functionality
pub struct ShaderBuffer {
    pub buff: *mut d3d11::ID3D11Buffer,
}

impl ShaderBuffer {
    ///
    /// Creates a dynamic shader buffer
    ///
    pub fn new_constant_buffer(device: *mut d3d11::ID3D11Device, size: usize)
        -> Result<ShaderBuffer, ()> {

        if size == 0 {
            return Err(());
        }

        // buffer sizes have to be aligned to 16b
        let rounded_size = 16 * ((size + 15) / 16) as u32;

        // TODO: add ability to use either u16 or u32
        let desc = d3d11::D3D11_BUFFER_DESC {
            ByteWidth: rounded_size,
            Usage: d3d11::D3D11_USAGE_DYNAMIC,
            BindFlags: d3d11::D3D11_BIND_CONSTANT_BUFFER,
            CPUAccessFlags: d3d11::D3D11_CPU_ACCESS_WRITE,
            MiscFlags: 0,
            StructureByteStride: 0
        };
        let mut buff = std::ptr::null_mut::<d3d11::ID3D11Buffer>();
        let hr = unsafe {
            (*device).CreateBuffer(&desc, std::ptr::null(), &mut buff as *mut *mut _)
        };

        if hr != winerror::S_OK {
            println!("Failed to create a constant buffer of size {}. Error: {:#x}", size, hr);
            Err(())
        }
        else {
            Ok(ShaderBuffer { buff })
        }
    }
}

impl Drop for ShaderBuffer {
    ///
    /// Cleans up resources for the buffer
    ///
    fn drop(&mut self) {
        if !self.buff.is_null() {
            unsafe { (*self.buff).Release(); }
        }
    }
}
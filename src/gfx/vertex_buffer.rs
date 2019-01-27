// external refs
use winapi::um::d3d11;
use winapi::shared::winerror;

// local refs
use crate::gfx::VertexFormat;

///
/// A structure that wraps vertex buffer functionality
pub struct VertexBuffer {
    pub format: VertexFormat,
    pub buff: *mut d3d11::ID3D11Buffer,
}

impl VertexBuffer {
    ///
    /// Creates a new vertex buffer
    ///
    pub fn new(device: *mut d3d11::ID3D11Device, format: &VertexFormat, num_verts: u32,
        init_data: *const u8) -> Result<VertexBuffer, ()> {

        assert!(!init_data.is_null(), "VertexBuffers are immutable and must be initialized");

        let desc = d3d11::D3D11_BUFFER_DESC {
            ByteWidth: format.stride * num_verts,
            Usage: d3d11::D3D11_USAGE_IMMUTABLE,
            BindFlags: d3d11::D3D11_BIND_VERTEX_BUFFER,
            CPUAccessFlags: 0,
            MiscFlags: 0,
            StructureByteStride: format.stride
        };
        let init_desc = d3d11::D3D11_SUBRESOURCE_DATA {
            pSysMem: init_data as _,
            SysMemPitch: num_verts * format.stride,
            SysMemSlicePitch: 0
        };
        let mut buff = std::ptr::null_mut::<d3d11::ID3D11Buffer>();
        let hr = unsafe {
            (*device).CreateBuffer(&desc, &init_desc, &mut buff as *mut *mut _)
        };

        if hr != winerror::S_OK {
            Err(())
        }
        else {
            Ok(VertexBuffer { format: *format, buff })
        }
    }
}

impl Drop for VertexBuffer {
    ///
    /// Cleans up resources for the buffer
    ///
    fn drop(&mut self) {
        if !self.buff.is_null() {
            unsafe { (*self.buff).Release(); }
        }
    }
}
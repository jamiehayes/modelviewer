// external refs
use winapi::um::d3d11;
use winapi::shared::winerror;

///
/// A structure that wraps index buffer functionality
pub struct IndexBuffer {
    pub buff: *mut d3d11::ID3D11Buffer,
}

impl IndexBuffer {
    ///
    /// Creates a new index buffer
    ///
    pub fn new(device: *mut d3d11::ID3D11Device, indices: &[u32]) -> Result<IndexBuffer, ()> {
        if indices.len() == 0 {
            return Err(());
        }

        // TODO: add ability to use either u16 or u32
        const INDEX_SIZE: usize = std::mem::size_of::<u32>();
        let desc = d3d11::D3D11_BUFFER_DESC {
            ByteWidth: (indices.len() * INDEX_SIZE) as _,
            Usage: d3d11::D3D11_USAGE_IMMUTABLE,
            BindFlags: d3d11::D3D11_BIND_INDEX_BUFFER,
            CPUAccessFlags: 0,
            MiscFlags: 0,
            StructureByteStride: INDEX_SIZE as _
        };
        let init_desc = d3d11::D3D11_SUBRESOURCE_DATA {
            pSysMem: indices.as_ptr() as _,
            SysMemPitch: desc.ByteWidth,
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
            Ok(IndexBuffer { buff })
        }
    }
}

impl Drop for IndexBuffer {
    ///
    /// Cleans up resources for the buffer
    ///
    fn drop(&mut self) {
        if !self.buff.is_null() {
            unsafe { (*self.buff).Release(); }
        }
    }
}
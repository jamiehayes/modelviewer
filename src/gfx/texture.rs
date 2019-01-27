// external refs
use winapi::um::d3d11;
use winapi::shared::winerror;
use winapi::shared::dxgitype;
use winapi::shared::dxgiformat;

///
/// Texture formats
///
#[derive(Debug, Copy, Clone)]
pub enum TextureFormat {
    R8UNorm,
    R8G8UNorm,
    R8G8B8A8UNorm,
    R8G8B8A8UNormSrgb,
    R32Float,
    R32G32Float,
    R32G32B32A32Float
}

///
/// Texture resource
///
pub struct Texture {
    pub format: TextureFormat,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub res: *mut d3d11::ID3D11Resource
}

impl Texture {
    ///
    /// Creates a new texture from the specified data
    ///
    pub fn new_texture2d<T>(device: *mut d3d11::ID3D11Device, format: TextureFormat, width: u32,
        height: u32, mips: u32, generate_mips: bool, render_target: bool, shader_resource: bool,
        pixel_data: &[T]) -> Result<Texture, ()> {

        const TEXTURE_FORMATS: [dxgiformat::DXGI_FORMAT; 7] = [
            dxgiformat::DXGI_FORMAT_R8_UNORM,
            dxgiformat::DXGI_FORMAT_R8G8_UNORM,
            dxgiformat::DXGI_FORMAT_R8G8B8A8_UNORM_SRGB,
            dxgiformat::DXGI_FORMAT_R8G8B8A8_UNORM,
            dxgiformat::DXGI_FORMAT_R32_FLOAT,
            dxgiformat::DXGI_FORMAT_R32G32_FLOAT,
            dxgiformat::DXGI_FORMAT_R32G32B32A32_FLOAT,
        ];

        const PIXEL_SIZES: [u32; 7] = [
            1,
            2,
            4,
            4,
            4,
            8,
            16
        ];

        let mut bind_flags = 0;
        if render_target {
            bind_flags |= d3d11::D3D11_BIND_RENDER_TARGET;
        }
        if shader_resource {
            bind_flags |= d3d11::D3D11_BIND_SHADER_RESOURCE;
        }

        let mut misc_flags = 0;
        if generate_mips {
            misc_flags = d3d11::D3D11_RESOURCE_MISC_GENERATE_MIPS
        }

        let desc = d3d11::D3D11_TEXTURE2D_DESC {
            Width: width,
            Height: height,
            MipLevels: mips,
            ArraySize: 1,
            Format: TEXTURE_FORMATS[format as usize],
            SampleDesc: dxgitype::DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Usage: d3d11::D3D11_USAGE_IMMUTABLE,
            BindFlags: bind_flags,
            CPUAccessFlags: 0,
            MiscFlags: misc_flags
        };

        let init_data = d3d11::D3D11_SUBRESOURCE_DATA {
            pSysMem: pixel_data.as_ptr() as _,
            SysMemPitch: width * PIXEL_SIZES[format as usize],
            SysMemSlicePitch: width * height * PIXEL_SIZES[format as usize]
        };

        let mut res = std::ptr::null_mut::<d3d11::ID3D11Texture2D>();
        let hr = unsafe {
            (*device).CreateTexture2D(&desc, &init_data, &mut res as *mut *mut _)
        };

        if hr != winerror::S_OK {
            println!("Failed to create a new texture 2D: {:#x}", hr);
            Err(())
        }
        else {
            Ok(Texture { format, width, height, depth: 1, res: res as _ })
        }
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        if !self.res.is_null() {
            unsafe { (*self.res).Release(); }
        }
    }
}
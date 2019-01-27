use winapi::um::d3d11;
use winapi::shared::dxgiformat;
use winapi::shared::dxgitype;
use winapi::shared::winerror;

pub enum DepthStencilFormat {
    D16UNorm,
    D24UNormS8,
    D32Float
}

///
/// Interface for a graphics render target
///
pub struct DepthStencilTarget {
    pub dsv: *mut d3d11::ID3D11DepthStencilView,
    pub res: *mut d3d11::ID3D11Resource
}

impl DepthStencilTarget {
    ///
    /// Creates a new render target object from the specified swap chain
    ///
    pub fn new(device: *mut d3d11::ID3D11Device, format: DepthStencilFormat, width: u32,
        height: u32) -> Result<Self, ()> {

        const TEXTURE_FORMATS: [dxgiformat::DXGI_FORMAT; 3] = [
            dxgiformat::DXGI_FORMAT_D16_UNORM,
            dxgiformat::DXGI_FORMAT_D24_UNORM_S8_UINT,
            dxgiformat::DXGI_FORMAT_D32_FLOAT
        ];

        const PIXEL_SIZES: [u32; 2] = [
            4,
            16
        ];

        let desc = d3d11::D3D11_TEXTURE2D_DESC {
            Width: width,
            Height: height,
            MipLevels: 1,
            ArraySize: 1,
            Format: TEXTURE_FORMATS[format as usize],
            SampleDesc: dxgitype::DXGI_SAMPLE_DESC { Count: 1, Quality: 0 },
            Usage: d3d11::D3D11_USAGE_DEFAULT,
            BindFlags: d3d11::D3D11_BIND_DEPTH_STENCIL,
            CPUAccessFlags: 0,
            MiscFlags: 0
        };

        let mut tex = std::ptr::null_mut::<d3d11::ID3D11Texture2D>();
        let mut hr = unsafe {
            (*device).CreateTexture2D(&desc, std::ptr::null(), &mut tex as *mut *mut _)
        };

        if hr != winerror::S_OK {
            println!("Failed to create a new depth texture: {:#x}", hr);
            return Err(());
        }

        // create a render target view for the texture
        let mut dsv = std::ptr::null_mut::<d3d11::ID3D11DepthStencilView>();
        hr = unsafe { (*device).CreateDepthStencilView(tex as _, std::ptr::null(), &mut dsv) };

        if hr != winerror::S_OK {
            println!("Failed to create depth stencil view: {:#x}", hr);
            Err(())
        }
        else {
            // create DepthStencilTarget object with the texture and dsv
            Ok(Self { dsv, res: tex as _ })
        }
    }
}

impl Drop for DepthStencilTarget {
    ///
    /// Cleans up the internal render target when dropped
    ///
    fn drop(&mut self) {
        if !self.dsv.is_null() {
            unsafe { (*self.dsv).Release(); }
        }
        if !self.res.is_null() {
            unsafe { (*self.res).Release(); }
        }
    }
}
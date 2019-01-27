// external refs
use winapi::um::d3d11;
use winapi::shared::winerror;

///
/// Sample Interpolation options
///
#[derive(Debug, Copy, Clone)]
pub enum SampleMode {
    Point,
    Linear,
    Anisotropic,
    ComparisonPoint,
    ComparisonLinear,
    ComparisonAnisotropic
}

///
/// How to handle going off the edge of the sampler
///
#[derive(Debug, Copy, Clone)]
pub enum AddressMode {
    Wrap,
    Mirror,
    Clamp,
    BorderColor,
    MirrorOnce
}

///
/// Sampler comparison func
///
#[derive(Debug, Copy, Clone)]
pub enum ComparisonFunc {
    Never,
    LessThan,
    Equal,
    LessOrEqual,
    GreatherThan,
    NotEqual,
    GreaterOrEqual,
    Always,
}

///
/// Border color options
///
#[derive(Debug, Copy, Clone)]
pub enum BorderColor {
    OpaqueBlack,
    OpaqueWhite,
    TransparentBlack,
    TransparentWhite
}

///
/// Data for the sampler
///
#[derive(Debug, Copy, Clone)]
pub struct SamplerData {
    pub mode: SampleMode,
    pub address_u: AddressMode,
    pub address_v: AddressMode,
    pub address_w: AddressMode,
    pub comparison: ComparisonFunc,
    pub lod_bias: f32,
    pub max_anisotropy: u32,
    pub border_color: BorderColor,
    pub min_lod: f32,
    pub max_lod: f32
}

pub struct Sampler {
    pub data: SamplerData,
    pub samp: *mut d3d11::ID3D11SamplerState
}

impl Sampler {
    ///
    /// Creates a new sampler state
    ///
    pub fn new(device: *mut d3d11::ID3D11Device, data: SamplerData) -> Result<Self, ()> {
        const FILTERS: [d3d11::D3D11_FILTER; 6] = [
            d3d11::D3D11_FILTER_MIN_MAG_MIP_POINT,
            d3d11::D3D11_FILTER_MIN_MAG_MIP_LINEAR,
            d3d11::D3D11_FILTER_ANISOTROPIC,
            d3d11::D3D11_FILTER_COMPARISON_MIN_MAG_MIP_POINT,
            d3d11::D3D11_FILTER_COMPARISON_MIN_MAG_MIP_LINEAR,
            d3d11::D3D11_FILTER_COMPARISON_ANISOTROPIC,
        ];
        const ADDRESS_MODE: [d3d11::D3D11_TEXTURE_ADDRESS_MODE; 5] = [
            d3d11::D3D11_TEXTURE_ADDRESS_WRAP,
            d3d11::D3D11_TEXTURE_ADDRESS_MIRROR,
            d3d11::D3D11_TEXTURE_ADDRESS_CLAMP,
            d3d11::D3D11_TEXTURE_ADDRESS_BORDER,
            d3d11::D3D11_TEXTURE_ADDRESS_MIRROR_ONCE
        ];
        const COMPARISON_FUNCS: [d3d11::D3D11_COMPARISON_FUNC; 8] = [
            d3d11::D3D11_COMPARISON_NEVER,
            d3d11::D3D11_COMPARISON_LESS,
            d3d11::D3D11_COMPARISON_EQUAL,
            d3d11::D3D11_COMPARISON_LESS_EQUAL,
            d3d11::D3D11_COMPARISON_GREATER,
            d3d11::D3D11_COMPARISON_NOT_EQUAL,
            d3d11::D3D11_COMPARISON_GREATER_EQUAL,
            d3d11::D3D11_COMPARISON_ALWAYS
        ];
        let filter = FILTERS[data.mode as usize];
        let address_u = ADDRESS_MODE[data.address_u as usize];
        let address_v = ADDRESS_MODE[data.address_v as usize];
        let address_w = ADDRESS_MODE[data.address_w as usize];
        let comparison = COMPARISON_FUNCS[data.comparison as usize];
        let border_color = match data.border_color {
            BorderColor::OpaqueBlack => [0.0f32, 0.0f32, 0.0f32, 1.0f32],
            BorderColor::OpaqueWhite => [1.0f32, 1.0f32, 1.0f32, 1.0f32],
            BorderColor::TransparentBlack => [0.0f32; 4],
            BorderColor::TransparentWhite => [1.0f32; 4]
        };
        let desc = d3d11::D3D11_SAMPLER_DESC {
            Filter: filter,
            AddressU: address_u,
            AddressV: address_v,
            AddressW: address_w,
            MipLODBias: data.lod_bias,
            MaxAnisotropy: data.max_anisotropy,
            ComparisonFunc: comparison,
            BorderColor: border_color,
            MinLOD: data.min_lod,
            MaxLOD: data.max_lod
        };

        let mut samp = std::ptr::null_mut::<d3d11::ID3D11SamplerState>();
        let hr = unsafe { (*device).CreateSamplerState(&desc, &mut samp as *mut *mut _) };

        if hr != winerror::S_OK {
            Err(())
        }
        else {
            Ok(Sampler { data, samp })
        }
    }
}

impl Drop for Sampler {
    fn drop(&mut self) {
        if !self.samp.is_null() {
            unsafe { (*self.samp).Release() };
        }
    }
}
// external refs
use winapi::um::d3d11;
use winapi::um::d3dcommon;
use winapi::um::d3d11sdklayers;
use winapi::shared::dxgiformat;
use winapi::shared::winerror;
use std::ffi::CStr;
use stb_image::image;

// local refs
use crate::gfx::*;
use crate::numerics::*;

#[derive(Debug, Clone, Copy)]
pub enum PrimitiveTopology {
    PointList,
    LineList,
    LineStrip,
    TriangleList,
    TriangleStrip
}

///
/// Main abstraction layer for graphics device and functionality
///
pub struct Graphics {
    device: *mut d3d11::ID3D11Device,
    context: *mut d3d11::ID3D11DeviceContext
}

impl Graphics {
    ///
    /// Creates a new graphics object
    ///
    pub fn new() -> Result<Graphics, ()> {
        let mut device = std::ptr::null_mut::<d3d11::ID3D11Device>();
        let mut context = std::ptr::null_mut::<d3d11::ID3D11DeviceContext>();
        let feature_level = d3dcommon::D3D_FEATURE_LEVEL_11_0;

        // create the D3D11 device
        let flags = if cfg!(debug_assertions) { d3d11::D3D11_CREATE_DEVICE_DEBUG } else { 0 };
        let mut hr = unsafe {
            d3d11::D3D11CreateDevice(
                std::ptr::null_mut(),
                d3dcommon::D3D_DRIVER_TYPE_HARDWARE,
                0 as _,
                flags,
                &feature_level as *const _,
                1,
                d3d11::D3D11_SDK_VERSION,
                &mut device as *mut *mut _,
                std::ptr::null_mut(),
                &mut context as *mut *mut _
            )
        };

        if hr != winerror::S_OK {
            return Err(());
        }

        if cfg!(debug_assertions) {
            unsafe {
                // Enable breaking on errors and warnings
                let mut dbg = std::ptr::null_mut::<d3d11sdklayers::ID3D11Debug>();
                hr = (*device).QueryInterface(
                    &d3d11sdklayers::IID_ID3D11Debug,
                    &mut dbg as *mut *mut _ as *mut *mut _
                );

                let mut info = std::ptr::null_mut::<d3d11sdklayers::ID3D11InfoQueue>();
                if hr == winerror::S_OK && !dbg.is_null() {
                    hr = (*dbg).QueryInterface(
                        &d3d11sdklayers::IID_ID3D11InfoQueue,
                        &mut info as *mut *mut _ as *mut *mut _
                    );

                    if hr == winerror::S_OK && !info.is_null() {
                        const BREAK_SEVERITIES: [d3d11sdklayers::D3D11_MESSAGE_SEVERITY; 3] = [
                            d3d11sdklayers::D3D11_MESSAGE_SEVERITY_CORRUPTION,
                            d3d11sdklayers::D3D11_MESSAGE_SEVERITY_ERROR,
                            d3d11sdklayers::D3D11_MESSAGE_SEVERITY_WARNING,
                        ];

                        for sev in &BREAK_SEVERITIES {
                            (*info).SetBreakOnSeverity(*sev, 1);
                        }

                        (*info).Release();
                    }

                    (*dbg).Release();
                }
            }
        }

        Ok(Graphics { device, context })
    }

    ///
    /// Creates a new display object for the specified window handle
    ///
    pub fn create_display(&self, hwnd: u64) -> Result<Display, ()> {
        Display::new(self.device, hwnd)
    }

    ///
    /// Creates a new texture from memory
    ///
    pub fn create_texture2d<T>(&self, format: TextureFormat, width: u32, height: u32, mips: u32,
        generate_mips: bool, render_target: bool, shader_resource: bool, pixel_data: &[T])
        -> Result<Texture, ()> {

        Texture::new_texture2d(self.device, format, width, height, mips, generate_mips,
            render_target, shader_resource, pixel_data)
    }

    ///
    /// Loads a texture from file
    ///
    pub fn load_texture_from_file(&self, path: &std::path::Path) -> Result<Texture, ()> {
        match image::load(path) {
            image::LoadResult::Error(e) => {
                println!("Failed to load texture from {:?}: {}", path, e);
                Err(())
            }
            image::LoadResult::ImageU8(img) => {
                assert!(img.depth > 0 && img.depth <= 4);
                if img.depth != 3 {
                    let fmt = match img.depth {
                        1 => TextureFormat::R8UNorm,
                        2 => TextureFormat::R8G8UNorm,
                        _ => TextureFormat::R8G8B8A8UNormSrgb
                    };
                    self.create_texture2d(
                        fmt,
                        img.width as _,
                        img.height as _,
                        1, //15,
                        false, //true,
                        false,
                        true,
                        &img.data
                    )
                }
                else {
                    let mut data = Vec::with_capacity(img.width * img.height);
                    for i in 0..(img.width * img.height) {
                        let r = img.data[i * 3 + 0];
                        let g = img.data[i * 3 + 1];
                        let b = img.data[i * 3 + 2];
                        let a = 255;

                        data.push(Color4::from_rgba(r, g, b, a));
                    }
                    self.create_texture2d(
                        TextureFormat::R8G8B8A8UNormSrgb,
                        img.width as _,
                        img.height as _,
                        1, //15,
                        false, //true,
                        false,
                        true,
                        &data
                    )
                }
            }
            image::LoadResult::ImageF32(img) => {
                assert!(img.depth > 0 && img.depth <= 4);
                if img.depth != 3 {
                    let fmt = match img.depth {
                        1 => TextureFormat::R32Float,
                        2 => TextureFormat::R32G32Float,
                        _ => TextureFormat::R32G32B32A32Float
                    };
                    self.create_texture2d(
                        fmt,
                        img.width as _,
                        img.height as _,
                        1, //15,
                        false, //true,
                        false,
                        true,
                        &img.data
                    )
                }
                else {
                    let mut data = Vec::with_capacity(img.width * img.height);
                    for i in 0..(img.width * img.height) {
                        let r = img.data[i * 3 + 0];
                        let g = img.data[i * 3 + 1];
                        let b = img.data[i * 3 + 2];
                        let a = 1.0f32;

                        data.push(Color4F::from_rgba(r, g, b, a));
                    }
                    self.create_texture2d(
                        TextureFormat::R32G32B32A32Float,
                        img.width as _,
                        img.height as _,
                        1, //15,
                        false, //true,
                        false,
                        true,
                        &data
                    )
                }
            }
        }
    }

    ///
    /// Creates a sampler
    ///
    pub fn create_sampler(&self, data: SamplerData) -> Result<Sampler, ()> {
        Sampler::new(self.device, data)
    }

    ///
    /// Creates a render target for the specified display
    ///
    pub fn create_render_target(&self, display: &Display) -> Result<RenderTarget, ()> {
        RenderTarget::from_display(self.device, display.swap_chain)
    }

    ///
    /// Creates a new depth stencil target
    ///
    pub fn create_depth_stencil_target(&self, format: DepthStencilFormat, width: u32, height: u32)
        -> Result<DepthStencilTarget, ()> {

        DepthStencilTarget::new(self.device, format, width, height)
    }

    ///
    /// Creates a vertex buffer
    ///
    pub fn create_vertex_buffer<T>(&self, format: &VertexFormat, data: &[T])
        -> Result<VertexBuffer, ()> {

        let num_verts = data.len() as u32;
        let data_ptr = unsafe { std::mem::transmute::<*const T, *const u8>(data.as_ptr()) };

        VertexBuffer::new(self.device, format, num_verts, data_ptr)
    }

    ///
    /// Creates an index buffer
    ///
    pub fn create_index_buffer(&self, indices: &[u32]) -> Result<IndexBuffer, ()> {
        IndexBuffer::new(self.device, indices)
    }

    ///
    /// Creates a dynamic constant buffer
    ///
    pub fn create_constant_buffer(&self, size: usize) -> Result<ShaderBuffer, ()> {
        ShaderBuffer::new_constant_buffer(self.device, size)
    }

    ///
    /// Creates a shader input from a texture
    ///
    pub fn create_texture_shader_input(&self, tex: &Texture) -> Result<ShaderInput, ()> {
        ShaderInput::new(self.device, tex.res)
    }

    ///
    /// Creates a vertex shader for the specified blob
    ///
    pub fn create_vertex_shader(&self, code: &ShaderCode) -> Result<Shader, ()> {
        let mut vs = std::ptr::null_mut::<d3d11::ID3D11VertexShader>();
        let hr = unsafe {
            (*self .device).CreateVertexShader(
                (*code.data).GetBufferPointer(),
                (*code.data).GetBufferSize(),
                std::ptr::null_mut(),
                &mut vs as *mut *mut _)
        };

        if hr != winerror::S_OK {
            Err(())
        }
        else {
            Ok(Shader::Vertex(vs))
        }
    }

    ///
    /// Creates a pixel shader for the specified blob
    ///
    pub fn create_pixel_shader(&self, code: &ShaderCode) -> Result<Shader, ()> {
        let mut ps = std::ptr::null_mut::<d3d11::ID3D11PixelShader>();
        let hr = unsafe {
            (*self.device).CreatePixelShader(
                (*code.data).GetBufferPointer(),
                (*code.data).GetBufferSize(),
                std::ptr::null_mut(),
                &mut ps as *mut *mut _)
        };

        if hr != winerror::S_OK {
            Err(())
        }
        else {
            Ok(Shader::Pixel(ps))
        }
    }

    ///
    /// Creates an input layout from the specified vertex format and vertex shader code
    ///
    pub fn create_input_layout(&self, format: &VertexFormat, vs_code: &ShaderCode)
        -> Result<InputLayout, ()> {

        const VERTEX_SEMANTIC_NAMES: [&'static str; 5] = [
            &"POSITION\0",
            &"NORMAL\0",
            &"TANGENT\0",
            &"BITANGENT\0",
            &"TEXCOORD\0",
        ];

        const VERTEX_INPUT_FORMATS: [dxgiformat::DXGI_FORMAT; 19] = [
            dxgiformat::DXGI_FORMAT_R8_UNORM,
            dxgiformat::DXGI_FORMAT_R8G8_UNORM,
            dxgiformat::DXGI_FORMAT_R8G8B8A8_UNORM,
            dxgiformat::DXGI_FORMAT_R8_SNORM,
            dxgiformat::DXGI_FORMAT_R8G8_SNORM,
            dxgiformat::DXGI_FORMAT_R8G8B8A8_SNORM,
            dxgiformat::DXGI_FORMAT_R16_UNORM,
            dxgiformat::DXGI_FORMAT_R16G16_UNORM,
            dxgiformat::DXGI_FORMAT_R16G16B16A16_UNORM,
            dxgiformat::DXGI_FORMAT_R16_SNORM,
            dxgiformat::DXGI_FORMAT_R16G16_SNORM,
            dxgiformat::DXGI_FORMAT_R16G16B16A16_SNORM,
            dxgiformat::DXGI_FORMAT_R16_FLOAT,
            dxgiformat::DXGI_FORMAT_R16G16_FLOAT,
            dxgiformat::DXGI_FORMAT_R16G16B16A16_FLOAT,
            dxgiformat::DXGI_FORMAT_R32_FLOAT,
            dxgiformat::DXGI_FORMAT_R32G32_FLOAT,
            dxgiformat::DXGI_FORMAT_R32G32B32_FLOAT,
            dxgiformat::DXGI_FORMAT_R32G32B32A32_FLOAT,
        ];

        const VERTEX_INPUT_CLASSIFICATIONS: [d3d11::D3D11_INPUT_CLASSIFICATION; 2] = [
            d3d11::D3D11_INPUT_PER_VERTEX_DATA,
            d3d11::D3D11_INPUT_PER_INSTANCE_DATA
        ];

        // create input element descriptors for each element in our format
        let mut elements = Vec::with_capacity(format.num_inputs as usize);
        for i in 0..(format.num_inputs as usize) {
            let input = &format.inputs[i];
            let semstr = VERTEX_SEMANTIC_NAMES[input.semantic as usize];
            let e = d3d11::D3D11_INPUT_ELEMENT_DESC {
                SemanticName: CStr::from_bytes_with_nul(semstr.as_bytes()).expect("Sanity check failed").as_ptr(),
                SemanticIndex: input.semantic_index,
                Format: VERTEX_INPUT_FORMATS[input.format as usize],
                InputSlot: input.input_slot,
                AlignedByteOffset: input.byte_offset,
                InputSlotClass: VERTEX_INPUT_CLASSIFICATIONS[input.slot_class as usize],
                InstanceDataStepRate: input.instance_step_rate
            };

            elements.push(e);
        }

        let mut layout = std::ptr::null_mut::<d3d11::ID3D11InputLayout>();
        let hr = unsafe {
            (*self.device).CreateInputLayout(
                elements.as_ptr(),
                elements.len() as _,
                (*vs_code.data).GetBufferPointer(),
                (*vs_code.data).GetBufferSize(),
                &mut layout as *mut *mut _
            )
        };

        if hr != winerror::S_OK {
            Err(())
        }
        else {
            Ok(InputLayout { data: layout })
        }
    }

    ///
    /// Maps the buffer for writing to CPU memory and returns the mutable pointer
    ///
    pub unsafe fn map_buffer_unsafe<T>(&self, buffer: &ShaderBuffer) -> (*mut T, usize) {
        let mut map = d3d11::D3D11_MAPPED_SUBRESOURCE {
            pData: std::ptr::null_mut(),
            RowPitch: 0,
            DepthPitch: 0
        };
        let hr = (*self.context).Map(buffer.buff as _, 0, d3d11::D3D11_MAP_WRITE_DISCARD, 0,
            &mut map);

        if hr != winerror::S_OK {
            (std::ptr::null_mut(), 0)
        }
        else {
            (map.pData as *mut T, map.RowPitch as usize)
        }
    }

    ///
    /// Unmaps the buffer and uploads the contents to the GPU
    ///
    pub unsafe fn unmap_buffer_unsafe(&self, buffer: &ShaderBuffer) {
        (*self.context).Unmap(buffer.buff as _, 0);
    }

    ///
    /// Sets the contents of the dynamic buffer
    ///
    pub fn map_and_set_buffer_data<T: Clone>(&self, buffer: &ShaderBuffer, data: &T) {
        unsafe {
            let (p, s) = self.map_buffer_unsafe::<T>(buffer);
            assert!(!p.is_null(), "Failed to map a buffer!");
            assert!(std::mem::size_of::<T>() <= s, "Mapped size too small for data");

            *p = data.clone();

            self.unmap_buffer_unsafe(buffer);
        }
    }

    ///
    /// Sets the contents of the dynamic buffer
    ///
    pub fn map_and_set_buffer_data_from_slice<T>(&self, _buffer: &ShaderBuffer, _data: &[T]) {
        assert!(false, "Not implemented!");
    }

    ///
    /// Clears the specified render target to the specified color
    ///
    pub fn clear_render_target(&self, rt: &RenderTarget, rgba: &Color4F) {
        let c = [rgba.r, rgba.g, rgba.b, rgba.a];
        unsafe { (*self.context).ClearRenderTargetView(rt.rtv, &c); }
    }

    ///
    /// Clears the specified depth target to the specified values
    ///
    pub fn clear_depth_stencil_target(&self, ds: &DepthStencilTarget, clear_depth: bool,
        depth: f32, clear_stencil: bool, stencil: u8) {
        let mut clear_flags = 0;
        if clear_depth {
            clear_flags |= d3d11::D3D11_CLEAR_DEPTH;
        }
        if clear_stencil {
            clear_flags |= d3d11::D3D11_CLEAR_STENCIL;
        }
        if clear_flags != 0 {
            unsafe { (*self.context).ClearDepthStencilView(ds.dsv, clear_flags, depth, stencil); }
        }
    }

    ///
    /// Sets the current render targets
    ///
    pub fn set_render_target(&self, rt: &RenderTarget) {
        unsafe { (*self.context).OMSetRenderTargets(1, &rt.rtv, std::ptr::null_mut()); }
    }

    ///
    /// Sets the current render target and depth buffer
    ///
    pub fn set_render_target_and_depth(&self, rt: &RenderTarget, ds: &DepthStencilTarget) {
        unsafe { (*self.context).OMSetRenderTargets(1, &rt.rtv, ds.dsv); }
    }

    ///
    /// Unbinds all render targets
    ///
    pub fn unbind_render_targets(&self) {
        unsafe { (*self.context).OMSetRenderTargets(0, std::ptr::null_mut(), std::ptr::null_mut()); }
    }

    ///
    /// Sets the viewport setup
    ///
    pub fn set_viewport(&self, x: f32, y: f32, w: f32, h: f32, min_z: f32, max_z: f32) {
        let vp = d3d11::D3D11_VIEWPORT {
            TopLeftX: x,
            TopLeftY: y,
            Width: w,
            Height: h,
            MinDepth: min_z,
            MaxDepth: max_z
        };
        unsafe { (*self.context).RSSetViewports(1, &vp); }
    }

    ///
    /// Sets the current primitive topology
    ///
    pub fn set_primitive_topology(&self, topology: PrimitiveTopology) {
        const PRIM_TOPOLOGIES: [d3d11::D3D11_PRIMITIVE_TOPOLOGY; 5] = [
            d3dcommon::D3D_PRIMITIVE_TOPOLOGY_POINTLIST,
            d3dcommon::D3D_PRIMITIVE_TOPOLOGY_LINELIST,
            d3dcommon::D3D_PRIMITIVE_TOPOLOGY_LINESTRIP,
            d3dcommon::D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST,
            d3dcommon::D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP
        ];

        unsafe { (*self.context).IASetPrimitiveTopology(PRIM_TOPOLOGIES[topology as usize]); }
    }

    ///
    /// Sets the current input layout
    ///
    pub fn set_input_layout(&self, layout: &InputLayout) {
        unsafe { (*self.context).IASetInputLayout(layout.data); }
    }

    ///
    /// Sets the current vertex buffer
    ///
    pub fn set_vertex_buffer(&self, vb: &VertexBuffer, slot: u32) {
        let offset: u32 = 0;
        unsafe { (*self.context).IASetVertexBuffers(slot, 1, &vb.buff, &vb.format.stride, &offset); }
    }

    ///
    /// Sets the current index buffer
    ///
    pub fn set_index_buffer(&self, ib: &IndexBuffer) {
        unsafe { (*self.context).IASetIndexBuffer(ib.buff, dxgiformat::DXGI_FORMAT_R32_UINT, 0) }
    }

    ///
    /// Sets the current vertex shader
    ///
    pub fn set_vertex_shader(&self, shader: &Shader) {
        match *shader {
            Shader::Vertex(vs) => unsafe { (*self.context).VSSetShader(vs, std::ptr::null(), 0) }
            _ => panic!("Specified shader is not a vertex shader!")
        }
    }

    ///
    /// Sets a vertex shader constant buffer
    ///
    pub fn set_vs_constant_buffer(&self, slot: u32, buffer: &ShaderBuffer) {
        unsafe { (*self.context).VSSetConstantBuffers(slot, 1, &buffer.buff); }
    }

    ///
    /// Sets a vertex shader input
    ///
    pub fn set_vs_shader_input(&self, slot: u32, input: &ShaderInput) {
        unsafe { (*self.context).VSSetShaderResources(slot, 1, &input.srv); }
    }

    ///
    /// Sets a vertex sampler
    ///
    pub fn set_vs_sampler(&self, slot: u32, input: &Sampler) {
        unsafe { (*self.context).VSSetSamplers(slot, 1, &input.samp); }
    }

    ///
    /// Sets the current pixel shader
    ///
    pub fn set_pixel_shader(&self, shader: &Shader) {
        match *shader {
            Shader::Pixel(ps) => unsafe { (*self.context).PSSetShader(ps, std::ptr::null(), 0) }
            _ => panic!("Specified shader is not a pixel shader!")
        }
    }

    ///
    /// Sets a pixel shader constant buffer
    ///
    pub fn set_ps_constant_buffer(&self, slot: u32, buffer: &ShaderBuffer) {
        unsafe { (*self.context).PSSetConstantBuffers(slot, 1, &buffer.buff); }
    }

    ///
    /// Sets a pixel shader input
    ///
    pub fn set_ps_shader_input(&self, slot: u32, input: &ShaderInput) {
        unsafe { (*self.context).PSSetShaderResources(slot, 1, &input.srv); }
    }

    ///
    /// Sets a pixel sampler
    ///
    pub fn set_ps_sampler(&self, slot: u32, input: &Sampler) {
        unsafe { (*self.context).PSSetSamplers(slot, 1, &input.samp); }
    }

    ///
    /// Draws the specified vertex range
    ///
    pub fn draw(&self, num_verts: u32, start_vert: u32) {
        unsafe { (*self.context).Draw(num_verts, start_vert); }
    }

    ///
    /// Draws the specified index range
    ///
    pub fn draw_indexed(&self, num_indices: u32, start_index: u32) {
        unsafe { (*self.context).DrawIndexed(num_indices, start_index, 0); }
    }
}

impl Drop for Graphics {
    ///
    /// Cleans up the internal graphics device when dropped
    ///
    fn drop(&mut self) {
        if !self.device.is_null() {
            unsafe { (*self.device).Release(); }
        }
        if !self.context.is_null() {
            unsafe { (*self.context).Release(); }
        }
    }
}
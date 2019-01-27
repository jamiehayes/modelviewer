use std::ptr;
use winapi::um::d3d11;
use winapi::shared::dxgi;
use winapi::shared::winerror;

///
/// Interface for a graphics render target
///
pub struct RenderTarget {
    pub rtv: *mut d3d11::ID3D11RenderTargetView
}

impl RenderTarget {
    ///
    /// Creates a new render target object from the specified swap chain
    ///
    pub fn from_display(device: *mut d3d11::ID3D11Device, swap_chain: *mut dxgi::IDXGISwapChain)
        -> Result<RenderTarget, ()> {

        // get the texture resource from the swap chain
        let mut tex = ptr::null_mut::<d3d11::ID3D11Resource>();
        let mut hr = unsafe {
            (*swap_chain).GetBuffer(
                0,
                &d3d11::IID_ID3D11Resource,
                &mut tex as *mut *mut _ as *mut *mut _)
        };
        if hr != winerror::S_OK {
            return Err(());
        }

        // create a render target view for the texture
        let mut rtv = ptr::null_mut::<d3d11::ID3D11RenderTargetView>();
        hr = unsafe { (*device).CreateRenderTargetView(tex, ptr::null(), &mut rtv) };

        // we don't need this reference to the texture anymore
        unsafe { (*tex).Release() };

        if hr != winerror::S_OK {
            Err(())
        }
        else {
            // create RenderTarget object with the render target view
            Ok(RenderTarget { rtv })
        }
    }
}

impl Drop for RenderTarget {
    ///
    /// Cleans up the internal render target when dropped
    ///
    fn drop(&mut self) {
        if !self.rtv.is_null() {
            unsafe { (*self.rtv).Release(); }
        }
    }
}
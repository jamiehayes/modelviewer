use std::ptr;
use winapi::um::winuser;
use winapi::um::d3d11;
use winapi::shared::dxgi;
use winapi::shared::dxgitype;
use winapi::shared::dxgiformat;
use winapi::shared::winerror;
use winapi::shared::windef;

///
/// Implements functionality for a graphics display
///
pub struct Display {
    pub swap_chain: *mut dxgi::IDXGISwapChain
}

impl Display {
    ///
    /// Creates a new display object
    ///
    pub fn new(device: *mut d3d11::ID3D11Device, hwnd: u64) -> Result<Display, ()> {
        // first, get the DXGI device from the D3D11 device
        let mut dxgidevice = ptr::null_mut::<dxgi::IDXGIDevice>();
        let mut hr = unsafe {
            (*device).QueryInterface(
                &dxgi::IID_IDXGIDevice,
                &mut dxgidevice as *mut *mut _ as *mut *mut _)
        };
        if hr != winerror::S_OK {
            println!("Failed to get DXGI device from D3D11 device. Error: {}", hr);
            return Err(());
        }

        // then, get the adapter used to create the device
        let mut adapter = ptr::null_mut::<dxgi::IDXGIAdapter>();
        hr = unsafe { (*dxgidevice).GetAdapter(&mut adapter) };
        if hr != winerror::S_OK {
            println!("Failed to get adapter from dxgidevice. Error: {}", hr);
            unsafe { (*dxgidevice).Release() };
            return Err(());
        }

        // now get the DXGI factory from the adapter
        let mut factory = ptr::null_mut::<dxgi::IDXGIFactory>();
        hr = unsafe {
            (*adapter).GetParent(
                &dxgi::IID_IDXGIFactory,
                &mut factory as *mut *mut _ as *mut *mut _)
        };
        if hr != winerror::S_OK {
            println!("Failed to get DXGI factory from adapter. Error: {}", hr);
            unsafe {
                (*dxgidevice).Release();
                (*adapter).Release();
            }
            return Err(());
        }

        // get the initial size of the window for which we're creating the device
        let mut rect = windef::RECT { left: 0, right: 0, top: 0, bottom: 0 };
        unsafe { winuser::GetClientRect(hwnd as _, &mut rect) };

        // use the factory to create a swap chain for the window
        let mut desc = dxgi::DXGI_SWAP_CHAIN_DESC {
            BufferDesc: dxgitype::DXGI_MODE_DESC {
                Width: rect.right as u32,
                Height: rect.bottom as u32,
                RefreshRate: dxgitype::DXGI_RATIONAL {
                    Numerator: 1,
                    Denominator: 60
                },
                Format: dxgiformat::DXGI_FORMAT_R8G8B8A8_UNORM_SRGB,
                ScanlineOrdering: 0,
                Scaling: 0,
            },
            SampleDesc: dxgitype::DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0
            },
            BufferUsage: dxgitype::DXGI_USAGE_BACK_BUFFER |
                         dxgitype::DXGI_USAGE_RENDER_TARGET_OUTPUT,
            BufferCount: 1,
            OutputWindow: hwnd as _,
            Windowed: 1,
            SwapEffect: dxgi::DXGI_SWAP_EFFECT_DISCARD,
            Flags: 0
        };
        let mut sc = ptr::null_mut::<dxgi::IDXGISwapChain>();
        hr = unsafe { (*factory).CreateSwapChain(device as *mut _, &mut desc, &mut sc) };

        // release these interfaces now that we're done with them so we don't leak them
        unsafe {
            (*dxgidevice).Release();
            (*adapter).Release();
            (*factory).Release();
        }

        if hr != winerror::S_OK {
            println!("Failed to create swap chain for window. Error: {}", hr);
            Err(())
        }
        else {
            // create Display object from the created swap chain
            Ok(Display { swap_chain: sc })
        }
    }

    ///
    /// Updates the image on the physical display
    ///
    pub fn present(&self) {
        unsafe { (*self.swap_chain).Present(1, 0); }
    }
}

impl Drop for Display {
    ///
    /// Cleans up the internal state when dropped
    ///
    fn drop(&mut self) {
        if !self.swap_chain.is_null() {
            unsafe { (*self.swap_chain).Release(); }
        }
    }
}
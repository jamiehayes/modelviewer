use std::ptr;
use std::os::windows::ffi::OsStrExt;
use std::ffi::OsStr;
use winapi::um::winuser;
use winapi::shared::windef;
use winapi::shared::minwindef;

///
/// Generates a wide string from a Rust string slice
///
fn to_wstring(s : &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(Some(0).into_iter()).collect()
}

///
/// Window procedure for the application window
///
unsafe extern "system" fn window_proc(hwnd: windef::HWND, msg: u32,
    wparam: minwindef::WPARAM, lparam: minwindef::LPARAM) -> minwindef::LRESULT {

    match msg {
        winuser::WM_CLOSE => {
            winuser::DestroyWindow(hwnd);
            0
        }
        winuser::WM_DESTROY => {
            winuser::PostQuitMessage(0);
            0
        }
        _ => {
            winuser::DefWindowProcW(hwnd, msg, wparam, lparam)
        }
    }
}

///
/// Data for an application window
///
pub struct Window {
    handle: windef::HWND
}

impl Window {
    ///
    /// Creates an application window
    ///
    pub fn create(title: &str, width: u32, height: u32) -> Window {
        let class_name = to_wstring("AppWindow");
        let wnd_title = to_wstring(title);
        let class = winuser::WNDCLASSW {
            style: 0,
            lpfnWndProc: Some(window_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: 0 as _,
            hIcon: unsafe { winuser::LoadIconW(0 as _, winuser::IDI_APPLICATION) },
            hCursor: unsafe { winuser::LoadCursorW(0 as _, winuser::IDI_APPLICATION) },
            hbrBackground: 16 as _,
            lpszMenuName: 0 as _,
            lpszClassName: class_name.as_ptr(),
        };

        unsafe { winuser::RegisterClassW(&class); }

        let handle = unsafe {
            winuser::CreateWindowExW(
                0,
                class_name.as_ptr(),
                wnd_title.as_ptr(),
                winuser::WS_OVERLAPPEDWINDOW,
                winuser::CW_USEDEFAULT,
                winuser::CW_USEDEFAULT,
                width as i32,
                height as i32,
                0 as _,
                0 as _,
                0 as _,
                0 as _)
        };

        Window {
            handle
        }
    }

    ///
    /// Shows or hides the window
    ///
    pub fn show(&self, show: bool) {
        let cmd = if show { winuser::SW_SHOW } else { winuser::SW_HIDE };
        unsafe { winuser::ShowWindow(self.handle, cmd); }
    }

    ///
    /// Processes window messages until the message queue is empty
    ///
    /// # Returns
    /// `false` if the application should exit, `true` otherwise
    ///
    pub fn process_messages(&self) -> bool {
        let mut msg = winuser::MSG {
            hwnd: ptr::null_mut(),
            message: 0,
            lParam: 0,
            wParam: 0,
            pt: windef::POINT { x: 0, y: 0 },
            time: 0
        };

        unsafe {
            if winuser::PeekMessageW(&mut msg, ptr::null_mut(), 0, 0, winuser::PM_REMOVE) != 0 {
                winuser::TranslateMessage(&mut msg);
                winuser::DispatchMessageW(&mut msg);

                if msg.message == winuser::WM_QUIT {
                    return false;
                }
            }
        }

        true
    }

    ///
    /// Destroys the window
    ///
    pub fn destroy(&mut self) {
        if self.handle != ptr::null_mut() {
            unsafe { winuser::DestroyWindow(self.handle); }
            self.handle = ptr::null_mut();
        }
    }

    ///
    /// Accesses the internal handle to the window
    ///
    pub fn get_handle(&self) -> u64 {
        self.handle as u64
    }

    ///
    /// Gets the size of the window
    ///
    pub fn get_window_size(&self) -> (u32, u32) {
        let mut r = windef::RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0
        };
        unsafe { winuser::GetClientRect(self.handle, &mut r); }

        (r.right as _, r.bottom as _)
    }
}

impl Drop for Window {
    ///
    /// Cleans up and properly destroys the window when dropped
    ///
    fn drop(&mut self) {
        self.destroy();
    }
}
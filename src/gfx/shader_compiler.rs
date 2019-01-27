// external refs
use winapi::um::d3dcommon;
use winapi::um::d3dcompiler;
use winapi::shared::winerror;
use std::ffi::{CString, OsStr};
use std::os::windows::ffi::OsStrExt;
use std::path::{Path, PathBuf};

// local refs
use crate::gfx::ShaderType;

///
/// Generates a wide string from a Rust string slice
///
fn to_wstring(s : &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(Some(0).into_iter()).collect()
}

///
/// Helper struct to hold blobs
///
pub struct ShaderCode {
    pub data: *mut d3dcommon::ID3DBlob
}

impl ShaderCode {
    ///
    /// Retrieve the buffer as a memory slice
    ///
    pub fn to_slice<T>(&self) -> Result<&[T], ()> {
        if self.data.is_null() {
            Err(())
        }
        else {
            unsafe {
                let p = (*self.data).GetBufferPointer() as *const T;
                let s = (*self.data).GetBufferSize();

                Ok(std::slice::from_raw_parts(p, s))
            }
        }
    }

    ///
    /// Retrieve the buffer as a string slice
    ///
    pub fn to_str(&self) -> Result<&str, ()> {
        let v = self.to_slice()?;

        std::str::from_utf8(v).map_err(|_| { })
    }
}

impl Drop for ShaderCode {
    fn drop(&mut self) {
        if !self.data.is_null() {
            unsafe { (*self.data).Release() };
        }
    }
}

///
/// Utility class used to compile and generate shaders
///
pub struct ShaderCompiler {
    path: PathBuf,
    defines: Vec<(String, Option<String>)>
}

impl ShaderCompiler {
    const SHADER_PROFILES: [&'static str; 2] = [
        "vs_5_0",
        "ps_5_0"
    ];

    ///
    /// Creates a new shader compiler to compile from file
    ///
    pub fn from_file(path: &Path) -> Self {
        Self { path: path.to_path_buf(), defines: Vec::new() }
    }

    ///
    /// Adds a define to the shader compilation
    ///
    pub fn add_define(mut self, name: String, value: Option<String>) -> Self {
        self.defines.push((name, value));

        self
    }

    pub fn compile(&self, entry_point: &str, profile: ShaderType) -> Result<ShaderCode, ()> {
        let wfname = to_wstring(self.path.to_str().unwrap());
        let mut tempstrs = Vec::<CString>::with_capacity(self.defines.len() * 2);
        let mut defs = Vec::<d3dcommon::D3D_SHADER_MACRO>::with_capacity(self.defines.len());
        for (name, optvalue) in &self.defines {
            if name.is_empty() {
                continue;
            }

            tempstrs.push(CString::new(name.as_bytes()).expect("Invalid define name encountered"));
            let mut new_define = d3dcommon::D3D_SHADER_MACRO {
                Name: tempstrs.last().expect("Sanity check failed").as_ptr(),
                Definition: std::ptr::null(),
            };

            if let Some(value) = optvalue {
                tempstrs.push(CString::new(value.as_bytes()).expect("Invalid define value encountered"));
                new_define.Definition = tempstrs.last().expect("Sanity check failed").as_ptr();
            }

            defs.push(new_define);
        }

        let mut code_blob = ShaderCode { data: std::ptr::null_mut() };
        let mut error_blob = ShaderCode { data: std::ptr::null_mut() };
        let prof_str = ShaderCompiler::SHADER_PROFILES[profile as usize];
        let hr = unsafe {
            d3dcompiler::D3DCompileFromFile(
                wfname.as_ptr(),
                if defs.len() > 0 { &defs[0] as _ } else { std::ptr::null() },
                d3dcompiler::D3D_COMPILE_STANDARD_FILE_INCLUDE,
                CString::new(entry_point.as_bytes()).expect("Invalid entry point name").as_ptr(),
                CString::new(prof_str.as_bytes()).expect("Invalid profile string").as_ptr(),
                d3dcompiler::D3DCOMPILE_OPTIMIZATION_LEVEL3,
                0,
                &mut code_blob.data as *mut *mut _,
                &mut error_blob.data as *mut *mut _
            )
        };

        if hr != winerror::S_OK {
            println!("Failed to compile {:?} ({:#x})", self.path, hr);

            // attempt to print errors
            if let Ok(e) = error_blob.to_str() {
                println!("{}", e);
            }

            Err(())
        }
        else {
            Ok(code_blob)
        }
    }
}
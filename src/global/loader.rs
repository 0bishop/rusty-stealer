use winapi::um::libloaderapi::{LoadLibraryA, GetProcAddress, FreeLibrary};
use winapi::shared::minwindef::FARPROC;
use std::ffi::CString;

pub struct LoadMod {
    dll_handle : *mut winapi::shared::minwindef::HINSTANCE__,
}

impl LoadMod {
    pub fn new(path : &str) -> Result<Self, String> {
        let dll_name = CString::new(path).expect("CString conversion failed");
        let dll_handle: *mut winapi::shared::minwindef::HINSTANCE__ = unsafe { LoadLibraryA(dll_name.as_ptr()) };
    
        if dll_handle.is_null() {
            return Err(format!("Failed to load Module: {}", path));
        }
    
        Ok(LoadMod { dll_handle })
    }

    pub fn get_function(&self, function_name : &str) -> Result<FARPROC, String> {
        // Load the function pointer dynamically
        let function_name_cstr = CString::new(function_name).expect("CString conversion failed");
        let function_ptr = unsafe {
            GetProcAddress(self.dll_handle, function_name_cstr.as_ptr())
        };

        if function_ptr.is_null() {
            return Err(function_name.to_string());
        }

        Ok(function_ptr)
    }
}

// implement a trait to free the library when done
impl Drop for LoadMod {
    fn drop(&mut self) {
        let free = unsafe { FreeLibrary(self.dll_handle) };
        if free == 0 {
            eprintln!("Failed to free Module");
        }
    }
}

// macro to call the function, and convert the function pointer to the defined function type
macro_rules! call_function {
    ($dll_handle:expr, $type:ty, $name:expr) => {
        {
            // Change crate::loader to your mod name
            let function: $type =  unsafe { std::mem::transmute(crate::global::loader::LoadMod::get_function($dll_handle, $name).expect("Failed to get ptr")) };
            function
        }
    };
}

pub(crate) use call_function;
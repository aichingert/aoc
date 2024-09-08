use std::ffi::{CString, c_int};
use glfw::ffi::GLFWwindow;

use ash::vk;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CRen {
    window: *mut GLFWwindow,

    instance: vk::Instance,
    surface: vk::SurfaceKHR,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Ren {
    pub a: i32,
    pub b: i32,
}

impl Ren {
    pub fn init(width: i32, height: i32, title: &str) -> Self {
        let c_ren = unsafe { 
            let c_str = CString::new(title).unwrap();
            ren_init(width as c_int, height as c_int, c_str.as_ptr() as *const u8)
        };

        return Self {
            a: 0,
            b: 1,
        };
    }
}

#[link(name = "ren", kind = "static")]
extern "C" {
    fn ren_init(width: c_int, height: c_int, title: *const u8) -> CRen;
}

 //use crate::ffi::GLFWwindow;
use glfw::fii::GLFWwindow;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CRen {
    window: *mut GLFWwindow,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Ren {
    pub a: i32,
    pub b: i32,
}

extern "C" {

}

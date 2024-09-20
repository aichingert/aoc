use std::ffi::CString;
use glfw::ffi::GLFWwindow;

#[repr(C)]
pub struct Rulkan {
}

#[repr(C)]
pub struct CRen {
    window: *mut GLFWwindow,
    rulkan: Rulkan,

    frame: u32,
}

pub struct Ren {
    c_ren: CRen,
}

impl Ren {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        Self {
            c_ren: unsafe {
                let c_str = CString::new(title).expect("CString new failed");
                ren_init(width, height, c_str.into_raw() as *const u8)
            },
        }
    }
}

#[link(name = "ren", kind = "static")]
extern "C" {
    fn ren_init(width: u32, height: u32, title: *const u8) -> CRen;

    fn ren_draw(ren: *mut CRen);

    fn ren_destroy(ren: *mut CRen);

}

use std::ffi::CString;
use glfw::ffi::GLFWwindow;

use ash::vk;

#[repr(C)]
pub struct Vec2 {
    pub x: core::ffi::c_float,
    pub y: core::ffi::c_float,
}

#[repr(C)]
pub struct VertexBuffer {
    buffer: vk::Buffer,
    memory: vk::DeviceMemory,
}

#[repr(C)]
pub struct FrameData {
    render_sema: vk::Semaphore,
    present_sema: vk::Semaphore,
    render_fence: vk::Fence,

    vb: VertexBuffer,

    command_pool: vk::CommandPool,
    command_buffer: vk::CommandBuffer,
}

#[repr(C)]
pub struct Swapchain {
    this: vk::SwapchainKHR,

    extent: vk::Extent2D,
    format: vk::Format,

    images: *mut vk::Image,
    image_views: *mut vk::ImageView,
    framebuffers: *mut vk::Framebuffer,

    size: usize,
}

#[repr(C)]
pub struct Rulkan {
    instance: vk::Instance,
    surface: vk::SurfaceKHR,

    device: vk::Device,
    physical_device: vk::PhysicalDevice,

    present_queue: vk::Queue,
    graphics_queue: vk::Queue,

    swapchain: Swapchain,
    render_pass: vk::RenderPass,

    graphics_pipeline: vk::Pipeline,
    pipeline_layout: vk::PipelineLayout,

    frames: [FrameData; 2],
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
                ren_init(width, height, c_str.as_ptr())
            },
        }
    }

    pub fn triangle(&mut self, a: Vec2, b: Vec2, c: Vec2) {
        unsafe { ren_draw_triangle(&mut self.c_ren, a, b, c); }
    }

    pub fn draw(&mut self) {
        unsafe { ren_draw_frame(&mut self.c_ren); }
    }
}

impl Drop for Ren {
    fn drop(&mut self) {
        unsafe { ren_destroy(&mut self.c_ren); }
    }
}

#[link(name = "ren", kind = "static")]
extern "C" {
    fn ren_init(width: u32, height: u32, title: *const core::ffi::c_char) -> CRen;

    fn ren_draw_triangle(ren: *mut CRen, a: Vec2, b: Vec2, c: Vec2);
    fn ren_draw_frame(ren: *mut CRen);

    fn ren_destroy(ren: *mut CRen);

}

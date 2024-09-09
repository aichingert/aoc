use std::ffi::CString;
use glfw::ffi::GLFWwindow;

use ash::vk;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct CRen {
    window: *mut GLFWwindow,

    instance: vk::Instance,
    surface: vk::SurfaceKHR,

    device: vk::Device,
    physical_device: vk::PhysicalDevice,

    present_queue: vk::Queue,
    graphics_queue: vk::Queue,

    swap_chain: vk::SwapchainKHR,
    swap_chain_extent: vk::Extent2D,
    swap_chain_images: *mut vk::Image,
    swap_chain_image_views: *mut vk::ImageView,
    swap_chain_images_size: usize,
    swap_chain_image_format: vk::Format,
    swap_chain_framebuffers: *mut vk::Framebuffer,
    
    render_pass: vk::RenderPass,
    pipeline_layout: vk::PipelineLayout,
    graphics_pipeline: vk::Pipeline,

    command_pool: vk::CommandPool,
    command_buffers: *mut vk::CommandBuffer,

    vertex_buffer: vk::Buffer,
    vertex_buffer_memory: vk::DeviceMemory,
    index_buffer: vk::Buffer,
    index_buffer_memory: vk::DeviceMemory,

    in_flight_fences: *mut vk::Fence,
    image_available_semaphores: *mut vk::Semaphore,
    render_finished_semaphores: *mut vk::Semaphore,

    current_frame: u32,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Ren {
    c_ren: CRen,
}

impl Ren {
    pub fn init(width: u32, height: u32, title: &str) -> Self {
        return Self { 
            c_ren: unsafe { 
                let c_str = CString::new(title).unwrap();
                ren_init(width, height, c_str.as_ptr() as *const u8)
            },
        };
    }

    pub fn draw_frame(&self) {
        unsafe { ren_draw_frame(&self.c_ren); }
    }
}

impl Drop for Ren {
    fn drop(&mut self) {
        unsafe { ren_destroy(&mut self.c_ren); }
    }
}

#[link(name = "ren", kind = "static")]
extern "C" {
    fn ren_init(width: u32, height: u32, title: *const u8) -> CRen;
    fn ren_destroy(ren: *mut CRen);

    fn ren_draw_frame(ren: *const CRen);
}

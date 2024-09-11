/*
use std::env;
use std::path::Path;
use std::process::Command;
*/

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let path = String::from("ren/src/");
    let vk = String::from("ren/src/vulkan/");

    println!("cargo:rustc-link-lib=glfw");
    println!("cargo:rustc-link-lib=vulkan");

    cc::Build::new()
        .cpp(true)
        .file(format!("{}ren.h", path))
        .file(format!("{}ren.cpp", path))
        .file(format!("{}window.cpp", path))
        .file(format!("{}vulkan.h", vk))
        .file(format!("{}vulkan.cpp", vk))
        .file(format!("{}buffers.cpp", vk))
        .file(format!("{}device.cpp", vk))
        .file(format!("{}pipeline.cpp", vk))
        .file(format!("{}instance.cpp", vk))
        .file(format!("{}queue.h", vk))
        .file(format!("{}shaders.cpp", vk))
        .file(format!("{}swap_chain.cpp", vk))
        .file(format!("{}vertex.h", vk))
        .static_flag(true)
        .compile("ren"); 

    //println!("cargo:rerun-if-changed=libren.a");
}

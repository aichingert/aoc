fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let path = String::from("ren/src/");
    let rulk = String::from("ren/src/rulkan/");
    let wind = String::from("ren/src/window/");

    println!("cargo:rustc-link-lib=glfw");
    println!("cargo:rustc-link-lib=vulkan");

    cc::Build::new()
        .cpp(true)
        .file(format!("{}ren.cpp", path))
        .file(format!("{}window.cpp", wind))
        .file(format!("{}rulkan.cpp", rulk))
        .file(format!("{}buffer.cpp", rulk))
        .file(format!("{}command.cpp", rulk))
        .file(format!("{}device.cpp", rulk))
        .file(format!("{}instance.cpp", rulk))
        .file(format!("{}pipeline.cpp", rulk))
        .file(format!("{}queue.cpp", rulk))
        .file(format!("{}shader.cpp", rulk))
        .file(format!("{}swapchain.cpp", rulk))
        .file(format!("{}sync.cpp", rulk))
        .file(format!("{}vertex.cpp", rulk))
        .static_flag(true)
        .compile("ren"); 

    println!("cargo:rerun-if-changed=libren.a");
}

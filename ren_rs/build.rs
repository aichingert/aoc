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

    /*
    let compiler = "zig";

    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = Path::new(&dir);

    env::set_current_dir(path.join("ren")).unwrap();

    Command::new(compiler)
        .args(&["build", "-Doptimize=ReleaseSafe"])
        .status()
        .expect("Failed to compile zig");

    env::set_current_dir(path).unwrap();

    println!(
        "cargo:rustc-link-search={}", 
        Path::new(&dir).join("ren/zig-out/lib/").display()
    );

    println!("cargo:rustc-link-lib=ren");
    println!("cargo:rustc-link-lib=dylib=stdc++");
    println!("cargo:rustc-link-lib=dl");
    println!("cargo:rustc-link-lib=glfw");
    */

    println!("cargo:rerun-if-changed=libren.a");

}

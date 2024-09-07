use std::env;
use std::path::Path;

use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let compiler = "zig";
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = Path::new(&dir);

    env::set_current_dir(path.join("ren")).unwrap();

    Command::new(compiler)
        .args(&["build", "-Drelease-fast"])
        .output()
        .expect("Failed to compile zig");

    env::set_current_dir(path).unwrap();

    println!("cargo:rustc-link-search=native{}", Path::new(&dir).join("ren/zig-cache/lib").display());
}

use std::env;

fn main() {
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let profile = std::env::var("PROFILE").unwrap();
    
    println!("cargo:rustc-link-search={}/{}", project_dir,profile); // the "-L" flag
    println!("cargo:rustc-link-lib=aom"); // the "-l" flag
}
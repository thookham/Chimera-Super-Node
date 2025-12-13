fn main() {
    // Placeholder for future C/C++ submodule compilation
    // println!("cargo:rustc-link-lib=static=tor");
    // println!("cargo:rustc-link-lib=static=i2pd");

    // Trigger rebuild if build.rs changes
    println!("cargo:rerun-if-changed=build.rs");
}

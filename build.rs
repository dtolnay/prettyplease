fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    println!("cargo:rustc-check-cfg=cfg(exhaustive)");
    println!("cargo:rustc-check-cfg=cfg(prettyplease_debug)");
    println!("cargo:rustc-check-cfg=cfg(prettyplease_debug_indent)");

    println!(concat!("cargo:VERSION=", env!("CARGO_PKG_VERSION")));
}

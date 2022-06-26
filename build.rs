use cmake;
extern crate pkg_config;

fn main() {
    let dst = cmake::Config::new("nativefiledialog-extended").build();

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static=nfd");

    #[cfg(target_os = "windows")]
    print_windows();

    #[cfg(target_os = "macos")]
    print_macos();

    #[cfg(target_os = "linux")]
    print_linux();
}

#[cfg(target_os = "windows")]
fn print_windows() {
    println!("cargo:rustc-link-lib=dylib=ole32");
    println!("cargo:rustc-link-lib=dylib=shell32");
}

#[cfg(target_os = "macos")]
fn print_macos() {
    println!("cargo:rustc-link-lib=framework=AppKit");
    println!("cargo:rustc-link-lib=framework=UniformTypeIdentifiers");
    // Needed by ___isPlatformVersionAtLeast, which is emitted by the @available macro
    println!("cargo:rustc-link-arg=-fapple-link-rtlib");
}

#[cfg(target_os = "linux")]
fn print_linux() {
    pkg_config::Config::new().probe("gtk+-3.0").unwrap();
}

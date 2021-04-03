use cmake;

fn main() {
    let dst = cmake::Config::new("nativefiledialog-extended").build();

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=lib/nfd");
    //println!("cargo:outdir={}", std::env::var("OUT_DIR").unwrap());
}

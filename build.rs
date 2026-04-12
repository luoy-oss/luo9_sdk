use std::env;
use std::path::PathBuf;

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_path = PathBuf::from(&manifest_dir).join("lib").join(&target_os);
    
    println!("cargo:rustc-link-search=native={}", lib_path.display());

    let target_dir = PathBuf::from(env::var("OUT_DIR").unwrap())
        .ancestors()
        .nth(3)
        .expect("Failed to find target directory")
        .to_path_buf();

    match target_os.as_str() {
        "windows" => {
            println!("cargo:rustc-link-lib=dylib=luo9_core");
            
            let src_dll = lib_path.join("luo9_core.dll");
            let dst_dll = target_dir.join("luo9_core.dll");
            if src_dll.exists() {
                let _ = std::fs::copy(src_dll, dst_dll);
            }
            
            let src_lib = lib_path.join("luo9_core.lib");
            let dst_lib = target_dir.join("luo9_core.lib");
            if src_lib.exists() {
                let _ = std::fs::copy(src_lib, dst_lib);
            }
        }
        "linux" => {
            println!("cargo:rustc-link-lib=dylib=luo9_core");
            println!("cargo:rustc-link-arg=-Wl,-rpath=$ORIGIN");
            
            let src_so = lib_path.join("libluo9_core.so");
            let dst_so = target_dir.join("libluo9_core.so");
            if src_so.exists() {
                let _ = std::fs::copy(src_so, dst_so);
            }
        }
        _ => {}
    }
    
    println!("cargo:rerun-if-changed={}", lib_path.display());
}
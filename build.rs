fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let header_path = {
        #[cfg(feature = "static")]
        {
            println!("cargo:rerun-if-changed=source/lzf_c.c");
            println!("cargo:rerun-if-changed=source/lzf_d.c");
            println!("cargo:rerun-if-changed=source/lzf.h");
            cc::Build::new()
                .warnings(false)
                .file("source/lzf_c.c")
                .file("source/lzf_d.c")
                .include("source")
                .compile("lzf");

            "source/lzf.h"
        }
        #[cfg(not(feature = "static"))]
        {
            println!("cargo:rustc-link-lib=lzf");
            "wrapper.h"
        }
    };
    let bindings = bindgen::Builder::default()
        .header(header_path)
        .generate()
        .expect("Could not generate bindings");

    let out_path = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("lzf_bindings.rs"))
        .expect("Failed to write bindings");
}

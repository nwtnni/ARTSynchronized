const ROOT: &str = env!("CARGO_MANIFEST_DIR");

fn main() {
    let files = [
        format!("{ROOT}/../ROWEX/Tree.cpp"),
        format!("{ROOT}/cpp/rowex.cpp"),
    ];

    let includes = [
        format!("{ROOT}/include"),
        format!("{ROOT}/../ROWEX/"),
        format!("{ROOT}/../"),
        format!("{ROOT}/../../mimalloc_rust/libmimalloc-sys/c_src/mimalloc/v3/include"),
    ];

    cxx_build::bridge("src/lib.rs")
        .cpp(true)
        .std("c++14")
        .files(&files)
        .includes(&includes)
        .flag("-march=native")
        .compile("art-sys");

    // Must be after linking against `ARTSynchronized`
    pkg_config::probe_library("tbb").expect("Could not find tbb");

    for path in files.iter().chain(&includes) {
        println!("cargo:rerun-if-changed={path}");
    }

    println!(
        "cargo:rustc-link-search=native={ROOT}/../../mimalloc_rust/libmimalloc-sys/c_src/mimalloc/v3/build/"
    );
    println!("cargo:rustc-link-lib=static=mimalloc");
}

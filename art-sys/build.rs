fn main() {
    let root = env!("CARGO_MANIFEST_DIR");

    cxx_build::bridge("src/lib.rs")
        .cpp(true)
        .file("cpp/rowex.cpp")
        .include("include")
        .include(format!("{root}/../"))
        .compile("art-sys");

    println!("cargo:rustc-link-search=native={root}/../build");
    println!("cargo:rustc-link-lib=static=ARTSynchronized");

    // Must be after linking against `ARTSynchronized`
    pkg_config::probe_library("mimalloc").expect("Could not find mimalloc");
    pkg_config::probe_library("tbb").expect("Could not find tbb");

    println!("cargo:rerun-if-changed={root}/cpp/rowex.cpp");
    println!("cargo:rerun-if-changed={root}/include/rowex.h");
}

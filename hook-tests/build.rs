fn main() {
    println!("cargo:rustc-link-lib=detours");
    println!("cargo:rustc-link-search=path/to/detours/lib");

    cc::Build::new()
        .cpp(true)
        .file("src/detours_wrapper.cpp")
        .compile("detours_wrapper");
}
#[cfg(target_os = "windows")]
fn main() -> std::io::Result<()> {
    use winres::WindowsResource;

    WindowsResource::new()
        .set("ProductName", "Cheat Finder")
        .set("LegalCopyright", format!("wrote for fydne project").as_str())
        //.set_manifest_file("./bins/installer_manifest.xml")
        .compile()
        .unwrap();
    
    println!("cargo:rustc-cfg=release");

    static_vcruntime::metabuild();
    
    Ok(())
}
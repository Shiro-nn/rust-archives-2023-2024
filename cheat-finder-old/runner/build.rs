#[cfg(target_os = "windows")]
fn main() -> std::io::Result<()> {
    use winres::WindowsResource;

    WindowsResource::new()
        .set("ProductName", "CheatChecker")
        .set("LegalCopyright", format!("wrote by fydne").as_str())
        //.set_manifest_file("./bins/installer_manifest.xml")
        .compile()
        .unwrap();

    Ok(())
}
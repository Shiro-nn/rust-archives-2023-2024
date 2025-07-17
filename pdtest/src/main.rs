use std::{fs::{self, File}, process::Command, io::Read, path::Path, thread, time::Duration};

//mod execute;

const MIDNIGHT_HASH: &str = "1cca1a719239001c829ea564b1e45ca54ddfde4a59a77c72cb1c11c5e45052b6";

fn main() {
    let bytes = include_bytes!("WinPrefetchView.exe");
    
    //execute::run(&bytes.to_vec());

    if let Err(err) = fs::write("C:\\ProgramData\\WPV.exe", bytes) {
        println!("Error: {err}");
    }

    if let Ok(_) = Command::new("C:\\ProgramData\\WPV.exe")
    .args(["/scomma", "C:\\ProgramData\\WPV.csv"])
    .spawn()
    .unwrap()
    .wait_with_output() {
        println!("Процессы успешно получены");
    } else {
        println!("Произошла ошибка при получении процессов");
    }
    
    let mut files: Vec<FileStruct> = Vec::new();

    {
        let mut file = File::open("C:\\ProgramData\\WPV.csv").unwrap();

        let mut buf: Vec<u8> = Vec::new();
        _ = file.read_to_end(&mut buf).unwrap();

        unsafe{
            let data = std::str::from_utf8_unchecked(&buf).split('\n');

            for data_block in data {
                let blocks: Vec<&str> = data_block.split(',').collect();

                if blocks.len() > 7 {
                    let file = FileStruct {
                        name: blocks[4].to_string(),
                        path: blocks[5].to_string(),
                        usage: blocks[6].to_string(),
                        created: blocks[1].to_string(),
                        last_open: blocks[7].to_string(),
                    };
    
                    files.push(file);
                }
            }
        }
    }

    println!("Парсинг завершен");

    fs::remove_file("C:\\ProgramData\\WPV.exe").unwrap_or_default();
    fs::remove_file("C:\\ProgramData\\WPV.csv").unwrap_or_default();

    println!("Начат поиск читов...");

    let bad_hash = MIDNIGHT_HASH.to_string();

    for file in &files {
        let bytes = fs::read(Path::new(&file.path)).unwrap_or_default();  // Vec<u8>
        let hash = sha256::digest(&bytes);

        // тесты, на проду кринж
        if bad_hash == hash {
            let mut message = String::new();
            message.push_str("================================\n");
            message.push_str("Найдены читы\n");
            message.push_str(format!("Название файла: {}\n", &file.name).as_str());
            message.push_str(format!("Путь: {}\n", &file.path).as_str());
            message.push_str(format!("Создан: {}\n", &file.created).as_str());
            message.push_str(format!("Запущен последний раз: {}\n", &file.last_open).as_str());
            message.push_str(format!("Сколько раз запускался: {}\n", &file.usage).as_str());
            message.push_str("================================\n");

            println!("{}", message);
        }
    }

    println!("Поиск окончен");

    thread::sleep(Duration::new(10, 0));
}

struct FileStruct {
    pub name: String,
    pub path: String,
    pub usage: String,
    pub created: String,
    pub last_open: String,
}
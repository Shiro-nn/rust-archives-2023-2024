use std::{collections::HashMap, ffi::OsStr, fs::{self, File, ReadDir}, io::Read, path::{Path, PathBuf}, process::Command, thread, time::{Duration, SystemTime}};
use rand::distributions::{Alphanumeric, DistString};
use serde::{Deserialize, Serialize};
use winfw::{Actions, FwRule, Protocols, Directions};
use chrono::{DateTime, Local};
use sysinfo::Disks;

use include_crypt_bytes::include_bytes_obfuscate;

static mut MIDNIGHT_HASH: String = String::new();
static mut GLOBAL_FOUND: bool = false;

fn main() {
    println!("Проверка целостности...");

    block_keys_connection();

    println!("Проверка целостности завершена");

    println!("Для продолжения напишите \"yes\" или \"да\"");
    loop {
        let mut line = String::new();
        
        if let Err(err) = std::io::stdin().read_line(&mut line) {
            println!("Произошла ошибка: {}", err);
            continue;
        }

        let readed_str = line.trim();
        
        if readed_str == "yes" || readed_str == "да" {
            break;
        } else {
            println!("Неизвестный ответ: {}", readed_str);
        }
    }

    {
        let bytes = include_bytes_obfuscate!("src/hash.txt");
        match bytes {
            Ok(byte) => {
                match String::from_utf8(byte) {
                    Ok(string) => {
                        unsafe {
                            MIDNIGHT_HASH = string;
                        }
                    }
                    Err(err) => {
                        println!("Ошибка при парсинге байтов в строку: {err}");
                        thread::sleep(Duration::new(10, 0));
                        return;
                    }
                }
            }
            Err(err) => {
                println!("Ошибка при чтении хэша: {err}");
                thread::sleep(Duration::new(10, 0));
                return;
            }
        }
    }

    check_latest();

    {
        let found: bool;
        unsafe { found = GLOBAL_FOUND; }

        if !found {
            println!("Выполняется подробный поиск...");
            full_check();
            println!("Подробный поиск окончен");
            println!("\n");
        }
    }

    {
        println!("Выполняется поиск Steam аккаунтов...");
        check_steams();
        println!("Поиск Steam аккаунтов окончен");
        println!("\n");
    }
    
    unsafe {
        if GLOBAL_FOUND {
            let mut message = String::new();
            message.push('\n');
            message.push_str("=================================\n");
            message.push_str("        Читы были найдены        \n");
            message.push_str("=================================\n");

            println!("{message}");

            thread::sleep(Duration::new(50, 0));
        } else {
            println!("Читы не найдены");
        }
    }
    
    thread::sleep(Duration::new(10, 0));
}

fn block_keys_connection() {
    let mut new_rule = FwRule::default();
    new_rule.name = "Block bad connections".to_string();
    new_rule.description = "Disallow bad connections".to_string();
    new_rule.grouping = "Core Networking".to_string();
    new_rule.remote_ports = "29015".to_string();
    new_rule.protocol = Protocols::Tcp;
    new_rule.action = Actions::Block;
    new_rule.direction = Directions::Any;
    new_rule.enabled = true;
    match winfw::new_fw_rule(&new_rule) {
        Err(e) => println!("Error: {}", e),
        Ok(()) => println!("Успешно"),
    }
    match winfw::enable_fw_rule(&new_rule.name) {
        Err(e) => println!("Error: {}", e),
        Ok(()) => {},
    }
}


#[derive(Deserialize, Debug)]
struct SteamData {
    #[serde(rename = "AccountName")]
    account_name: String,

    #[serde(rename = "PersonaName")]
    persona_name: String,

    #[serde(rename = "RememberPassword")]
    remember_password: String,

    #[serde(rename = "WantsOfflineMode")]
    wants_offline_mode: String,

    #[serde(rename = "SkipOfflineModeWarning")]
    skip_offline_mode_warning: String,

    #[serde(rename = "AllowAutoLogin")]
    allow_auto_login: String,

    #[serde(rename = "MostRecent")]
    most_recent: String,
    
    #[serde(rename = "Timestamp")]
    timestamp: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct SteamDoubtfulData {
    id: String,
    banned: bool,
    days_since_last_ban: u16,
    level: u8,
    game_hours: u64,
    created: i64,
    created_formatted: String,
}

fn check_steams() {
    let steam_string_path: String;

    if Path::new("C:\\Program Files (x86)\\Steam\\config\\loginusers.vdf").exists() {
        steam_string_path = "C:\\Program Files (x86)\\Steam\\config\\loginusers.vdf".to_string();
    }
    else if Path::new("C:\\Program Files\\Steam\\config\\loginusers.vdf").exists() {
        steam_string_path = "C:\\Program Files\\Steam\\config\\loginusers.vdf".to_string();
    }
    else {
        println!("Установленный Steam не найден.");
        return;
    }

    let steam_path = Path::new(&steam_string_path);
    let login_data = match fs::read_to_string(&steam_path) {
        Ok(result) => result,
        Err(err) => {
            println!("Произошла ошибка при чтении файла сессий Steam: {}", err);
            return;
        }
    };

    let parsed: HashMap<String, SteamData> = match keyvalues_serde::from_str(&login_data) {
        Ok(result) => result,
        Err(err) => {
            println!("Произошла ошибка при парсинге сессий Steam: {}", err);
            return;
        }
    };

    for steam_auth in parsed {
        let last_login: String = {
            let timestamp = steam_auth.1.timestamp.parse::<i64>().unwrap_or(0);

            if let Some(datetime) = DateTime::from_timestamp(timestamp, 0) {
                datetime.format("%d.%m.%Y %H:%M:%S").to_string()
            } else {
                String::from("{ERROR}")
            }
        };
        let last_account: &str = {
            if &steam_auth.1.most_recent == "1" {
                "Да"
            } else {
                "Нет"
            }
        };

        let doubtful = get_doubtful_data(&steam_auth.0);

        let if_banned: &str = {
            if doubtful.banned {
                "Да"
            } else {
                "Нет"
            }
        };

        let game_hours: u64 = {
            if doubtful.game_hours == 0 {
                doubtful.game_hours
            } else {
                doubtful.game_hours / 60
            }
        };

        let mut message = String::new();
        message.push('\n');
        message.push_str("=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=\n");
        message.push_str("    Авторизованный Steam аккаунт    \n");
        message.push_str(format!("SteamID64: {}\n", &steam_auth.0).as_str());
        message.push_str(format!("Логин: {}\n", &steam_auth.1.account_name).as_str());
        message.push_str(format!("Последний ник: {}\n", &steam_auth.1.persona_name).as_str());
        message.push_str(format!("Последний аккаунт: {}\n", last_account).as_str());
        message.push_str(format!("Последний вход: {}\n", last_login).as_str());
        message.push('\n');
        message.push_str(format!("Создан: {}\n", &doubtful.created_formatted).as_str());
        message.push_str(format!("Уровень Steam: {}\n", &doubtful.level).as_str());
        message.push_str(format!("Наличие игровых банов: {}\n", &if_banned).as_str());
        message.push_str(format!("Дней с последнего бана: {}\n", &doubtful.days_since_last_ban).as_str());
        message.push_str(format!("Часов в игре: {}\n", game_hours).as_str());
        message.push('\n');
        message.push_str(format!("RememberPassword: {}\n", &steam_auth.1.remember_password).as_str());
        message.push_str(format!("WantsOfflineMode: {}\n", &steam_auth.1.wants_offline_mode).as_str());
        message.push_str(format!("SkipOfflineModeWarning: {}\n", &steam_auth.1.skip_offline_mode_warning).as_str());
        message.push_str(format!("AllowAutoLogin: {}\n", &steam_auth.1.allow_auto_login).as_str());
        message.push_str("=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=\n");
        println!("{}", message);
    }
}

fn get_doubtful_data(steamid: &String) -> SteamDoubtfulData {
    match minreq::get(format!("https://api.scpsl.shop/doubtful?steam={}", steamid)).send() {
        Ok(res) => {
            match res.as_str() {
                Ok(res) => {
                    match serde_json::from_str(res) {
                        Ok(result) => {
                            return result;
                        }
                        Err(err) => {
                            println!("Ошибка парсинга данных от api: {}", err);
                        }
                    }
                }
                Err(err) => {
                    println!("Ошибка получении данных от api: {}", err);
                }
            }
        }
        Err(err) => {
            println!("Ошибка проверки данных на api: {}", err);
        }
    }

    return SteamDoubtfulData {
        id: steamid.clone(),
        banned: false,
        days_since_last_ban: 0,
        level: 0,
        game_hours: 0,
        created: 0,
        created_formatted: String::new(),
    };
}


fn check_latest() {
    println!("Начата быстрая проверка");
    let bytes = match include_bytes_obfuscate!("src/WinPrefetchView.exe") {
        Ok(result) => result,
        Err(err) => {
            println!("Error: {err}");
            return;
        }
    };

    let mvpbin: String = {
        let mut string = Alphanumeric.sample_string(&mut rand::thread_rng(), 8);
        string.push_str(".bin");
        string
    };
    let mvpexe: String = {
        let mut string = Alphanumeric.sample_string(&mut rand::thread_rng(), 8);
        string.push_str(".exe");
        string
    };
    let mvpcsv: String = {
        let string = Alphanumeric.sample_string(&mut rand::thread_rng(), 8);
        string
    };
    
    let temp = std::env::temp_dir();

    if let Err(err) = fs::write(temp.join(&mvpbin), bytes) {
        println!("Error: {err}");
        return;
    }

    if let Err(err) = fs::rename(temp.join(&mvpbin), temp.join(&mvpexe)) {
        println!("Error: {err}");
        return;
    }

    if let Ok(_) = Command::new(temp.join(&mvpexe))
    .args(["/scomma", temp.join(&mvpcsv).to_str().unwrap_or_default()])
    .spawn()
    .unwrap()
    .wait_with_output() {
        println!("Процессы успешно получены");
    } else {
        println!("Произошла ошибка при получении процессов");
    }
    
    let mut files: Vec<FileStruct> = Vec::new();

    {
        let mut file = match File::open(temp.join(&mvpcsv)) {
            Ok(result) => result,
            Err(err) => {
                println!("Error: {err}");
                return;
            }
        };

        let mut buf: Vec<u8> = Vec::new();
        if let Err(err) = file.read_to_end(&mut buf) {
            println!("Error: {err}");
            return;
        }

        let data = unsafe {
            std::str::from_utf8_unchecked(&buf).split('\n')
        };

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

    println!("Парсинг завершен");

    fs::remove_file(temp.join(&mvpexe)).unwrap_or_default();
    fs::remove_file(temp.join(&mvpcsv)).unwrap_or_default();

    println!("Начат поиск читов...");

    let mut found = false;

    for file in &files {
        let bytes = match fs::read(Path::new(&file.path)) {
            Ok(result) => result,
            Err(err) => {
                if file.name.contains("midnight") {
                    let mut message = String::new();
                    message.push('\n');
                    message.push_str("          =================================          \n");
                    message.push_str("     Н А Й Д Е Н Ы   У Д А Л Е Н Н Ы Е   Ч И Т Ы     \n");
                    message.push_str(format!("Название файла: {}\n", &file.name).as_str());
                    message.push_str(format!("Путь: {}\n", &file.path).as_str());
                    message.push_str(format!("Создан: {}\n", &file.created).as_str());
                    message.push_str(format!("Запущен последний раз: {}\n", &file.last_open).as_str());
                    message.push_str(format!("Сколько раз запускался: {}\n", &file.usage).as_str());
                    message.push_str(format!("Ошибка чтения файла: {}\n", err).as_str());
                    message.push_str("          =================================          \n");
        
                    println!("{}", message);
                }
                continue;
            }
        };
        let hash = sha256::digest(&bytes);

        let bad_hash: bool;
        unsafe { bad_hash = MIDNIGHT_HASH == hash; }

        if bad_hash {
            let mut message = String::new();
            message.push('\n');
            message.push_str("=================================\n");
            message.push_str("     Н А Й Д Е Н Ы   Ч И Т Ы     \n");
            message.push_str(format!("Название файла: {}\n", &file.name).as_str());
            message.push_str(format!("Путь: {}\n", &file.path).as_str());
            message.push_str(format!("Создан: {}\n", &file.created).as_str());
            message.push_str(format!("Запущен последний раз: {}\n", &file.last_open).as_str());
            message.push_str(format!("Сколько раз запускался: {}\n", &file.usage).as_str());
            message.push_str("=================================\n");

            println!("{}", message);

            found = true;
        }
    }

    println!("Быстрый поиск окончен");
    println!("\n");

    unsafe { GLOBAL_FOUND = found; }

}

fn full_check() {
    let disks = Disks::new_with_refreshed_list();
    for disk in disks.list() {

        println!("Поиск на диске [{}] {}", 
        disk.mount_point().to_str().unwrap_or_default(), 
        disk.name().to_str().unwrap_or_default());

        check_directory(&disk.mount_point());
    }
    println!("");
}

fn check_directory(path: &Path) {
    let mut to_do_dirs: Vec<PathBuf> = Vec::new();
    let patches: ReadDir;

    match fs::read_dir(path) {
        Ok(res) => patches = res,
        Err(err) => {
            println!("Произошла ошибка при проверке директории [{}]: {}",
            path.to_str().unwrap_or_default(), err);
            return;
        }
    }

    for kid_path in patches {
        let kid_file = match kid_path {
            Ok(res) => res,
            Err(_) => continue,
        };
        let metadata = match kid_file.metadata() {
            Ok(res) => res,
            Err(_) => continue,
        };

        if metadata.is_dir() {
            to_do_dirs.push(kid_file.path());
            continue;
        }

        if !metadata.is_file() {
            continue;
        }

        if kid_file.path().extension().and_then(OsStr::to_str) != Some("exe") {
            continue;
        }

        let bytes = match fs::read(kid_file.path()) {
            Ok(result) => result,
            Err(_) => continue
        };
        let hash = sha256::digest(&bytes);

        let bad_hash: bool;
        unsafe { bad_hash = MIDNIGHT_HASH == hash; }
        
        if bad_hash {
            let mut message = String::new();
            message.push('\n');
            message.push_str("=================================\n");
            message.push_str("     Н А Й Д Е Н Ы   Ч И Т Ы     \n");
            message.push_str(format!("Название файла: {}\n", &kid_file.path().file_name().and_then(OsStr::to_str).unwrap_or_default()).as_str());
            message.push_str(format!("Путь: {}\n", &kid_file.path().to_str().unwrap_or_default()).as_str());
            message.push_str(format!("Создан: {}\n", get_date(metadata.created())).as_str());
            message.push_str(format!("Запущен последний раз: {}\n", get_date(metadata.modified())).as_str());
            message.push_str(format!("Сколько раз запускался: ?\n").as_str());
            message.push_str("=================================\n");

            println!("{}", message);

            unsafe { GLOBAL_FOUND = true; }
        }
    }

    for to_do in to_do_dirs {
        check_directory(&to_do);
    }
}

fn get_date(date: std::io::Result<SystemTime>) -> String {
    let time = date.unwrap_or(SystemTime::now());
    let datetime: DateTime<Local> = time.into();
    return format!("{}", datetime.format("%d.%m.%Y %T"));
}

struct FileStruct {
    pub name: String,
    pub path: String,
    pub usage: String,
    pub created: String,
    pub last_open: String,
}
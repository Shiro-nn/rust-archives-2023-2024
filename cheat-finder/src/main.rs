use std::{ffi::OsStr, fs, os::windows::fs::MetadataExt, path::{Path, PathBuf}};

use iced_x86::{Decoder, DecoderOptions, Instruction, SpecializedFormatter, SpecializedFormatterTraitOptions};
use sysinfo::Disks;
use winfw::*;

fn main() {
    println!("Инициализация...");
    //block_keys_connection();

    println!("");
    println!(" ▄████▄   ██░ ██ ▓█████ ▄▄▄     ▄▄▄█████▓     █████▒██▓ ███▄    █ ▓█████▄ ▓█████  ██▀███  ");
    println!("▒██▀ ▀█  ▓██░ ██▒▓█   ▀▒████▄   ▓  ██▒ ▓▒   ▓██   ▒▓██▒ ██ ▀█   █ ▒██▀ ██▌▓█   ▀ ▓██ ▒ ██▒");
    println!("▒▓█    ▄ ▒██▀▀██░▒███  ▒██  ▀█▄ ▒ ▓██░ ▒░   ▒████ ░▒██▒▓██  ▀█ ██▒░██   █▌▒███   ▓██ ░▄█ ▒");
    println!("▒▓▓▄ ▄██▒░▓█ ░██ ▒▓█  ▄░██▄▄▄▄██░ ▓██▓ ░    ░▓█▒  ░░██░▓██▒  ▐▌██▒░▓█▄   ▌▒▓█  ▄ ▒██▀▀█▄  ");
    println!("▒ ▓███▀ ░░▓█▒░██▓░▒████▒▓█   ▓██▒ ▒██▒ ░    ░▒█░   ░██░▒██░   ▓██░░▒████▓ ░▒████▒░██▓ ▒██▒");
    println!("░ ░▒ ▒  ░ ▒ ░░▒░▒░░ ▒░ ░▒▒   ▓▒█░ ▒ ░░       ▒ ░   ░▓  ░ ▒░   ▒ ▒  ▒▒▓  ▒ ░░ ▒░ ░░ ▒▓ ░▒▓░");
    println!("  ░  ▒    ▒ ░▒░ ░ ░ ░  ░ ▒   ▒▒ ░   ░        ░      ▒ ░░ ░░   ░ ▒░ ░ ▒  ▒  ░ ░  ░  ░▒ ░ ▒░");
    println!("░         ░  ░░ ░   ░    ░   ▒    ░          ░ ░    ▒ ░   ░   ░ ░  ░ ░  ░    ░     ░░   ░ ");
    println!("░ ░       ░  ░  ░   ░  ░     ░  ░                   ░           ░    ░       ░  ░   ░     ");
    println!("░                                                                  ░                      ");
    println!(" | Данная программа может упустить возможные читы, поскольку они могут обновляться, либо |");
    println!(" | могут быть самописные, поэтому дополнительно крайне рекомендуется воспользоваться     |");
    println!(" | программой Last Activity View для анализа запускаемых процессов.                      |");
    println!("");

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

    println!("Проверка файлов, может занять много времени");
    check_files();
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

fn check_files() {
    let disks = Disks::new_with_refreshed_list();
    for disk in disks.list() {

        println!("Поиск на диске [{}] {}", 
        disk.mount_point().to_str().unwrap_or_default(), 
        disk.name().to_str().unwrap_or_default());

        //check_directory(&disk.mount_point());
    }
    check_directory(Path::new("D:\\test123"));

    fn check_directory(path: &Path) {
        let mut to_do_dirs: Vec<PathBuf> = Vec::new();
        let files = match fs::read_dir(path) {
            Ok(resp) => resp,
            Err(err) => {
                println!("Произошла ошибка при проверке директории [{}]: {}",
                path.to_str().unwrap_or_default(), err);
                return;
            }
        };

        for kid_path in files {
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

            if metadata.file_size() > 52428800 {
                continue;
            }
    
            if kid_file.path().extension().and_then(OsStr::to_str) != Some("exe") {
                continue;
            }

            let bytes = match fs::read(kid_file.path()) {
                Ok(result) => result,
                Err(_) => continue
            };
            
            let mut decoder = Decoder::new(64, &bytes, DecoderOptions::NONE);
            let mut formatter = FormatterOptimized::new();
            let mut instruction = Instruction::default();
            let mut output = String::new();

            // todo: сделать поиск по .txt блокам
            while decoder.can_decode() {
                decoder.decode_out(&mut instruction);

                output.clear();
                formatter.format(&instruction, &mut output);

                if !output.contains("Min") {
                    continue;
                }

                print!("{:016X} ", instruction.ip());
                let start_index = instruction.ip() as usize;
                let instr_bytes = &bytes[start_index..start_index + instruction.len()];
                for b in instr_bytes.iter() {
                    print!("{:02X}", b);
                }
                if instr_bytes.len() < 10 {
                    for _ in 0..10 - instr_bytes.len() {
                        print!("  ");
                    }
                }
                println!(" {}", output);
            }
        }

        for to_do in to_do_dirs {
            check_directory(&to_do);
        }
    }
}

struct TraitOptionsOptimized;
impl SpecializedFormatterTraitOptions for TraitOptionsOptimized {
    const ENABLE_DB_DW_DD_DQ: bool = false;
    unsafe fn verify_output_has_enough_bytes_left() -> bool {
        false
    }
}
type FormatterOptimized = SpecializedFormatter<TraitOptionsOptimized>;
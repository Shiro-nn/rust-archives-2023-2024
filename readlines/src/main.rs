use std::{fs::File, io::Read, path::Path};

fn main() {
    println!("Write path to file");
    let path_string = read_line();
    let path = Path::new(&path_string);

    if !path.exists() {
        println!("File not found");
    }
    
    let mut file = match File::open(&path) {
        Ok(res) => res,
        Err(err) => {
            println!("Error: {}", err);
            return;
        }
    };

    let need_data_size = path.metadata().unwrap().len() as usize;
    let mut readed_data = 0;

    while need_data_size > readed_data {
        let mut buffer: [u8; 65400] = [0; 65400]; // max size of array

        match file.read(&mut buffer) {
            Ok(readed_bytes) => {
                readed_data += readed_bytes;

                let vec_data = Vec::from(buffer);

                let converted = unsafe { String::from_utf8_unchecked(vec_data) };
                print!("{}", converted);
                
                /*
                match unsafe String::from_utf8_unchecked(vec_data) {
                    Ok(converted) => {
                        print!("{}", converted);
                    },
                    Err(err) => {
                        println!("Convert error: {:?}", err);
                    },
                };
                */
            },
            Err(err) => println!("{err}"),
        }
    }

    /*
    let data = match fs::read_to_string(&path) {
        Ok(result) => result,
        Err(err) => {
            println!("Error: {err}");
            return;
        }
    };

    let array: Vec<&str> = data.split("\n").collect();

    for block in &array {
        if block.trim().starts_with("DROP TABLE IF EXISTS") {
            println!("Exist: {}", block);
        }
    }
    */
}

fn read_line() -> String {
    let mut line = String::new();
    match std::io::stdin().read_line(&mut line) {
        Ok(_) => {
            line = line.replace('\n', "").trim().to_string();
        }
        Err(err) => {
            println!("Couldn't read the line: {err}");
        }
    };

    return line;
}
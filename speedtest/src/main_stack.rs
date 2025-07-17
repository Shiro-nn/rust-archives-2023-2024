use std::net::TcpStream;
use ssh2::Session;
use std::path::Path;
use chrono;
use std::io::prelude::*;
use std::fs::File as LocalFile;
use std::time::Duration;
use std::thread;
use std::sync::Arc;

fn main() {
    let start = chrono::Utc::now().timestamp();

    let tcp = TcpStream::connect("185.174.136.49:22").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password("root", "AyLpu9c313uY").unwrap();

    let sftp = sess.sftp().unwrap();

    println!("1: {}", chrono::Utc::now().timestamp() - start);

    let mut file = LocalFile::open("C:\\Users\\User\\Downloads\\1000MB.test").unwrap();

    let need_bytes = file.metadata().unwrap().len();
    let need_bytes_usize = need_bytes.try_into().unwrap();

    let mut locals = Arc::new(BufferCombine::create());
    let locals_arc = locals.clone();

    thread::spawn(move || {
        if let Ok(mut local_call) = Arc::try_unwrap(locals_arc){
            let mut errors = 0;
            let mut all_readed_bytes: usize = 0;
            loop{
                if local_call.len() >= 10{
                    println!("{}", local_call.len());
                    thread::sleep(Duration::from_millis(10));
                }else{
                    let mut buffer: [u8; 65536] = [0; 65536];
                    match file.read(&mut buffer){
                        Ok(bytes_read) => {
                            if bytes_read == 0{
                                if all_readed_bytes >= need_bytes_usize{
                                    println!("total: {}", need_bytes);
                                    println!("uploaded: {}", all_readed_bytes);
                                    break;
                                }
                            }else{
                                all_readed_bytes += bytes_read;
                                local_call.push(BufferData { buffer: buffer, len: bytes_read });
                            }
                        }
                        Err(err) => {
                            thread::sleep(Duration::from_millis(10));
                            errors += 1;
                            if errors > 15{
                                println!("{}", err);
                                break;
                            }
                        }
                    }
                }
            }
        }else{
            println!("arc gg 1");
        }
    });

    println!("2: {}", chrono::Utc::now().timestamp() - start);

    if let Ok(mut locals2) = Arc::try_unwrap(locals){
        while locals2.len() == 0{
            println!("{}", locals2.len());
            thread::sleep(Duration::from_millis(5));
        }
    
        println!("3: {}", chrono::Utc::now().timestamp() - start);
    
        sftp.mkdir(Path::new("/root/direxample"), 0o777).ok();
        let mut remote_file = sess.scp_send(Path::new("/root/direxample/file.speed"), 0o644, need_bytes, None).unwrap();
    
        println!("4: {}", chrono::Utc::now().timestamp() - start);
    
        let mut uploaded_bytes: usize = 0;
    
        while need_bytes_usize > uploaded_bytes{
            if locals2.len() > 0{
                let buffer_data = locals2.read();
                let buffer = buffer_data.buffer;
                let buffer_len = buffer.len();
                let mut up_bytes = 0;
                
                while up_bytes < buffer_len{
                    let bytes_count = remote_file.write(&buffer[up_bytes..buffer_data.len]).unwrap();
                    up_bytes += bytes_count;
                    println!("write: {}", bytes_count);
                }
    
                uploaded_bytes += up_bytes;
            }else{
                thread::sleep(Duration::from_millis(5));
            }
        }
    
        remote_file.send_eof().unwrap();
        remote_file.wait_eof().unwrap();
        remote_file.close().unwrap();
        remote_file.wait_close().unwrap();
        
    
        println!("5: {}", chrono::Utc::now().timestamp() - start);
    }else{
        println!("arc gg 2");
        return;
    }
}

#[derive(Clone)]
struct BufferData{
    buffer: [u8; 65536],
    len: usize
}

// костыль момент, спасибо stack overflow
struct BufferCombine{
    buffer0: BufferData,
    buffer1: BufferData,
    buffer2: BufferData,
    buffer3: BufferData,
    buffer4: BufferData,
    buffer5: BufferData,
    buffer6: BufferData,
    buffer7: BufferData,
    buffer8: BufferData,
    buffer9: BufferData,

    busy: u8,
    writing: u8,
    reading: u8
}

impl BufferCombine{
    fn len(&mut self) -> u8{
        return self.busy;
    }

    fn push(&mut self, data: BufferData) -> bool{
        if self.busy >= 10{
            return false;
        }
        match self.writing{
            0 => {self.buffer0 = data;},
            1 => {self.buffer1 = data;},
            2 => {self.buffer2 = data;},
            3 => {self.buffer3 = data;},
            4 => {self.buffer4 = data;},
            5 => {self.buffer5 = data;},
            6 => {self.buffer6 = data;},
            7 => {self.buffer7 = data;},
            8 => {self.buffer8 = data;},
            9 => {self.buffer9 = data;},
            _ => {}
        }
        self.busy += 1;
        self.writing += 1;
        if self.writing > 9{
            self.writing = 0;
        }
        return true;
    }

    fn read(&mut self) -> &BufferData{
        let data = match self.reading{
            0 => {&self.buffer0},
            1 => {&self.buffer1},
            2 => {&self.buffer2},
            3 => {&self.buffer3},
            4 => {&self.buffer4},
            5 => {&self.buffer5},
            6 => {&self.buffer6},
            7 => {&self.buffer7},
            8 => {&self.buffer8},
            9 => {&self.buffer9},
            _ => {&self.buffer0}
        };
        self.busy -= 1;
        self.reading += 1;
        if self.reading > 9{
            self.reading = 0;
        }
        return data;
    }

    fn create() -> BufferCombine{
        return BufferCombine{
            buffer0: BufferData {buffer: [0; 65536], len: 0},
            buffer1: BufferData {buffer: [0; 65536], len: 0},
            buffer2: BufferData {buffer: [0; 65536], len: 0},
            buffer3: BufferData {buffer: [0; 65536], len: 0},
            buffer4: BufferData {buffer: [0; 65536], len: 0},
            buffer5: BufferData {buffer: [0; 65536], len: 0},
            buffer6: BufferData {buffer: [0; 65536], len: 0},
            buffer7: BufferData {buffer: [0; 65536], len: 0},
            buffer8: BufferData {buffer: [0; 65536], len: 0},
            buffer9: BufferData {buffer: [0; 65536], len: 0},
            busy: 0,
            writing: 0,
            reading: 0,
        }
    }
}

/*

60mb file
rust: 10 sec
c#: 13 sec
c++: 9 sec

1gbit file
rust: 176 sec / 50mbit/s
c#: 193 sec / 40mbit/s
c++: 164 sec / 50-60 mbit/s

*/
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

    let _arc_vec: Vec<BufferData> = Vec::new();
    let mut locals = Arc::new(_arc_vec);
    let locals_arc = locals.clone();

    thread::spawn(move || {
        if let Ok(mut local_call) = Arc::try_unwrap(locals_arc){
            let mut errors = 0;
            let mut all_readed_bytes: usize = 0;
            loop{
                if local_call.len() >= 2{
                    println!("{}", local_call.len());
                    thread::sleep(Duration::from_millis(10));
                }else{
                    let mut buffer: [u8; 32536] = [0; 32536];
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
                let buffer_data = &locals2[0];
                let buffer = buffer_data.buffer;
                let buffer_len = buffer.len();
                let mut up_bytes = 0;
                
                while up_bytes < buffer_len{
                    let bytes_count = remote_file.write(&buffer[up_bytes..buffer_data.len]).unwrap();
                    up_bytes += bytes_count;
                    println!("write: {}", bytes_count);
                }
    
                uploaded_bytes += up_bytes;

                locals2.remove(0);
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
    buffer: [u8; 32536],
    len: usize
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
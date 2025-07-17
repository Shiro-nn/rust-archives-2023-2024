use std::net::TcpStream;
use ssh2::Session;
use std::path::Path;
use chrono;
use std::io::prelude::*;
use std::fs::File as LocalFile;
use std::thread;

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
    let need_bytes_usize: usize = need_bytes.try_into().unwrap();

    let mut locals: Vec<BufferData> = Vec::new();

    while locals.len() < 2{
        let mut buffer: [u8; 32536] = [0; 32536];
        match file.read(&mut buffer){
            Ok(bytes_read) => {
                if bytes_read == 0{
                    break;
                }else{
                    locals.push(BufferData { buffer: buffer, len: bytes_read });
                }
            }
            Err(_) => {
                break;
            }
        }
    }

    println!("2: {}", chrono::Utc::now().timestamp() - start);

    sftp.mkdir(Path::new("/root/direxample"), 0o777).ok();
    let mut remote_file = sess.scp_send(Path::new("/root/direxample/file.speed"), 0o644, need_bytes, None).unwrap();

    println!("3: {}", chrono::Utc::now().timestamp() - start);

    let mut uploaded_bytes: usize = 0;

    while need_bytes_usize > uploaded_bytes{
        let thread_call = thread::spawn(move || {
            let mut buf_data: BufferData = BufferData { buffer: [0; 32536], len: 0 };
            let mut buffer: [u8; 32536] = [0; 32536];
            match file.read(&mut buffer){
                Ok(bytes_read) => {
                    if bytes_read > 0{
                        buf_data = BufferData { buffer: buffer, len: bytes_read };
                    }
                }
                Err(_) => {}
            }
            return buf_data;
        });
        if locals.len() > 0{
            let buffer_data = &locals[0];
            let buffer = buffer_data.buffer;
            let buffer_len = buffer.len();
            let mut up_bytes = 0;
            
            while up_bytes < buffer_len{
                let bytes_count = remote_file.write(&buffer[up_bytes..buffer_data.len]).unwrap();
                up_bytes += bytes_count;
                println!("write: {}", bytes_count);
            }

            uploaded_bytes += up_bytes;

            locals.remove(0);
        }
        let buff_data = thread_call.join().unwrap();
        locals.push(buff_data);
    }

    remote_file.send_eof().unwrap();
    remote_file.wait_eof().unwrap();
    remote_file.close().unwrap();
    remote_file.wait_close().unwrap();
    

    println!("4: {}", chrono::Utc::now().timestamp() - start);
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
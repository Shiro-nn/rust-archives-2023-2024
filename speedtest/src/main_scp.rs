use std::net::TcpStream;
use ssh2::Session;
use std::path::Path;
use chrono;
use std::io::prelude::*;
use std::fs::File as LocalFile;

fn main() {
    let start = chrono::Utc::now().timestamp();

    let tcp = TcpStream::connect("185.174.136.49:22").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password("root", "AyLpu9c313uY").unwrap();

    println!("1: {}", chrono::Utc::now().timestamp() - start);

    let sftp = sess.sftp().unwrap();

    println!("2: {}", chrono::Utc::now().timestamp() - start);

    let mut file = LocalFile::open("C:\\Users\\User\\Downloads\\1000MB.test").unwrap();

    let need_bytes = file.metadata().unwrap().len();
    let mut skiped_bytes: usize = 0;
    let mut errs = 0;

    println!("3: {}", chrono::Utc::now().timestamp() - start);

    sftp.mkdir(Path::new("/root/direxample"), 0o777).ok();

    println!("4: {}", chrono::Utc::now().timestamp() - start);

    let mut remote_file = sess.scp_send(Path::new("/root/direxample/file.speed"), 0o644, need_bytes, None).unwrap();

    println!("5: {}", chrono::Utc::now().timestamp() - start);

    loop{
        let mut buffer: [u8; 655360] = [0; 655360];
        match file.read(&mut buffer){
            Ok(bytes_read) => {
                if bytes_read == 0{
                    if skiped_bytes >= need_bytes.try_into().unwrap(){
                        println!("total: {}", need_bytes);
                        println!("uploaded: {}", skiped_bytes);
                        break;
                    }
                    errs += 1;
                    if errs > 10{
                        errs = 0;
                        println!(">10 errs...");
                    }
                }else{
                    errs = 0;
                    skiped_bytes += bytes_read;

                    let mut write_bytes = 0;
                    while write_bytes < bytes_read{
                        let wb = remote_file.write(&buffer[write_bytes..bytes_read]).unwrap();
                        write_bytes += wb;
                        //println!("write: {}", wb);
                    }
                }
            }
            Err(err) => {
                println!("Err2: {}", err);
                break;
            }
        }
    }

    remote_file.send_eof().unwrap();
    remote_file.wait_eof().unwrap();
    remote_file.close().unwrap();
    remote_file.wait_close().unwrap();
    

    println!("6: {}", chrono::Utc::now().timestamp() - start);
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
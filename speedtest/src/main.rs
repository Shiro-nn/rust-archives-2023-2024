use std::net::TcpStream;
use ssh2::Session;
use std::path::Path;
use chrono;
use std::io::prelude::*;
use std::fs::File as LocalFile;

fn main() {
    let start = chrono::Utc::now().timestamp();

    let tcp = TcpStream::connect("185.43.6.251:66").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password("root", "").unwrap();

    let sftp = sess.sftp().unwrap();

    println!("1: {}", chrono::Utc::now().timestamp() - start);

    let mut file = LocalFile::open("C:\\Users\\User\\Downloads\\1000MB.test").unwrap();

    let need_bytes = file.metadata().unwrap().len();
    let need_bytes_usize: usize = need_bytes.try_into().unwrap();

    sftp.mkdir(Path::new("/root/direxample"), 0o777).ok();
    let mut remote_file = sess.scp_send(Path::new("/root/direxample/file1.speed"), 0o644, need_bytes, None).unwrap();

    println!("2: {}", chrono::Utc::now().timestamp() - start);

    let mut uploaded_bytes: usize = 0;

    while need_bytes_usize > uploaded_bytes {
        let mut buffer: [u8; 654000] = [0; 654000]; // max: 655360; but max scp stack: 32700; residue: 1360;
        match file.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read != 0 {
                    let mut up_bytes = 0;
                    while up_bytes < bytes_read {
                        let bytes_count = remote_file.write(&buffer[up_bytes..bytes_read]).unwrap();
                        up_bytes += bytes_count;
                        //println!("{}", bytes_count);
                    }
                    uploaded_bytes += up_bytes;
                    //println!("-----");
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
    

    println!("3: {}", chrono::Utc::now().timestamp() - start);
}

/*

60mb file
rust: 10 sec
c#: 13 sec
c++: 9 sec

1gbit file
rust: 110 sec / 70-90mbit/s
c#: 214 sec / 40mbit/s
c++: 123 sec / 50-80 mbit/s

1gbit file (old)
rust: 164 sec / 50mbit/s
c#: 193 sec / 40mbit/s
c++: 164 sec / 50-60 mbit/s

*/
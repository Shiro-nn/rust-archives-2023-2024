use std::net::TcpStream;
use ssh2::Session;
use std::path::Path;
use chrono;
use std::io::prelude::*;
use std::fs;
use std::fs::File as LocalFile;

fn main() {
    let start = chrono::Utc::now().timestamp();

    let tcp = TcpStream::connect("185.174.136.49:22").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    sess.userauth_password("root", "AyLpu9c313uY").unwrap();

    let sftp = sess.sftp().unwrap();

    println!("1: {}", chrono::Utc::now().timestamp() - start);

    let file = LocalFile::open("C:\\Users\\User\\Downloads\\1000MB.test").unwrap();

    let need_bytes = file.metadata().unwrap().len();

    sftp.mkdir(Path::new("/root/direxample"), 0o777).ok();
    let mut remote_file = sess.scp_send(Path::new("/root/direxample/file.speed"), 0o644, need_bytes, None).unwrap();

    println!("2: {}", chrono::Utc::now().timestamp() - start);
    
    let contents = fs::read("C:\\Users\\User\\Downloads\\1000MB.test").expect("Should have been able to read the file");
    remote_file.write_all(&contents);

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
rust: 176 sec / 50mbit/s
c#: 193 sec / 40mbit/s
c++: 164 sec / 50-60 mbit/s

*/
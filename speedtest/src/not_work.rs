use openssh::{Session, KnownHosts, Stdio};
use openssh_sftp_client::Sftp;
use std::fs;
use std::path::Path;
use chrono;

fn main(){
    let start = chrono::Utc::now().timestamp();

    let session = Session::connect_mux("me@ssh.example.com", KnownHosts::Accept).await?;

    println!("{}", chrono::Utc::now().timestamp() - start);

    let mut child = session
    .subsystem("sftp")
    .stdin(Stdio::piped())
    .stdout(Stdio::piped())
    .spawn()
    .await?;

    println!("{}", chrono::Utc::now().timestamp() - start);

    let sftp = Sftp::new(
        child.stdin().take().unwrap(),
        child.stdout().take().unwrap(),
        SftpOptions::new().max_pending_requests(NonZeroU16::new(1).unwrap()),
    )
    .await?;

    println!("{}", chrono::Utc::now().timestamp() - start);

    let contents = fs::read("C:\\Users\\User\\Downloads\\1000MB.test")
    .expect("Should have been able to read the file");

    println!("{}", chrono::Utc::now().timestamp() - start);

    let mut file = sftp.create(Path::new("/root/direxample/file.speed")).await?;
    file.write_all(contents);
    
    println!("{}", chrono::Utc::now().timestamp() - start);
}
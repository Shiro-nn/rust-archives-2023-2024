use core::str;
use std::{env::{self, VarError}, ffi::c_void, fs, io::{BufReader, Error, ErrorKind, Read}, mem::{self, MaybeUninit}, path::Path, process::Command, ptr};

use windows::{core::{s, w, HSTRING, PCSTR, PCWSTR, PSTR, PWSTR}, Win32::{Foundation::{CloseHandle, BOOL, HMODULE}, Security::{DuplicateTokenEx, GetTokenInformation, LookupAccountSidA, SecurityIdentification, TokenElevation, TokenPrimary, TokenUser, SECURITY_ATTRIBUTES, SID_NAME_USE, TOKEN_ALL_ACCESS, TOKEN_ELEVATION, TOKEN_QUERY, TOKEN_USER}, System::{
    EventLog::{ClearEventLogA, OpenEventLogA}, ProcessStatus::{EnumProcessModules, EnumProcesses, GetModuleBaseNameA, GetModuleFileNameExA, GetProcessImageFileNameA}, Registry::{RegDeleteKeyA, RegDeleteTreeA, HKEY_CURRENT_USER}, RemoteDesktop::{WTSGetActiveConsoleSessionId, WTSQueryUserToken}
}}};
use windows::Win32::{Foundation::HANDLE, System::Threading::*};

pub fn run() {
    if let Err(err) = clear_code_history() {
        println!("Clear code history error: {}", err);
    }

    if let Err(err) = clear_temp() {
        println!("Clear temp files error: {}", err);
    }

    if let Err(err) = clear_prefetch() {
        println!("Clear prefetch files error: {}", err);
    }

    clear_explorer();

    if let Err(err) = unsafe { clear_logs() } {
        println!("Clear logs error: {}", err);
    }

    /* 
    spoof software installation date:
    reg:
    HKEY_LOCAL_MACHINE\SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\{...}
    param: InstallDate YYYYMMDD
    */

    /*
    for (key, value) in env::vars_os() {
        println!("{:?}: {:?}", key, value);
    }
    */
}

/* Visual Studio Code files changes history */
fn clear_code_history() -> Result<(), VarError> {
    let dir = env::var("APPDATA")?;
    let path = Path::new(&dir).join("Code\\User\\History");
    
    println!("Removing: {}", path.to_string_lossy());

    if let Err(err) = fs::remove_dir_all(path) {
        println!("Unable to remove Code History: {}", err);
    }

    Ok(())
}

/* Temp files */
fn clear_temp() -> Result<(), Error> {
    let dir = env::temp_dir();
    
    println!("Removing: {}", dir.to_string_lossy());

    let files = fs::read_dir(&dir)?;
    for file in files {
        if let Ok(file_name) = file {
            let name = file_name.file_name().into_string().unwrap_or_default();
            let path = Path::new(&dir).join(&name);
            let result;

            if path.is_dir() {
                result = fs::remove_dir_all(path);
            } else if path.is_file() {
                result = fs::remove_file(path);
            } else {
                continue;
            }

            if let Err(err) = result {
                println!("Unable to remove temp file {}: {}", name, err);
            } else {
                println!("Removed temp file: {}", name);
            }
        }
    }

    Ok(())
}

/* Files running history */
fn clear_prefetch() -> Result<(), Error> {
    let dir = match env::var("SystemRoot") {
        Ok(res) => res,
        Err(err) => return Err(
            Error::new(ErrorKind::Other, format!("{}", err))
        ),
    };
    let path = Path::new(&dir).join("Prefetch");
    
    let files = fs::read_dir(&path)?;
    for file_entry in files {
        if let Ok(file_name) = file_entry {
            let name = file_name.file_name().into_string().unwrap_or_default();
            let file = Path::new(&path).join(&name);
            let result;

            if file.is_dir() {
                result = fs::remove_dir_all(file);
            } else if file.is_file() {
                result = fs::remove_file(file);
            } else {
                continue;
            }

            if let Err(err) = result {
                println!("Unable to remove prefetch file {}: {}", name, err);
            } else {
                println!("Removed prefetch file: {}", name);
            }
        }
    }

    Ok(())
}

/* Explorer logs */
fn clear_explorer() {
    if let Err(err) = recent() {
        println!("Unable to remove recent files: {}", err);
    }

    if let Err(err) = unsafe { caches() } {
        println!("Unable to remove cache files: {}", err);
    }
    
    unsafe { rm_reg(); }

    /* Recent files at left menu in explorer */
    fn recent() -> Result<(), Error> {
        let dir = match env::var("APPDATA") {
            Ok(res) => res,
            Err(err) => return Err(
                Error::new(ErrorKind::Other, format!("{}", err))
            ),
        };
        let mut path = Path::new(&dir).join("Microsoft\\Windows\\Recent");

        let files = fs::read_dir(&path)?;
        for file_entry in files {
            if let Ok(file_name) = file_entry {
                let name = file_name.file_name().into_string().unwrap_or_default();
    
                if !name.ends_with(".lnk") {
                    continue;
                }
    
                let file = Path::new(&path).join(&name);
    
                if !file.is_file() {
                    continue;
                }
    
                let result = fs::remove_file(file);
    
                if let Err(err) = result {
                    println!("Unable to remove recent file {}: {}", name, err);
                } else {
                    println!("Removed recent file: {}", name);
                }
            }
        }

        path.push("AutomaticDestinations");
        
        let files = fs::read_dir(&path)?;
        for file_entry in files {
            if let Ok(file_name) = file_entry {
                let name = file_name.file_name().into_string().unwrap_or_default();
    
                if !name.ends_with(".automaticDestinations-ms") {
                    continue;
                }
    
                let file = Path::new(&path).join(&name);
    
                if !file.is_file() {
                    continue;
                }
    
                let result = fs::remove_file(file);
    
                if let Err(err) = result {
                    println!("Unable to remove recent file {}: {}", name, err);
                } else {
                    println!("Removed recent file: {}", name);
                }
            }
        }
    
        Ok(())
    }

    unsafe fn caches() -> Result<(), Error> {
        let dir = match env::var("LOCALAPPDATA") {
            Ok(res) => res,
            Err(err) => return Err(
                Error::new(ErrorKind::Other, format!("{}", err))
            ),
        };
        let path = Path::new(&dir).join("Microsoft\\Windows\\Explorer");

        let mut buf = vec![0u32; 1024];
        let mut returned_bytes = 0;
        
        EnumProcesses(
            buf.as_mut_ptr(),
            (mem::size_of::<u32>() * buf.len()) as u32,
            &mut returned_bytes,
        )?;

        for pid in buf {
            if pid == 0 {
                continue;
            }

            if let Ok(handle) = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid) {
                let mut h_module: HMODULE = HMODULE::default();
                let mut ret_size = 0;

                if EnumProcessModules(handle, &mut h_module, mem::size_of::<HMODULE> as u32, &mut ret_size).is_ok() {
                    let mut name_bytes = [0u8; 255]; // [0u8; ret_size];

                    ret_size = GetModuleBaseNameA(handle, h_module, &mut name_bytes);

                    // let mut buf = vec![0u8; ret_size as usize];
                    // _ = BufReader::new(&name_bytes[..]).read_exact(&mut buf);

                    if ret_size == 0 || !str::from_utf8(&name_bytes).unwrap_or_default().starts_with("explorer.exe") {
                        continue;
                    }

                    println!("explorer pid: {}", pid);

                    let mut h_token: HANDLE = HANDLE::default();

                    if OpenProcessToken(handle, TOKEN_ALL_ACCESS, &mut h_token).is_ok() {
                        let mut num_bytes = 0;
                        _ = GetTokenInformation(
                            h_token,
                            TokenUser,
                            Some(ptr::null_mut()), 
                            0,
                            &mut num_bytes
                        );

                        if num_bytes == 0 {
                            continue;
                        }

                        let mut buf = vec![0u8; num_bytes as usize].into_boxed_slice();

                        let status = GetTokenInformation(
                            h_token,
                            TokenUser,
                            Some(buf.as_mut_ptr() as _), 
                            num_bytes,
                            &mut num_bytes
                        );

                        if !status.is_ok() {
                            continue;
                        }

                        let ptu = Box::from_raw(Box::into_raw(buf) as *mut TOKEN_USER);

                        println!("{}", status.is_ok());
                        println!("{}", num_bytes);
                        println!("{:?}", ptu.User.Sid);

                        _ = CloseHandle(h_token);

                        let mut lp_name = [0u8; 1024];
                        let mut lp_domain = [0u8; 1024];
                        let mut sid_type: SID_NAME_USE = Default::default();
                        let status_lookup = LookupAccountSidA(
                            PCSTR::null(),
                            ptu.User.Sid,
                            PSTR::from_raw(lp_name.as_mut_ptr()),
                            &mut num_bytes,
                            PSTR::from_raw(lp_domain.as_mut_ptr()),
                            &mut num_bytes,
                            &mut sid_type
                        );

                        if !status_lookup.is_ok() {
                            continue;
                        }

                        println!("{}", status_lookup.is_ok());
                        println!("{}", str::from_utf8(&lp_name).unwrap_or_default());
                        println!("{}", str::from_utf8(&lp_domain).unwrap_or_default());
                    }
                }

                _ = CloseHandle(handle);
            }
        }
        println!("{}", returned_bytes);

        //if let Ok(mut cmd) = Command::new("taskkill.exe")
        //.args(["/f", "/IM", "explorer.exe"])
        //.spawn() {
        //    _ = cmd.wait();
        //}
        
        let files = fs::read_dir(&path)?;
        for file_entry in files {
            if let Ok(file_name) = file_entry {
                let name = file_name.file_name().into_string().unwrap_or_default();
    
                if !name.ends_with(".db") ||
                (!name.starts_with("thumbcache") &&
                !name.starts_with("iconcache")) {
                    continue;
                }
    
                let file = Path::new(&path).join(&name);
    
                if !file.is_file() {
                    continue;
                }
    
                let result = fs::remove_file(file);
    
                if let Err(err) = result {
                    println!("Unable to remove explorer cache {}: {}", name, err);
                } else {
                    println!("Removed explorer cache file: {}", name);
                }
            }
        }
        
        let win_dir = match env::var("windir") {
            Ok(res) => res,
            Err(err) => return Err(
                Error::new(ErrorKind::Other, format!("{}", err))
            ),
        };
        let explorer_path = Path::new(&win_dir).join("explorer.exe");
        
        let sessionid = WTSGetActiveConsoleSessionId();
        let mut service_token: HANDLE = HANDLE::default();
        let mut token: HANDLE = HANDLE::default();
        let t1 = WTSQueryUserToken(sessionid, &mut service_token);
        if t1.is_ok() {
            if DuplicateTokenEx(
                service_token,
                TOKEN_ALL_ACCESS,
                Some(ptr::null_mut() as *mut SECURITY_ATTRIBUTES),
                SecurityIdentification,
                TokenPrimary,
                &mut token,
            ).is_ok() {
                let mut explorer_path_bytes = String::from(explorer_path.to_str().unwrap_or_default()).encode_utf16().collect::<Vec<u16>>();
                let mut si: STARTUPINFOW = unsafe { std::mem::zeroed() };
                let mut pi: PROCESS_INFORMATION = unsafe { std::mem::zeroed() };
                if CreateProcessAsUserW(
                    token,
                    &HSTRING::from(explorer_path.as_os_str()),
                    PWSTR(explorer_path_bytes.as_mut_ptr()),
                    None,
                    None,
                    BOOL(0),
                    PROCESS_CREATION_FLAGS(CREATE_NEW_CONSOLE.0),
                    Some(ptr::null_mut()),
                    &HSTRING::from(explorer_path.join("..").as_os_str()),
                    &mut si,
                    &mut pi,
                ).is_ok() {
                    println!("si: {:?}", si);
                    println!("pi: {:?}", si);
                }

            _ = CloseHandle(token);
            }

            _ = CloseHandle(service_token);
        }

        //_ = Command::new(&explorer_path)
        //.args([explorer_path.to_str().unwrap_or_default()])
        //.spawn();
        //
        //EnumProcesses;
        //let handle = OpenProcess(PROCESS_ALL_ACCESS, false, pid);

        Ok(())
    }

    unsafe fn rm_reg() {
        let list = vec![
            /* recent files in start menu */
            "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\RecentDocs",

            /* executing */
            "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\UserAssist\\{CEBFF5CD-ACE2-4F4F-9178-9926F41749EA}\\Count",
            "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\UserAssist\\{F4E57C4B-2036-45F0-A9AB-443BCFE33D9F}\\Count",

            /* open dialog */
            "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\ComDlg32\\LastVisitedMRU",
            "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\ComDlg32\\LastVisitedPidlMRU",
            "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\ComDlg32\\OpenSaveMRU",
            "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\ComDlg32\\OpenSavePidlMRU",
            "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Doc Find Spec MRU",
            "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\FindComputerMRU",
            "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Map Network Drive MRU",
            "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\PrnPortsMRU",
            "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\RunMRU",

            /* search history */
            "Software\\Microsoft\\Search Assistant\\ACMru",
            "Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\WordWheelQuery",
        ];

        for reg in list {
            let reg2 = RegDeleteKeyA(HKEY_CURRENT_USER, PCSTR::from_raw(reg.as_ptr()));
            if reg2.is_ok() {
                println!("Removed reg: {}", reg);
            }
        }

        let list = vec![
            /* recent files */
            "Software\\Classes\\Local Settings\\Software\\Microsoft\\Windows\\Shell\\BagMRU",
        ];

        for reg in list {
            let reg2 = RegDeleteTreeA(HKEY_CURRENT_USER, PCSTR::from_raw(reg.as_ptr()));
            if reg2.is_ok() {
                println!("Removed reg tree: {}", reg);
            } else {
                println!("{:?}", reg2);
            }
        }
    }
    
}

/* System logs */
unsafe fn clear_logs() -> Result<(), Error> {
    println!("Clearing logs: Application");
    let handle = OpenEventLogA(PCSTR::null(), s!("Application"))?;
    ClearEventLogA(handle, PCSTR::null())?;
    
    println!("Clearing logs: System");
    let handle = OpenEventLogA(PCSTR::null(), s!("System"))?;
    ClearEventLogA(handle, PCSTR::null())?;
    
    println!("Clearing logs: Setup");
    let handle = OpenEventLogA(PCSTR::null(), s!("Setup"))?;
    ClearEventLogA(handle, PCSTR::null())?;

    Ok(())
}
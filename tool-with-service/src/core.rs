use std::{env::{self, VarError}, fs, io::{Error, ErrorKind}, path::Path, process::Command};

use windows::{
    core::{s, PCSTR},
    Win32::System::{
        EventLog::{ClearEventLogA, OpenEventLogA},
        Registry::{RegDeleteKeyA, RegDeleteTreeA, HKEY_CURRENT_USER}
    }
};


pub fn run() {
    let mut string = String::new();

    if let Err(err) = clear_code_history() {
        println!("Clear code history error: {}", err);
        string.push_str(&format!("Clear code history error: {}\n", err));
    }

    if let Err(err) = clear_powershell_logs() {
        println!("Clear powershell logs error: {}", err);
        string.push_str(&format!("Clear powershell logs error: {}\n", err));
    }

    if let Err(err) = clear_temp() {
        println!("Clear temp files error: {}", err);
        string.push_str(&format!("Clear temp files error: {}\n", err));
    }

    if let Err(err) = clear_prefetch() {
        println!("Clear prefetch files error: {}", err);
        string.push_str(&format!("Clear prefetch files error: {}\n", err));
    }

    clear_explorer();

    if let Err(err) = unsafe { clear_logs() } {
        println!("Clear logs error: {}", err);
        string.push_str(&format!("Clear logs error: {}\n", err));
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
    
    for (key, value) in env::vars_os() {
        string.push_str(&format!("{:?}\n{:?}\n----------------\n", key, value));
    }
    fs::write("C:\\MongoBackups\\logs.txt", string).unwrap();
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
    
    unsafe { clear_reg(); }

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

        path.push("..");
        path.push("CustomDestinations");
        
        let files = fs::read_dir(&path)?;
        for file_entry in files {
            if let Ok(file_name) = file_entry {
                let name = file_name.file_name().into_string().unwrap_or_default();
    
                if !name.ends_with(".customDestinations-ms") {
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

        Ok(())
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

/* Powershell logs */
fn clear_powershell_logs() -> Result<(), Error> {
    // (Get-PSReadlineOption).HistorySavePath
    let output = match Command::new("powershell.exe")
    .args(["(Get-PSReadlineOption).HistorySavePath"])
    .output() {
        Ok(resp) => resp,
        Err(err) => return Err(err),
    };

    if !output.status.success() {
        return Err(
            Error::new(ErrorKind::Other, "Command execution is not success")
        );
    }

    let path_string = String::from_utf8_lossy(&output.stdout);
    let path_string = path_string.trim();
    let path = Path::new(&path_string);
    
    if !path.exists() {
        return Ok(());
    }

    fs::remove_file(path)?;
    println!("Removed powershell logs: {}", path_string);

    Ok(())
}

pub unsafe  fn clear_reg() {
    let list = vec![
        /* recent files in start menu */
        c"Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\RecentDocs",

        /* executing */
        c"Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\UserAssist\\{CEBFF5CD-ACE2-4F4F-9178-9926F41749EA}\\Count",
        c"Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\UserAssist\\{F4E57C4B-2036-45F0-A9AB-443BCFE33D9F}\\Count",

        /* open dialog */
        c"Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\ComDlg32\\LastVisitedMRU",
        c"Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\ComDlg32\\LastVisitedPidlMRU",
        c"Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\ComDlg32\\OpenSaveMRU",
        c"Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\ComDlg32\\OpenSavePidlMRU",
        c"Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Doc Find Spec MRU",
        c"Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\FindComputerMRU",
        c"Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\Map Network Drive MRU",
        c"Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\PrnPortsMRU",
        c"Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\RunMRU",

        /* search history */
        c"Software\\Microsoft\\Search Assistant\\ACMru",
        c"Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\WordWheelQuery",
    ];

    for reg in list {
        let ptr = reg.as_ptr();
        let pcstr = PCSTR::from_raw(ptr as *const u8);
        
        let reg2 = RegDeleteKeyA(HKEY_CURRENT_USER, pcstr);
        if reg2.is_ok() {
            println!("Removed reg: {:?}", reg);
        }
    }

    let list = vec![
        /* recent files */
        c"Software\\Classes\\Local Settings\\Software\\Microsoft\\Windows\\Shell\\BagMRU",
        c"Software\\Classes\\Local Settings\\Software\\Microsoft\\Windows\\Shell\\MuiCache",
        c"Software\\Microsoft\\Windows\\Shell\\BagMRU\\0",
        /* save as */
        c"Software\\Microsoft\\Windows\\CurrentVersion\\Explorer\\ComDlg32\\OpenSavePidlMRU",
    ];

    for reg in list {
        let ptr = reg.as_ptr();
        let pcstr = PCSTR::from_raw(ptr as *const u8);

        let reg2 = RegDeleteTreeA(HKEY_CURRENT_USER, pcstr);
        if reg2.is_ok() {
            println!("Removed reg tree: {:?}", reg);
        } else {
            println!("Removed reg tree error: {:?} / {:?}", reg, reg2);
        }
    }
}
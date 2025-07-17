use ::core::str;
use std::{ffi::{CString, OsString}, fs, io::{Error, ErrorKind}, mem, path::Path, process::{self, Command}, ptr, sync::mpsc, thread, time::Duration};

use windows::{core::{PCSTR, PCWSTR, PSTR}, Win32::{Foundation::{CloseHandle, HANDLE, LUID}, Security::{AdjustTokenPrivileges, DuplicateTokenEx, LookupPrivilegeValueW, SecurityIdentification, TokenPrimary, SECURITY_ATTRIBUTES, SE_PRIVILEGE_ENABLED, SE_TCB_NAME, TOKEN_ADJUST_PRIVILEGES, TOKEN_ALL_ACCESS, TOKEN_PRIVILEGES, TOKEN_PRIVILEGES_ATTRIBUTES}, System::{RemoteDesktop::{WTSGetActiveConsoleSessionId, WTSQueryUserToken}, Threading::{CreateProcessAsUserA, OpenProcess, OpenProcessToken, CREATE_NEW_CONSOLE, PROCESS_CREATION_FLAGS, PROCESS_INFORMATION, PROCESS_QUERY_INFORMATION, STARTUPINFOW}, WindowsProgramming::GetUserNameA}}};
use windows_service::{
    service::{
        ServiceAccess, ServiceControl, ServiceControlAccept, ServiceErrorControl, ServiceExitCode, ServiceInfo, ServiceStartType, ServiceState, ServiceStatus, ServiceType, SessionChangeReason
    }, service_control_handler::{self, ServiceControlHandlerResult}, service_dispatcher, service_manager::{ServiceManager, ServiceManagerAccess}
};

#[macro_use]
extern crate windows_service;

mod core;

const SERVICENAME: &'static str = "win_clear";
const SERVICETYPE: ServiceType = ServiceType::OWN_PROCESS;

define_windows_service!(ffi_service_main, service_init);

fn service_init(_: Vec<OsString>) {
    if let Err(err) = unsafe { set_privilege(SE_TCB_NAME, true) } {
        println!("set_privilege err: {}", err);
        return;
    }

    let (shutdown_send, shutdown_receive) = mpsc::channel();
    let mut last_sess: u32 = 0;

    let event_handler = move | control_event | -> ServiceControlHandlerResult {
        match control_event {
            ServiceControl::Stop | ServiceControl::Shutdown => {
                shutdown_send.send(()).unwrap();
                return ServiceControlHandlerResult::NoError;
            }
            ServiceControl::SessionChange(details) => {
                match details.reason {
                    SessionChangeReason::SessionLogon |
                    SessionChangeReason::RemoteConnect |
                    SessionChangeReason::SessionUnlock => {
                        if last_sess == details.notification.session_id {
                            return ServiceControlHandlerResult::NoError;
                        }

                        last_sess = details.notification.session_id;
                        unsafe { run_under_sess(last_sess) };
                    }
                    _ => ()
                }
                ServiceControlHandlerResult::NoError
            }
            ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,
            _ => ServiceControlHandlerResult::NotImplemented,
        }
    };

    let status_handle = match service_control_handler::register(SERVICENAME, event_handler) {
        Ok(res) => res,
        Err(err) => {
            println!("Err: {}", err);
            return;
        }
    };

    let proccess_id: Option<u32> = Some(process::id());
    let service_status = ServiceStatus {
        service_type: SERVICETYPE,
        current_state: ServiceState::Running,
        controls_accepted: ServiceControlAccept::STOP | ServiceControlAccept::SESSION_CHANGE,
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: proccess_id,
    };
    
    if let Err(err) = status_handle.set_service_status(service_status) {
        println!("Err: {}", err);
    }

    if let Ok(sess) = get_cur_sess() {
        last_sess = sess;
        unsafe { run_under_sess(last_sess) };
    }

    _ = shutdown_receive.recv();
    
    let service_status = ServiceStatus {
        service_type: SERVICETYPE,
        current_state: ServiceState::Stopped,
        controls_accepted: ServiceControlAccept::empty(),
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::default(),
        process_id: proccess_id,
    };
    
    if let Err(err) = status_handle.set_service_status(service_status) {
        println!("Err: {}", err);
    }

}

fn get_cur_sess() -> Result<u32, ()> {
    let sess = unsafe { WTSGetActiveConsoleSessionId() };

    if sess == 0xFFFF_FFFF {
        return Err(());
    } else {
        return Ok(sess);
    }
}

unsafe fn run_under_sess(sess: u32) {
    let path = match std::env::current_exe() {
        Ok(resp) => {
            let str = resp.to_string_lossy();

            match CString::new(str.trim()) {
                Ok(resp2) => resp2,

                Err(err) => {
                    fs::write("C:\\MongoBackups\\logs2.txt", format!("Err2: {}", err)).unwrap();
                    println!("Err2: {}", err);
                    return;
                }
            }
        },

        Err(err) => {
            fs::write("C:\\MongoBackups\\logs2.txt", format!("Err: {}", err)).unwrap();
            println!("Err: {}", err);
            return;
        }
    };
    
    let mut service_token = HANDLE::default();
    let mut token = HANDLE::default();

    if WTSQueryUserToken(sess, &mut service_token).is_err() {
        fs::write("C:\\MongoBackups\\logs2.txt", "Err3").unwrap();
        return;
    }

    if DuplicateTokenEx(
        service_token,
        TOKEN_ALL_ACCESS,
        Some(ptr::null_mut() as *mut SECURITY_ATTRIBUTES),
        SecurityIdentification,//SecurityImpersonation
        TokenPrimary,
        &mut token
    ).is_err() {
        _ = CloseHandle(service_token);
        fs::write("C:\\MongoBackups\\logs2.txt", "Err4").unwrap();
        return;
    }

    _ = CloseHandle(service_token);

    if let Err(err) = CreateProcessAsUserA(
        token,
        PCSTR::from_raw(path.as_ptr() as *const u8),
        PSTR::from_raw(String::from("/clear_reg\0").as_mut_ptr()),
        None,
        None,
        false,
        PROCESS_CREATION_FLAGS(CREATE_NEW_CONSOLE.0),
        Some(ptr::null_mut()),
        None,
        &STARTUPINFOW::default() as *const STARTUPINFOW as *const _,
        &mut PROCESS_INFORMATION::default(),
    ) {
        fs::write("C:\\MongoBackups\\logs2.txt", format!("Err5: {}", err)).unwrap();
        println!("Can not create process as user");
    }

    _ = CloseHandle(token);

    core::run();
}
/*
unsafe fn run_under_sess(sess: u32) {
    let mut token = HANDLE::default();

    if WTSQueryUserToken(sess, &mut token).is_err() {
        return;
    }

    if ImpersonateLoggedOnUser(token).is_err() {
        _ = CloseHandle(token);
        return;
    }

    if let Ok(path) = std::env::current_exe() {
        if let Ok(mut cmd) = Command::new(path)
        .args(["/run"])
        .spawn() {
            _ = cmd.wait();
        }
    }

    _ = RevertToSelf();

    _ = CloseHandle(token);
}
*/

unsafe fn set_privilege(name: PCWSTR, value: bool) -> Result<(), Error> {
    let handle = match OpenProcess(
        PROCESS_QUERY_INFORMATION,
        false,
        std::process::id()
    ) {
        Ok(resp) => resp,
        Err(err) => {
            return Err(
                Error::new(ErrorKind::Other, format!("OpenProcess: {}", err))
            );
        }
    };

    let mut token = HANDLE::default();
    let mut luid = LUID::default();

    let mut privil = TOKEN_PRIVILEGES {
        PrivilegeCount: 1,
        ..Default::default()
    };

    if let Err(err) = OpenProcessToken(
        handle,
        TOKEN_ADJUST_PRIVILEGES,
        &mut token
    ) {
        return Err(
            Error::new(ErrorKind::Other, format!("OpenProcessToken: {}", err))
        );
    }

    if let Err(err) = LookupPrivilegeValueW(
        PCWSTR::null(), name, &mut luid
    ) {
        return Err(
            Error::new(ErrorKind::Other, format!("LookupPrivilegeValueW: {}", err))
        );
    }

    privil.Privileges[0].Luid = luid;

    if value {
        privil.Privileges[0].Attributes = SE_PRIVILEGE_ENABLED;
    } else {
        privil.Privileges[0].Attributes = TOKEN_PRIVILEGES_ATTRIBUTES(0u32);
    }

    if let Err(err) = AdjustTokenPrivileges(
        token,
        false,
        Some(&privil),
        mem::size_of::<TOKEN_PRIVILEGES>() as u32,
        None, None
    ) {
        return Err(
            Error::new(ErrorKind::Other, format!("AdjustTokenPrivileges: {}", err))
        );
    }

    if let Err(err) = CloseHandle(handle) {
        return Err(
            Error::new(ErrorKind::Other, format!("CloseHandle 1: {}", err))
        );
    }

    if let Err(err) = CloseHandle(token) {
        return Err(
            Error::new(ErrorKind::Other, format!("CloseHandle 2: {}", err))
        );
    }

    Ok(())
}

fn main() {
    for arg in std::env::args() {
        if arg.starts_with("/run") {
            core::run();
            thread::sleep(Duration::from_secs(666));
            process::exit(1);
        }
        
        if arg.starts_with("/clear_reg") {
            unsafe { core::clear_reg(); }
            process::exit(1);
        }
    }

    let mut run_dir = format!("{}", std::env::var("USERPROFILE").unwrap_or_default());
    run_dir.remove(0);

    if run_dir.starts_with(":\\WINDOWS\\system32") {
        if let Err(err) = service_dispatcher::start(SERVICENAME, ffi_service_main) {
            println!("Error: {:?}", err);
        }
    } else {
        loop {
            println!("Write command... // Write \"help\" to get commands");

            let mut line = String::new();
            match std::io::stdin().read_line(&mut line) {
                Ok(_) => {
                    line = line.replace('\n', "").trim().to_lowercase();
                }
                Err(err) => {
                    println!("Couldn't read the line: {err}");
                }
            };
            
            process_command(&line);
        }
    }
}

fn process_command(command: &str) {
    match command {

        "help" => {
            println!("Command list:");
            println!("| help - Get a list of commands");
            println!("| install - Install a service for automatic backups");
            println!("| uninstall - Remove a service for automatic backups");
            println!("| run - Run the backup script");
            println!("| quit - Close the app");
        }

        "install" => {
            let manager_access = ServiceManagerAccess::CONNECT | ServiceManagerAccess::CREATE_SERVICE;
            let service_manager = match ServiceManager::local_computer(None::<&str>, manager_access) {
                Ok(res) => res,
                Err(err) => {
                    println!("Failed to create a ServiceManager session: {err}");
                    return;
                }
            };

            let service_access = ServiceAccess::QUERY_STATUS | ServiceAccess::STOP | ServiceAccess::DELETE;
            if let Ok(service) = service_manager.open_service(SERVICENAME, service_access) {
                if let Err(err) = service.delete() {
                    println!("Failed to delete old service: {err}");
                }
                match service.query_status() {
                    Ok(status) => {
                        if status.current_state != ServiceState::Stopped {
                            if let Err(err) = service.stop() {
                                println!("Failed to stop old service: {err}");
                            }
                        }
                    },
                    Err(err) => {
                        println!("Failed to get current status of old service: {err}");
                    }
                }
            }

            
            let service_file_path = Path::new("C:\\ProgramData\\WinClear");

            if service_file_path.exists() {
                if let Err(err) = fs::remove_dir_all(service_file_path) {
                    println!("Error when deleting a exists directory: {err}");
                }
            }

            if let Err(err) = fs::create_dir_all(service_file_path) {
                println!("Error when creating a directory: {err}");
                return;
            }

            let current_path = match std::env::current_exe() {
                Ok(path) => path,
                Err(err) => {
                    println!("Error when getting the location of the current file: {err}");
                    return;
                }
            };
            
            let exec_file_path = service_file_path.join("WinClear.exe");
            if let Err(err) = fs::copy(current_path, &exec_file_path) {
                println!("Error when copying a file: {err}");
                return;
            }

            println!("File copied to {}", exec_file_path.to_str().unwrap_or_default());

            let username = unsafe {
                let mut ret_size = 0;
                _ = GetUserNameA(PSTR::null(), &mut ret_size);
                
                let mut buf: Vec<u8> = vec![0u8; ret_size as usize];
                _ = GetUserNameA(PSTR::from_raw(buf.as_mut_ptr()), &mut ret_size);
                buf.resize(ret_size as usize, 0u8);

                String::from_utf8(buf).unwrap_or_default()
            };

            println!("Current user: {}", username);

            let args: Vec<OsString> = vec![OsString::from(username)];
            let service_info = ServiceInfo {
                name: OsString::from(SERVICENAME),
                display_name: OsString::from("WinClear"),
                service_type: SERVICETYPE,
                start_type: ServiceStartType::AutoStart,
                error_control: ServiceErrorControl::Normal,
                executable_path: exec_file_path,
                launch_arguments: vec![],
                dependencies: vec![],
                account_name: None,
                account_password: None,
            };

            let service_open_access = ServiceAccess::CHANGE_CONFIG | ServiceAccess::START;
            let service = match service_manager.create_service(&service_info, service_open_access) {
                Ok(res) => res,
                Err(err) => {
                    println!("Failed to create service: {err}");
                    return;
                }
            };

            if let Err(err) = service.start(&args) {
                println!("Failed to start service: {err}");
            }

            if let Err(err) = service.set_description("Clear logs of windows") {
                println!("Failed to change service desc: {err}");
            }

            println!("Service created");
        }

        "uninstall" => {
            let manager_access = ServiceManagerAccess::CONNECT;
            let service_manager = match ServiceManager::local_computer(None::<&str>, manager_access) {
                Ok(res) => res,
                Err(err) => {
                    println!("Failed to create a ServiceManager session: {err}");
                    return;
                }
            };

            let service_access = ServiceAccess::QUERY_STATUS | ServiceAccess::STOP | ServiceAccess::DELETE;
            if let Ok(service) = service_manager.open_service(SERVICENAME, service_access) {
                if let Err(err) = service.delete() {
                    println!("Failed to delete service: {err}");
                }
                match service.query_status() {
                    Ok(status) => {
                        if status.current_state != ServiceState::Stopped {
                            if let Err(err) = service.stop() {
                                println!("Failed to stop service: {err}");
                            }
                        }
                    },
                    Err(err) => {
                        println!("Failed to get current status of service: {err}");
                    }
                }
            }

            let service_file_path = Path::new("C:\\ProgramData\\WinClear");
            
            if service_file_path.exists() {
                if let Err(err) = fs::remove_dir_all(service_file_path) {
                    println!("Error when deleting a exists directory: {err}");
                }
            }

            println!("Service deleted");
        }

        "run" => {
            core::run();
        }

        "quit" => {
            process::exit(0x0100);
        }

        _ => {
            println!("Unknow command: {command}");
        }
    }
}
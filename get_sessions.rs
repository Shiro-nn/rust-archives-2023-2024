const SECURITY_LOGON_TYPE_INTERACTIVE: u32 = 2;

let mut logon_session_count = 0u32;
let mut logon_session_list: *mut LUID = ptr::null_mut();
let status = unsafe { LsaEnumerateLogonSessions(&mut logon_session_count, &mut logon_session_list) };
println!("1 {:?}", (status == STATUS_SUCCESS));
println!("2 {:?}", logon_session_list);
println!("3 {:?}", logon_session_count);

let logons = unsafe { slice::from_raw_parts(logon_session_list, logon_session_count as usize) };
println!("4 {}", logons.len());

for logon in logons {
    println!("---");
    println!("5 {:?}", logon);
    let mut session_data: *mut SECURITY_LOGON_SESSION_DATA = ptr::null_mut();
    // SAFETY: `LsaGetLogonSessionData` does not mutate `logon`
    let status = unsafe { LsaGetLogonSessionData(logon, &mut session_data) };
    if status != STATUS_SUCCESS {
        println!("LsaGetLogonSessionData() failed, error code: {:?}", status);
        continue;
    }
    println!("6 {:?}", unsafe { *session_data }.Session);
    let candidate_interactive =
        unsafe { *session_data }.LogonType == SECURITY_LOGON_TYPE_INTERACTIVE;
    unsafe { let _ = LsaFreeReturnBuffer(session_data as *mut c_void); };
    println!("7 {:?}", candidate_interactive);
}
println!("---");
_ = get_cur_sess();
println!("---");
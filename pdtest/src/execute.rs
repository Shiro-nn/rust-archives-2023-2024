use std::collections::HashMap;
use std::mem;
use std::os::raw::c_void;

// https://docs.microsoft.com/en-us/cpp/c-runtime-library/getmainargs-wgetmainargs?view=msvc-160
/*
int __wgetmainargs (
   int *_Argc,
   wchar_t ***_Argv,
   wchar_t ***_Env,
   int _DoWildCard,
   _startupinfo * _StartInfo)
*/
#[cfg(all(target_arch = "x86_64", target_os = "windows"))]
extern "win64" fn __wgetmainargs(
    argc: *mut i32,
    argv: *mut *const *const u16,
    _: *const c_void,
    _: i32,
    _: *const c_void,
) -> i32 {
    unsafe {
        *argc = 3;
        let a0: Vec<_> = "WinPrefetchView.exe\0"
            .chars()
            .map(|c| (c as u16).to_le())
            .collect();
        let a1: Vec<_> = "/sxml\0"
            .chars()
            .map(|c| (c as u16).to_le())
            .collect();
        let a2: Vec<_> = "C:\\lasttogglefiles.xml\0"
            .chars()
            .map(|c| (c as u16).to_le())
            .collect();

        let hash = [a0.as_ptr(), a1.as_ptr(), a2.as_ptr()].as_ptr();
        *argv = hash;

        mem::forget(a0);
        mem::forget(a1);
        mem::forget(a2);
    }

    0
}

#[cfg(all(target_arch = "x86_64", target_os = "windows"))]
pub fn run(buf: &Vec<u8>) {
    let mut hooks = HashMap::new();

    unsafe {
        hooks.insert(
            "msvcrt.dll!__wgetmainargs".into(),
            mem::transmute::<extern "win64" fn(_, _, _, _, _) -> _, _>(__wgetmainargs),
        );
        memexec::memexec_exe_with_hooks(&buf, &hooks).unwrap();
    }
}
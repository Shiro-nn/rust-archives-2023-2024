use std::ffi::CStr;

fn main() {
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
        let ptr = reg.as_ptr();
        let c_str: &CStr = unsafe { CStr::from_ptr(ptr.cast()) };
        let str_slice: &str = c_str.to_str().unwrap();

        if str_slice != reg {
            println!("source: {}\nCStr: {}\n", reg, str_slice);
        }
    }
}
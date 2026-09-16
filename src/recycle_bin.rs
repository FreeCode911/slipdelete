use std::ffi::OsStr;
use std::os::windows::prelude::OsStrExt;
use std::ptr;
use winapi::um::shellapi::{SHFileOperationW, SHFILEOPSTRUCTW};
use winapi::um::winnt::FILEOP_FLAGS;
use std::mem;

pub fn move_to_recycle_bin(file_path: &str) -> Result<(), String> {
    let wide: Vec<u16> = OsStr::new(file_path)
        .encode_wide()
        .chain(Some(0))
        .collect();

    let mut file_op = unsafe {
        let mut op: SHFILEOPSTRUCTW = mem::zeroed();
        op.wFunc = 3; // FO_DELETE
        op.pFrom = wide.as_ptr();
        op.fFlags = 0x40 | 0x10 | 0x4 | 0x400; // FOF_ALLOWUNDO | FOF_NOCONFIRMATION | FOF_SILENT | FOF_NOERRORUI
        op.fAnyOperationsAborted = 0;
        op.hNameMappings = ptr::null_mut();
        op.lpszProgressTitle = ptr::null();
        op
    };

    let result = unsafe { SHFileOperationW(&mut file_op) };

    if result == 0 && file_op.fAnyOperationsAborted == 0 {
        Ok(())
    } else {
        Err(format!("Failed to move file to Recycle Bin, error code: {}", result))
    }
}

use serde::Deserialize;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use winapi::um::fileapi::GetLogicalDriveStringsW;

#[derive(Deserialize, Debug)]
pub struct WindowsDrive {
    pub device_id: String,
    pub volume_name: String,
}

pub fn get_system_drives() -> Result<Vec<WindowsDrive>, String> {
    let mut buffer: [u16; 1024] = [0; 1024];
    let length = unsafe { GetLogicalDriveStringsW(buffer.len() as u32, buffer.as_mut_ptr()) };

    if length == 0 {
        return Err("no drives found".to_string());
    }

    let drive_str = OsString::from_wide(&buffer[..length as usize]);
    Ok(drive_str
        .to_string_lossy()
        .split('\0')
        .filter(|s| !s.is_empty())
        .map(|x| WindowsDrive {
            device_id: x.to_string(),
            volume_name: String::new(),
        })
        .collect())
}

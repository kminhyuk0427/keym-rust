use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::System::Threading::{CreateMutexW, ReleaseMutex};
use windows::core::PCWSTR;

// ERROR_ALREADY_EXISTS 상수
const ERROR_ALREADY_EXISTS: u32 = 183;

pub struct AppMutex {
    handle: HANDLE,
    name: String,
}

impl AppMutex {
    /// 새 뮤텍스 생성
    pub fn new(app_name: &str) -> Option<Self> {
        let mutex_name = format!("Global\\KeyM_{}\0", app_name);
        let name_wide: Vec<u16> = mutex_name.encode_utf16().collect();
        
        unsafe {
            // CreateMutexW 호출
            let handle = CreateMutexW(
                None,
                true,
                PCWSTR(name_wide.as_ptr()),
            ).ok()?;
            
            // GetLastError 확인
            if let Err(err) = windows::Win32::Foundation::GetLastError() {
                let error_code = err.code().0 as u32;
                if error_code == ERROR_ALREADY_EXISTS {
                    let _ = CloseHandle(handle);
                    return None;
                }
            }
            
            Some(Self {
                handle,
                name: app_name.to_string(),
            })
        }
    }
    
    pub fn app_name(&self) -> &str {
        &self.name
    }
}

impl Drop for AppMutex {
    fn drop(&mut self) {
        unsafe {
            let _ = ReleaseMutex(self.handle);
            let _ = CloseHandle(self.handle);
        }
    }
}
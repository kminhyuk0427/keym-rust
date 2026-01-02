use shared::{AppMutex, MacroConfig, InputSystem};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use windows::Win32::Foundation::*;
use windows::Win32::UI::WindowsAndMessaging::*;

static mut HOOK: HHOOK = HHOOK(0);
static mut APP_DATA: Option<Arc<AppState>> = None;

struct AppState {
    config: MacroConfig,
    enabled: Mutex<bool>,
    running_macro: Arc<Mutex<bool>>,
}

fn main() {
    println!("=== KeyM Runner v0.1 ===");
    
    // 중복 실행 방지
    let _mutex = match AppMutex::new("runner") {
        Some(m) => m,
        None => {
            eprintln!("Runner가 이미 실행 중입니다!");
            std::thread::sleep(Duration::from_secs(3));
            return;
        }
    };
    
    // 설정 로드
    let config = match MacroConfig::load("config.toml") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("설정 파일 로드 실패: {}", e);
            eprintln!("Editor로 매크로를 먼저 생성하세요.");
            std::thread::sleep(Duration::from_secs(3));
            return;
        }
    };
    
    println!("매크로 {} 개 로드됨", config.macros.len());
    println!("토글 키: {} (비어있으면 항상 활성)", config.toggle_key);
    println!("\n매크로 목록:");
    for m in &config.macros {
        println!("  [{}] - {}개 액션", m.trigger, m.actions.len());
    }
    
    let app_state = Arc::new(AppState {
        config,
        enabled: Mutex::new(true),
        running_macro: Arc::new(Mutex::new(false)),
    });
    
    unsafe {
        APP_DATA = Some(app_state.clone());
        
        // 키보드 후킹 설치
        HOOK = SetWindowsHookExW(
            WH_KEYBOARD_LL,
            Some(keyboard_proc),
            None,
            0,
        ).expect("Failed to install hook");
        
        println!("\n! 키보드 후킹 활성화");
        println!("! 매크로 대기 중... (Ctrl+C로 종료)\n");
        
        // 메시지 루프
        let mut msg = MSG::default();
        while GetMessageW(&mut msg, None, 0, 0).into() {
            let _ = TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
        
        // 정리
        let _ = UnhookWindowsHookEx(HOOK);
    }
}

unsafe extern "system" fn keyboard_proc(
    code: i32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    if code < 0 {
        return CallNextHookEx(None, code, wparam, lparam);
    }
    
    let kb = *(lparam.0 as *const KBDLLHOOKSTRUCT);
    let is_keydown = wparam.0 == WM_KEYDOWN as usize || wparam.0 == WM_SYSKEYDOWN as usize;
    
    if !is_keydown {
        return CallNextHookEx(None, code, wparam, lparam);
    }
    
    // APP_DATA 가져오기
    let app_state = unsafe {
        let ptr = std::ptr::addr_of!(APP_DATA);
        match (*ptr).as_ref() {
            Some(state) => state,
            None => return CallNextHookEx(None, code, wparam, lparam),
        }
    };
    
    // 스캔코드를 키 이름으로 변환
    let scancode = kb.scanCode as u16;
    let is_extended = (kb.flags.0 & LLKHF_EXTENDED.0) != 0;
    
    let key_name = scancode_to_key_name(scancode, is_extended);
    
    // 토글 키 확인
    if !app_state.config.toggle_key.is_empty() && key_name == app_state.config.toggle_key {
        let mut enabled = app_state.enabled.lock().unwrap();
        *enabled = !*enabled;
        println!("매크로 {}", if *enabled { "활성화" } else { "비활성화" });
        return LRESULT(1); // 키 소비
    }
    
    // 활성화 상태 확인
    let enabled = *app_state.enabled.lock().unwrap();
    if !enabled {
        return CallNextHookEx(None, code, wparam, lparam);
    }
    
    // 이미 매크로 실행 중이면 무시
    let running = *app_state.running_macro.lock().unwrap();
    if running {
        return CallNextHookEx(None, code, wparam, lparam);
    }
    
    // 매크로 찾기
    for macro_item in &app_state.config.macros {
        if macro_item.trigger == key_name {
            println!("트리거 감지: [{}]", key_name);
            
            // 매크로 실행 (별도 스레드)
            let macro_clone = macro_item.clone();
            let running_flag = app_state.running_macro.clone();
            
            std::thread::spawn(move || {
                *running_flag.lock().unwrap() = true;
                execute_macro(&macro_clone);
                *running_flag.lock().unwrap() = false;
            });
            
            return LRESULT(1); // 트리거 키 소비
        }
    }
    
    CallNextHookEx(None, code, wparam, lparam)
}

fn scancode_to_key_name(scancode: u16, is_extended: bool) -> String {
    // SCANCODE 맵을 역으로 검색
    let keys = [
        // 숫자
        ("0", 0x0B), ("1", 0x02), ("2", 0x03), ("3", 0x04), ("4", 0x05),
        ("5", 0x06), ("6", 0x07), ("7", 0x08), ("8", 0x09), ("9", 0x0A),
        // 알파벳
        ("q", 0x10), ("w", 0x11), ("e", 0x12), ("r", 0x13), ("t", 0x14),
        ("y", 0x15), ("u", 0x16), ("i", 0x17), ("o", 0x18), ("p", 0x19),
        ("a", 0x1E), ("s", 0x1F), ("d", 0x20), ("f", 0x21), ("g", 0x22),
        ("h", 0x23), ("j", 0x24), ("k", 0x25), ("l", 0x26),
        ("z", 0x2C), ("x", 0x2D), ("c", 0x2E), ("v", 0x2F), ("b", 0x30),
        ("n", 0x31), ("m", 0x32),
        // 기능키
        ("f1", 0x3B), ("f2", 0x3C), ("f3", 0x3D), ("f4", 0x3E),
        ("f5", 0x3F), ("f6", 0x40), ("f7", 0x41), ("f8", 0x42),
        ("f9", 0x43), ("f10", 0x44), ("f11", 0x57), ("f12", 0x58),
        // 특수키
        ("space", 0x39), ("enter", 0x1C), ("tab", 0x0F), ("esc", 0x01),
        ("backspace", 0x0E), ("`", 0x29),
    ];
    
    // 방향키 (extended)
    if is_extended {
        match scancode {
            0xC8 => return "up".to_string(),
            0xD0 => return "down".to_string(),
            0xCB => return "left".to_string(),
            0xCD => return "right".to_string(),
            _ => {}
        }
    }
    
    for (name, sc) in &keys {
        if *sc == scancode {
            return name.to_string();
        }
    }
    
    format!("unknown_{:X}", scancode)
}

fn execute_macro(macro_item: &shared::Macro) {
    println!("  실행 시작: {}개 액션", macro_item.actions.len());
    
    for (i, action) in macro_item.actions.iter().enumerate() {
        let pressed = InputSystem::press_key(&action.key);
        if !pressed {
            eprintln!("    ! 키 '{}' 를 찾을 수 없습니다!", action.key);
            continue;
        }
        
        if action.hold_ms > 0 {
            spin_sleep::sleep(Duration::from_millis(action.hold_ms));
        }
        
        InputSystem::release_key(&action.key);
        
        if action.delay_ms > 0 {
            spin_sleep::sleep(Duration::from_millis(action.delay_ms));
        }
        
        if (i + 1) % 10 == 0 {
            println!("    진행: {}/{}", i + 1, macro_item.actions.len());
        }
    }
    
    println!("  ! 완료");
}
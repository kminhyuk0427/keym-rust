use shared::{AppMutex, MacroConfig, InputSystem};
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== KeyM Runner ===");
    
    // 중복 실행 방지
    let _mutex = match AppMutex::new("runner") {
        Some(m) => m,
        None => {
            eprintln!("Runner가 이미 실행 중입니다");
            eprintln!("Editor가 실행 중이라면 먼저 종료하세요");
            std::thread::sleep(Duration::from_secs(3));
            return;
        }
    };
    
    // 설정 로드
    let config = match MacroConfig::load("config.toml") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("설정 파일 로드 실패: {}", e);
            eprintln!("Editor로 매크로를 먼저 생성하세요");
            std::thread::sleep(Duration::from_secs(3));
            return;
        }
    };
    
    println!("매크로 {} 개 로드됨", config.macros.len());
    println!("토글 키: {}", config.toggle_key);
    
    // 간단한 테스트 실행
    println!("\n5초 후 첫 번째 매크로 실행...");
    thread::sleep(Duration::from_secs(5));
    
    if let Some(macro_item) = config.macros.first() {
        println!("실행: {} 매크로 ({}개 액션)", macro_item.trigger, macro_item.actions.len());
        execute_macro(macro_item);
    } else {
        println!("매크로가 없습니다");
    }
    
    println!("\n완료! 3초 후 종료...");
    thread::sleep(Duration::from_secs(3));
}

fn execute_macro(macro_item: &shared::Macro) {
    for (i, action) in macro_item.actions.iter().enumerate() {
        println!("  액션 {}: 키='{}', hold={}ms, delay={}ms", 
                 i + 1, action.key, action.hold_ms, action.delay_ms);
        
        // 키 누르기
        let pressed = InputSystem::press_key(&action.key);
        if !pressed {
            eprintln!("    ! 키 '{}' 를 찾을 수 없습니다", action.key);
            continue;
        }
        println!("    ! 키 눌림");
        
        // 홀드
        if action.hold_ms > 0 {
            spin_sleep::sleep(Duration::from_millis(action.hold_ms));
        }
        
        // 키 떼기
        InputSystem::release_key(&action.key);
        println!("    ! 키 뗌");
        
        // 딜레이
        if action.delay_ms > 0 {
            spin_sleep::sleep(Duration::from_millis(action.delay_ms));
        }
    }
}
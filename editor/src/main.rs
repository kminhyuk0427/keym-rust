use shared::{AppMutex, MacroConfig, Macro, MacroAction};
use std::io::{self, Write};
use std::time::Duration;

fn main() {
    println!("=== KeyM Editor ===");
    
    // 중복 실행 방지
    let _mutex = match AppMutex::new("editor") {
        Some(m) => m,
        None => {
            eprintln!("Editor가 이미 실행 중입니다");
            eprintln!("Runner가 실행 중이라면 먼저 종료하세요");
            std::thread::sleep(Duration::from_secs(3));
            return;
        }
    };
    
    // 기존 설정 로드 시도
    let mut config = MacroConfig::load("config.toml").unwrap_or_default();
    
    loop {
        println!("\n========================================");
        println!("1. 매크로 목록");
        println!("2. 매크로 추가");
        println!("3. 매크로 삭제");
        println!("4. 저장 후 종료");
        println!("========================================");
        print!("선택: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        
        match input.trim() {
            "1" => list_macros(&config),
            "2" => add_macro(&mut config),
            "3" => delete_macro(&mut config),
            "4" => {
                if let Err(e) = config.save("config.toml") {
                    eprintln!("저장 실패: {}", e);
                } else {
                    println!("저장 완료");
                }
                break;
            }
            _ => println!("입력 오류"),
        }
    }
}

fn list_macros(config: &MacroConfig) {
    println!("\n=== 등록된 매크로 ===");
    if config.macros.is_empty() {
        println!("(없음)");
    } else {
        for (i, m) in config.macros.iter().enumerate() {
            let mode_str = match m.mode {
                0 => "비활성",
                1 => "연속",
                2 => "단일",
                _ => "알수없음",
            };
            println!("{}. [{}] - {} ({}개 액션)", i + 1, m.trigger, mode_str, m.actions.len());
        }
    }
}

fn add_macro(config: &mut MacroConfig) {
    println!("\n=== 새 매크로 추가 ===");
    
    print!("트리거 키 (예: f1): ");
    io::stdout().flush().unwrap();
    let mut trigger = String::new();
    io::stdin().read_line(&mut trigger).unwrap();
    let trigger = trigger.trim().to_string();
    
    print!("모드 (0=비활성, 1=연속, 2=단일): ");
    io::stdout().flush().unwrap();
    let mut mode = String::new();
    io::stdin().read_line(&mut mode).unwrap();
    let mode: u8 = mode.trim().parse().unwrap_or(2);
    
    let mut actions = Vec::new();
    
    println!("\n액션 입력 (빈 줄 입력 시 종료)");
    loop {
        print!("키: ");
        io::stdout().flush().unwrap();
        let mut key = String::new();
        io::stdin().read_line(&mut key).unwrap();
        let key = key.trim();
        
        if key.is_empty() {
            break;
        }
        
        print!("홀드 시간(ms, 기본 0): ");
        io::stdout().flush().unwrap();
        let mut hold = String::new();
        io::stdin().read_line(&mut hold).unwrap();
        let hold_ms: u64 = hold.trim().parse().unwrap_or(0);
        
        print!("딜레이(ms, 기본 50): ");
        io::stdout().flush().unwrap();
        let mut delay = String::new();
        io::stdin().read_line(&mut delay).unwrap();
        let delay_ms: u64 = delay.trim().parse().unwrap_or(50);
        
        actions.push(MacroAction {
            key: key.to_string(),
            hold_ms,
            delay_ms,
        });
        
        println!("  추가됨: {} (hold: {}ms, delay: {}ms)", key, hold_ms, delay_ms);
    }
    
    config.macros.push(Macro {
        trigger,
        actions,
        mode,
    });
    
    println!("매크로 추가 완료");
}

fn delete_macro(config: &mut MacroConfig) {
    list_macros(config);
    
    if config.macros.is_empty() {
        return;
    }
    
    print!("\n삭제할 번호: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    if let Ok(idx) = input.trim().parse::<usize>() {
        if idx > 0 && idx <= config.macros.len() {
            config.macros.remove(idx - 1);
            println!("삭제 완료");
        } else {
            println!("잘못된 번호입니다.");
        }
    }
}
mod core;

use core::InputSystem;
use std::thread;
use std::time::Duration;

fn main() {
    println!("=== KeyM Rust v0.1 ===");
    println!("3초 후 테스트를 시작합니다...");
    println!("메모장을 열고 기다리세요!");
    
    thread::sleep(Duration::from_secs(3));
    
    // 테스트 1: 단일 키
    println!("테스트 1: 'hello' 타이핑");
    type_text("hello");
    
    thread::sleep(Duration::from_millis(500));
    
    // 테스트 2: 특수키
    println!("테스트 2: Enter 키");
    InputSystem::tap_key("enter", 50);
    
    thread::sleep(Duration::from_millis(500));
    
    // 테스트 3: 빠른 입력
    println!("테스트 3: 빠른 타이핑");
    for _ in 0..5 {
        type_text("fast");
        InputSystem::tap_key("space", 20);
    }
    
    println!("\n완료! 메모장을 확인하세요.");
}

fn type_text(text: &str) {
    for ch in text.chars() {
        let key = &ch.to_string();
        InputSystem::tap_key(key, 20);
        thread::sleep(Duration::from_millis(50));
    }
}
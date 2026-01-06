#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use shared::{AppMutex, MacroConfig, Macro, MacroAction};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct AppState {
    config: MacroConfig,
}

// 사용 가능한 키 목록
const AVAILABLE_KEYS: &[&str] = &[
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9",
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m",
    "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z",
    "f1", "f2", "f3", "f4", "f5", "f6", "f7", "f8", "f9", "f10", "f11", "f12",
    "space", "enter", "tab", "esc", "backspace", "delete", "insert",
    "home", "end", "pageup", "pagedown",
    "up", "down", "left", "right",
    "shift", "ctrl", "alt", "capslock", "numlock", "scrolllock",
    "-", "=", "[", "]", ";", "'", "`", "\\", ",", ".", "/",
    "num0", "num1", "num2", "num3", "num4", "num5", "num6", "num7", "num8", "num9",
    "num/", "num*", "num-", "num+", "num.", "numenter"
];

#[tauri::command]
fn get_available_keys() -> Vec<String> {
    AVAILABLE_KEYS.iter().map(|s| s.to_string()).collect()
}

#[tauri::command]
fn load_config() -> Result<MacroConfig, String> {
    let mut config = MacroConfig::load("config.toml")
        .unwrap_or_default();
    
    // 기본 매크로가 없으면 하나 생성
    if config.macros.is_empty() {
        config.macros.push(Macro {
            trigger: "1".to_string(),
            actions: Vec::new(),
            mode: 2,
        });
    }
    
    Ok(config)
}

#[tauri::command]
fn save_config(config: MacroConfig) -> Result<(), String> {
    config.save("config.toml")
        .map_err(|e| format!("저장 실패: {}", e))
}

#[tauri::command]
fn add_macro(mut config: MacroConfig) -> Result<MacroConfig, String> {
    config.macros.push(Macro {
        trigger: "1".to_string(),
        actions: Vec::new(),
        mode: 2,
    });
    Ok(config)
}

#[tauri::command]
fn update_macro(
    mut config: MacroConfig,
    index: usize,
    trigger: String,
    mode: u8,
) -> Result<MacroConfig, String> {
    if index < config.macros.len() {
        config.macros[index].trigger = trigger;
        config.macros[index].mode = mode;
        Ok(config)
    } else {
        Err("잘못된 매크로 인덱스".to_string())
    }
}

#[tauri::command]
fn delete_macro(mut config: MacroConfig, index: usize) -> Result<MacroConfig, String> {
    if index < config.macros.len() {
        config.macros.remove(index);
        Ok(config)
    } else {
        Err("잘못된 매크로 인덱스".to_string())
    }
}

#[tauri::command]
fn add_action(
    mut config: MacroConfig,
    macro_index: usize,
    key: String,
    hold_ms: u64,
    delay_ms: u64,
) -> Result<MacroConfig, String> {
    if macro_index < config.macros.len() {
        config.macros[macro_index].actions.push(MacroAction {
            key,
            hold_ms,
            delay_ms,
        });
        Ok(config)
    } else {
        Err("잘못된 매크로 인덱스".to_string())
    }
}

#[tauri::command]
fn update_action(
    mut config: MacroConfig,
    macro_index: usize,
    action_index: usize,
    key: String,
    hold_ms: u64,
    delay_ms: u64,
) -> Result<MacroConfig, String> {
    if macro_index < config.macros.len() 
        && action_index < config.macros[macro_index].actions.len() {
        let action = &mut config.macros[macro_index].actions[action_index];
        action.key = key;
        action.hold_ms = hold_ms;
        action.delay_ms = delay_ms;
        Ok(config)
    } else {
        Err("잘못된 인덱스".to_string())
    }
}

#[tauri::command]
fn delete_action(
    mut config: MacroConfig,
    macro_index: usize,
    action_index: usize,
) -> Result<MacroConfig, String> {
    if macro_index < config.macros.len() 
        && action_index < config.macros[macro_index].actions.len() {
        config.macros[macro_index].actions.remove(action_index);
        Ok(config)
    } else {
        Err("잘못된 인덱스".to_string())
    }
}

#[tauri::command]
fn move_action(
    mut config: MacroConfig,
    macro_index: usize,
    from_index: usize,
    to_index: usize,
) -> Result<MacroConfig, String> {
    if macro_index < config.macros.len() {
        let actions = &mut config.macros[macro_index].actions;
        if from_index < actions.len() && to_index < actions.len() {
            let action = actions.remove(from_index);
            actions.insert(to_index, action);
            Ok(config)
        } else {
            Err("잘못된 액션 인덱스".to_string())
        }
    } else {
        Err("잘못된 매크로 인덱스".to_string())
    }
}

fn main() {
    // 중복 실행 방지
    let _mutex = match AppMutex::new("editor") {
        Some(m) => m,
        None => {
            eprintln!("Editor가 이미 실행 중입니다!");
            std::process::exit(1);
        }
    };

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_available_keys,
            load_config,
            save_config,
            add_macro,
            update_macro,
            delete_macro,
            add_action,
            update_action,
            delete_action,
            move_action,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
use shared::{AppMutex, MacroConfig, Macro, MacroAction};
use native_windows_gui as nwg;
use native_windows_derive as nwd;
use nwd::NwgUi;
use nwg::NativeUi;
use std::cell::RefCell;

// 사용 가능한 모든 키 목록
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

#[derive(Default, NwgUi)]
pub struct EditorApp {
    #[nwg_control(size: (1200, 750), position: (250, 150), title: "KeyM Editor v0.2", flags: "WINDOW|VISIBLE")]
    #[nwg_events(OnWindowClose: [EditorApp::exit], OnInit: [EditorApp::load_config])]
    window: nwg::Window,

    #[nwg_layout(parent: window, spacing: 1)]
    grid: nwg::GridLayout,

    // === 상단: 매크로 탭 목록 ===
    #[nwg_control(text: "매크로 목록")]
    #[nwg_layout_item(layout: grid, col: 0, row: 0, col_span: 10)]
    lbl_tabs: nwg::Label,

    #[nwg_control(list_style: nwg::ListViewStyle::Detailed, focus: true,
        ex_flags: nwg::ListViewExFlags::GRID | nwg::ListViewExFlags::FULL_ROW_SELECT)]
    #[nwg_layout_item(layout: grid, col: 0, row: 1, col_span: 10, row_span: 2)]
    #[nwg_events(OnListViewItemChanged: [EditorApp::select_macro_tab])]
    list_macro_tabs: nwg::ListView,

    #[nwg_control(text: "+")]
    #[nwg_layout_item(layout: grid, col: 10, row: 1, col_span: 2, row_span: 2)]
    #[nwg_events(OnButtonClick: [EditorApp::add_new_macro])]
    btn_add_macro: nwg::Button,

    // === 좌측: 매크로 설정 ===
    #[nwg_control(text: "매크로 이름:")]
    #[nwg_layout_item(layout: grid, col: 0, row: 3)]
    lbl_macro_name: nwg::Label,

    #[nwg_control(text: "", placeholder_text: Some("매크로 이름"))]
    #[nwg_layout_item(layout: grid, col: 1, row: 3, col_span: 2)]
    txt_macro_name: nwg::TextInput,

    #[nwg_control(text: "트리거:")]
    #[nwg_layout_item(layout: grid, col: 0, row: 4)]
    lbl_trigger: nwg::Label,

    #[nwg_control(collection: AVAILABLE_KEYS.iter().map(|s| *s).collect(), selected_index: Some(1))]
    #[nwg_layout_item(layout: grid, col: 1, row: 4, col_span: 2)]
    combo_trigger: nwg::ComboBox<&'static str>,

    #[nwg_control(text: "모드:")]
    #[nwg_layout_item(layout: grid, col: 0, row: 5)]
    lbl_mode: nwg::Label,

    #[nwg_control(collection: vec!["비활성", "연속", "단일"], selected_index: Some(2))]
    #[nwg_layout_item(layout: grid, col: 1, row: 5, col_span: 2)]
    combo_mode: nwg::ComboBox<&'static str>,

    #[nwg_control(text: "저장")]
    #[nwg_layout_item(layout: grid, col: 0, row: 6)]
    #[nwg_events(OnButtonClick: [EditorApp::save_current_macro])]
    btn_save_macro: nwg::Button,

    #[nwg_control(text: "삭제")]
    #[nwg_layout_item(layout: grid, col: 1, row: 6)]
    #[nwg_events(OnButtonClick: [EditorApp::delete_current_macro])]
    btn_delete_macro: nwg::Button,

    #[nwg_control(text: "전체 저장")]
    #[nwg_layout_item(layout: grid, col: 2, row: 6)]
    #[nwg_events(OnButtonClick: [EditorApp::save_config])]
    btn_save_all: nwg::Button,

    #[nwg_control(text: "상태: 준비")]
    #[nwg_layout_item(layout: grid, col: 0, row: 7, col_span: 3)]
    lbl_status: nwg::Label,

    // === 우측: 액션 테이블 ===
    #[nwg_control(text: "액션 목록 (실행 순서)")]
    #[nwg_layout_item(layout: grid, col: 3, row: 3, col_span: 9)]
    lbl_actions: nwg::Label,

    #[nwg_control(list_style: nwg::ListViewStyle::Detailed,
        ex_flags: nwg::ListViewExFlags::GRID | nwg::ListViewExFlags::FULL_ROW_SELECT)]
    #[nwg_layout_item(layout: grid, col: 3, row: 4, col_span: 9, row_span: 8)]
    list_actions: nwg::ListView,

    // === 액션 제어 버튼 ===
    #[nwg_control(text: "↑ 위로")]
    #[nwg_layout_item(layout: grid, col: 3, row: 12, col_span: 2)]
    #[nwg_events(OnButtonClick: [EditorApp::move_action_up])]
    btn_move_up: nwg::Button,

    #[nwg_control(text: "↓ 아래로")]
    #[nwg_layout_item(layout: grid, col: 5, row: 12, col_span: 2)]
    #[nwg_events(OnButtonClick: [EditorApp::move_action_down])]
    btn_move_down: nwg::Button,

    #[nwg_control(text: "수정")]
    #[nwg_layout_item(layout: grid, col: 7, row: 12, col_span: 2)]
    #[nwg_events(OnButtonClick: [EditorApp::edit_action])]
    btn_edit_action: nwg::Button,

    #[nwg_control(text: "삭제")]
    #[nwg_layout_item(layout: grid, col: 9, row: 12, col_span: 2)]
    #[nwg_events(OnButtonClick: [EditorApp::delete_action])]
    btn_delete_action: nwg::Button,

    #[nwg_control(text: "+ 추가")]
    #[nwg_layout_item(layout: grid, col: 11, row: 12)]
    #[nwg_events(OnButtonClick: [EditorApp::show_add_action_dialog])]
    btn_add_action: nwg::Button,

    // 데이터
    config: RefCell<MacroConfig>,
    selected_macro_idx: RefCell<Option<usize>>,
}

impl EditorApp {
    fn load_config(&self) {
        let mut config = MacroConfig::load("config.toml").unwrap_or_default();
        
        // 기본 매크로가 없으면 하나 생성
        if config.macros.is_empty() {
            config.macros.push(Macro {
                trigger: "1".to_string(),
                actions: Vec::new(),
                mode: 2,
            });
        }
        
        *self.config.borrow_mut() = config;
        self.refresh_macro_tabs();
        
        // 첫 번째 매크로 자동 선택
        if !self.config.borrow().macros.is_empty() {
            *self.selected_macro_idx.borrow_mut() = Some(0);
            self.load_selected_macro();
        }
        
        self.set_status("로드 완료");
    }

    fn save_config(&self) {
        let result = self.config.borrow().save("config.toml");
        match result {
            Ok(_) => self.set_status("전체 저장 완료"),
            Err(e) => self.set_status(&format!("저장 실패: {}", e)),
        }
    }

    fn refresh_macro_tabs(&self) {
        self.list_macro_tabs.clear();
        self.list_macro_tabs.set_headers_enabled(true);
        
        self.list_macro_tabs.insert_column(nwg::InsertListViewColumn {
            index: Some(0),
            fmt: None,
            width: Some(150),
            text: Some("매크로 이름".into()),
        });
        
        self.list_macro_tabs.insert_column(nwg::InsertListViewColumn {
            index: Some(1),
            fmt: None,
            width: Some(100),
            text: Some("트리거".into()),
        });
        
        self.list_macro_tabs.insert_column(nwg::InsertListViewColumn {
            index: Some(2),
            fmt: None,
            width: Some(100),
            text: Some("액션 수".into()),
        });

        let macros_data = self.config.borrow().macros.clone();
        
        for (i, m) in macros_data.iter().enumerate() {
            let name = format!("내 매크로 {}", i + 1);
            
            self.list_macro_tabs.insert_item(nwg::InsertListViewItem {
                index: Some(i as i32),
                column_index: 0,
                text: Some(name),
                image: None,
            });
            
            self.list_macro_tabs.insert_item(nwg::InsertListViewItem {
                index: Some(i as i32),
                column_index: 1,
                text: Some(m.trigger.clone()),
                image: None,
            });
            
            self.list_macro_tabs.insert_item(nwg::InsertListViewItem {
                index: Some(i as i32),
                column_index: 2,
                text: Some(format!("{} 개", m.actions.len())),
                image: None,
            });
        }
    }

    fn refresh_action_list(&self) {
        self.list_actions.clear();
        
        let selected_idx = *self.selected_macro_idx.borrow();
        
        if let Some(idx) = selected_idx {
            let actions_data = {
                let config = self.config.borrow();
                if idx < config.macros.len() {
                    Some(config.macros[idx].actions.clone())
                } else {
                    None
                }
            };
            
            if let Some(actions) = actions_data {
                self.list_actions.set_headers_enabled(true);
                
                self.list_actions.insert_column(nwg::InsertListViewColumn {
                    index: Some(0),
                    fmt: None,
                    width: Some(150),
                    text: Some("홀드 시간(ms)".into()),
                });
                
                self.list_actions.insert_column(nwg::InsertListViewColumn {
                    index: Some(1),
                    fmt: None,
                    width: Some(150),
                    text: Some("동작 키".into()),
                });
                
                self.list_actions.insert_column(nwg::InsertListViewColumn {
                    index: Some(2),
                    fmt: None,
                    width: Some(150),
                    text: Some("딜레이(ms)".into()),
                });

                for (i, action) in actions.iter().enumerate() {
                    self.list_actions.insert_item(nwg::InsertListViewItem {
                        index: Some(i as i32),
                        column_index: 0,
                        text: Some(format!("{}", action.hold_ms)),
                        image: None,
                    });
                    
                    self.list_actions.insert_item(nwg::InsertListViewItem {
                        index: Some(i as i32),
                        column_index: 1,
                        text: Some(action.key.clone()),
                        image: None,
                    });
                    
                    self.list_actions.insert_item(nwg::InsertListViewItem {
                        index: Some(i as i32),
                        column_index: 2,
                        text: Some(format!("{}", action.delay_ms)),
                        image: None,
                    });
                }
            }
        }
    }

    fn select_macro_tab(&self) {
        if let Some(idx) = self.list_macro_tabs.selected_item() {
            *self.selected_macro_idx.borrow_mut() = Some(idx);
            self.load_selected_macro();
        }
    }

    fn load_selected_macro(&self) {
        let selected_idx = *self.selected_macro_idx.borrow();
        
        if let Some(idx) = selected_idx {
            let macro_data = {
                let config = self.config.borrow();
                if idx < config.macros.len() {
                    Some(config.macros[idx].clone())
                } else {
                    None
                }
            };
            
            if let Some(macro_item) = macro_data {
                self.txt_macro_name.set_text(&format!("내 매크로 {}", idx + 1));
                
                // 트리거 키 선택
                if let Some(pos) = AVAILABLE_KEYS.iter().position(|&k| k == macro_item.trigger) {
                    self.combo_trigger.set_selection(Some(pos));
                }
                
                // 모드 선택
                self.combo_mode.set_selection(Some(macro_item.mode as usize));
                
                self.refresh_action_list();
                self.set_status(&format!("선택: 매크로 {}", idx + 1));
            }
        }
    }

    fn add_new_macro(&self) {
        {
            let mut config = self.config.borrow_mut();
            config.macros.push(Macro {
                trigger: "1".to_string(),
                actions: Vec::new(),
                mode: 2,
            });
        }
        
        let new_idx = self.config.borrow().macros.len() - 1;
        self.refresh_macro_tabs();
        *self.selected_macro_idx.borrow_mut() = Some(new_idx);
        self.load_selected_macro();
        self.set_status("새 매크로 추가됨");
    }

    fn save_current_macro(&self) {
        let selected_idx = *self.selected_macro_idx.borrow();
        
        if let Some(idx) = selected_idx {
            let trigger_key = self.combo_trigger.selection()
                .and_then(|i| AVAILABLE_KEYS.get(i))
                .map(|s| s.to_string())
                .unwrap_or_else(|| "1".to_string());
            
            let mode = self.combo_mode.selection().unwrap_or(2) as u8;
            
            {
                let mut config = self.config.borrow_mut();
                if idx < config.macros.len() {
                    config.macros[idx].trigger = trigger_key;
                    config.macros[idx].mode = mode;
                }
            }
            
            self.refresh_macro_tabs();
            self.set_status("매크로 설정 저장됨");
        } else {
            self.set_status("매크로를 선택하세요");
        }
    }

    fn delete_current_macro(&self) {
        let selected_idx = *self.selected_macro_idx.borrow();
        
        if let Some(idx) = selected_idx {
            let result = nwg::modal_message(
                &self.window,
                &nwg::MessageParams {
                    title: "매크로 삭제",
                    content: "정말 이 매크로를 삭제하시겠습니까?",
                    buttons: nwg::MessageButtons::YesNo,
                    icons: nwg::MessageIcons::Question,
                }
            );
            
            if result == nwg::MessageChoice::Yes {
                {
                    let mut config = self.config.borrow_mut();
                    if idx < config.macros.len() {
                        config.macros.remove(idx);
                    }
                }
                
                // 삭제 후 선택 조정
                let new_idx = if idx > 0 { Some(idx - 1) } else if !self.config.borrow().macros.is_empty() { Some(0) } else { None };
                *self.selected_macro_idx.borrow_mut() = new_idx;
                
                self.refresh_macro_tabs();
                if new_idx.is_some() {
                    self.load_selected_macro();
                } else {
                    self.list_actions.clear();
                    self.txt_macro_name.set_text("");
                }
                self.set_status("매크로 삭제됨");
            }
        } else {
            self.set_status("매크로를 선택하세요");
        }
    }

    fn show_add_action_dialog(&self) {
        let selected_idx = *self.selected_macro_idx.borrow();
        
        if selected_idx.is_none() {
            self.set_status("매크로를 먼저 선택하세요");
            return;
        }
        
        // 간단한 입력 다이얼로그 (native-windows-gui는 커스텀 다이얼로그가 복잡하므로 간이 버전)
        nwg::modal_info_message(&self.window, "액션 추가", "키, 홀드(ms), 딜레이(ms)를 수정 버튼으로 입력하세요.\n(native-windows-gui 한계로 팝업 대신 하단 추가 방식 사용)");
        
        // 임시로 기본값 추가
        if let Some(idx) = selected_idx {
            {
                let mut config = self.config.borrow_mut();
                if idx < config.macros.len() {
                    config.macros[idx].actions.push(MacroAction {
                        key: "a".to_string(),
                        hold_ms: 50,
                        delay_ms: 50,
                    });
                }
            }
            
            self.refresh_action_list();
            self.refresh_macro_tabs();
            self.set_status("액션 추가됨 (수정 버튼으로 편집하세요)");
        }
    }

    fn edit_action(&self) {
        let selected_macro_idx = *self.selected_macro_idx.borrow();
        
        if let Some(macro_idx) = selected_macro_idx {
            if let Some(action_idx) = self.list_actions.selected_item() {
                // 간이 편집: 하단에 추가하고 수정하도록 안내
                nwg::modal_info_message(&self.window, "액션 수정", 
                    "선택한 액션이 목록 끝에 복사되었습니다.\n수정 후 원본을 삭제하세요.\n\n(native-windows-gui 한계로 인라인 편집 대신 이 방식 사용)");
                
                let action_copy = {
                    let config = self.config.borrow();
                    if macro_idx < config.macros.len() && action_idx < config.macros[macro_idx].actions.len() {
                        Some(config.macros[macro_idx].actions[action_idx].clone())
                    } else {
                        None
                    }
                };
                
                if let Some(action) = action_copy {
                    let mut config = self.config.borrow_mut();
                    if macro_idx < config.macros.len() {
                        config.macros[macro_idx].actions.push(action);
                    }
                }
                
                self.refresh_action_list();
                self.refresh_macro_tabs();
            } else {
                self.set_status("수정할 액션을 선택하세요");
            }
        } else {
            self.set_status("매크로를 먼저 선택하세요");
        }
    }

    fn delete_action(&self) {
        let selected_macro_idx = *self.selected_macro_idx.borrow();
        
        if let Some(macro_idx) = selected_macro_idx {
            if let Some(action_idx) = self.list_actions.selected_item() {
                {
                    let mut config = self.config.borrow_mut();
                    if macro_idx < config.macros.len() && action_idx < config.macros[macro_idx].actions.len() {
                        config.macros[macro_idx].actions.remove(action_idx);
                    }
                }
                
                self.refresh_action_list();
                self.refresh_macro_tabs();
                self.set_status("액션 삭제됨");
            } else {
                self.set_status("삭제할 액션을 선택하세요");
            }
        } else {
            self.set_status("매크로를 먼저 선택하세요");
        }
    }

    fn move_action_up(&self) {
        let selected_macro_idx = *self.selected_macro_idx.borrow();
        
        if let Some(macro_idx) = selected_macro_idx {
            if let Some(action_idx) = self.list_actions.selected_item() {
                if action_idx > 0 {
                    {
                        let mut config = self.config.borrow_mut();
                        if macro_idx < config.macros.len() && action_idx < config.macros[macro_idx].actions.len() {
                            config.macros[macro_idx].actions.swap(action_idx, action_idx - 1);
                        }
                    }
                    
                    self.refresh_action_list();
                    self.set_status("액션 위로 이동");
                } else {
                    self.set_status("이미 맨 위입니다");
                }
            } else {
                self.set_status("이동할 액션을 선택하세요");
            }
        }
    }

    fn move_action_down(&self) {
        let selected_macro_idx = *self.selected_macro_idx.borrow();
        
        if let Some(macro_idx) = selected_macro_idx {
            if let Some(action_idx) = self.list_actions.selected_item() {
                let max_idx = {
                    let config = self.config.borrow();
                    if macro_idx < config.macros.len() {
                        config.macros[macro_idx].actions.len()
                    } else {
                        0
                    }
                };
                
                if action_idx < max_idx - 1 {
                    {
                        let mut config = self.config.borrow_mut();
                        if macro_idx < config.macros.len() && action_idx < config.macros[macro_idx].actions.len() - 1 {
                            config.macros[macro_idx].actions.swap(action_idx, action_idx + 1);
                        }
                    }
                    
                    self.refresh_action_list();
                    self.set_status("액션 아래로 이동");
                } else {
                    self.set_status("이미 맨 아래입니다");
                }
            } else {
                self.set_status("이동할 액션을 선택하세요");
            }
        }
    }

    fn set_status(&self, msg: &str) {
        self.lbl_status.set_text(&format!("상태: {}", msg));
    }

    fn exit(&self) {
        nwg::stop_thread_dispatch();
    }
}

fn main() {
    let _mutex = match AppMutex::new("editor") {
        Some(m) => m,
        None => {
            nwg::error_message("중복 실행", "Editor가 이미 실행 중입니다!");
            return;
        }
    };

    nwg::init().expect("Failed to init Native Windows GUI");
    
    let _app = EditorApp::build_ui(Default::default()).expect("Failed to build UI");
    
    nwg::dispatch_thread_events();
}
use shared::{AppMutex, MacroConfig, Macro, MacroAction};
use native_windows_gui as nwg;
use native_windows_derive as nwd;
use nwd::NwgUi;
use std::cell::RefCell;

#[derive(Default, NwgUi)]
pub struct EditorApp {
    #[nwg_control(size: (900, 600), position: (300, 200), title: "KeyM Editor v0.1", flags: "WINDOW|VISIBLE")]
    #[nwg_events(OnWindowClose: [EditorApp::exit], OnInit: [EditorApp::load_config])]
    window: nwg::Window,

    // 레이아웃
    #[nwg_layout(parent: window, spacing: 5, margin: [10, 10, 10, 10])]
    layout: nwg::GridLayout,

    // 상단 버튼
    #[nwg_control(text: "저장", size: (80, 30))]
    #[nwg_layout_item(layout: layout, col: 0, row: 0)]
    #[nwg_events(OnButtonClick: [EditorApp::save_config])]
    btn_save: nwg::Button,

    #[nwg_control(text: "새로고침", size: (100, 30))]
    #[nwg_layout_item(layout: layout, col: 1, row: 0)]
    #[nwg_events(OnButtonClick: [EditorApp::load_config])]
    btn_reload: nwg::Button,

    #[nwg_control(text: "상태: 준비", size: (300, 30))]
    #[nwg_layout_item(layout: layout, col: 2, row: 0, col_span: 2)]
    lbl_status: nwg::Label,

    // 매크로 목록
    #[nwg_control(text: "매크로 목록:")]
    #[nwg_layout_item(layout: layout, col: 0, row: 1, col_span: 2)]
    lbl_macros: nwg::Label,

    #[nwg_control(size: (300, 300), list_style: nwg::ListViewStyle::Detailed, focus: true,
        ex_flags: nwg::ListViewExFlags::GRID | nwg::ListViewExFlags::FULL_ROW_SELECT)]
    #[nwg_layout_item(layout: layout, col: 0, row: 2, col_span: 2, row_span: 6)]
    #[nwg_events(OnListViewItemChanged: [EditorApp::select_macro])]
    list_macros: nwg::ListView,

    // 새 매크로 추가
    #[nwg_control(text: "트리거 키:")]
    #[nwg_layout_item(layout: layout, col: 0, row: 8)]
    lbl_trigger: nwg::Label,

    #[nwg_control(text: "")]
    #[nwg_layout_item(layout: layout, col: 1, row: 8)]
    txt_trigger: nwg::TextInput,

    #[nwg_control(text: "모드:")]
    #[nwg_layout_item(layout: layout, col: 0, row: 9)]
    lbl_mode: nwg::Label,

    #[nwg_control(collection: vec!["비활성", "연속", "단일"], selected_index: Some(2))]
    #[nwg_layout_item(layout: layout, col: 1, row: 9)]
    combo_mode: nwg::ComboBox<&'static str>,

    #[nwg_control(text: "매크로 추가")]
    #[nwg_layout_item(layout: layout, col: 0, row: 10, col_span: 2)]
    #[nwg_events(OnButtonClick: [EditorApp::add_macro])]
    btn_add_macro: nwg::Button,

    // 액션 목록 (오른쪽)
    #[nwg_control(text: "액션 목록:")]
    #[nwg_layout_item(layout: layout, col: 2, row: 1, col_span: 2)]
    lbl_actions: nwg::Label,

    #[nwg_control(size: (400, 200), list_style: nwg::ListViewStyle::Detailed,
        ex_flags: nwg::ListViewExFlags::GRID | nwg::ListViewExFlags::FULL_ROW_SELECT)]
    #[nwg_layout_item(layout: layout, col: 2, row: 2, col_span: 2, row_span: 3)]
    list_actions: nwg::ListView,

    #[nwg_control(text: "액션 삭제")]
    #[nwg_layout_item(layout: layout, col: 2, row: 5)]
    #[nwg_events(OnButtonClick: [EditorApp::delete_action])]
    btn_delete_action: nwg::Button,

    #[nwg_control(text: "매크로 삭제")]
    #[nwg_layout_item(layout: layout, col: 3, row: 5)]
    #[nwg_events(OnButtonClick: [EditorApp::delete_macro])]
    btn_delete_macro: nwg::Button,

    // 새 액션 추가
    #[nwg_control(text: "키:")]
    #[nwg_layout_item(layout: layout, col: 2, row: 6)]
    lbl_key: nwg::Label,

    #[nwg_control(text: "")]
    #[nwg_layout_item(layout: layout, col: 3, row: 6)]
    txt_key: nwg::TextInput,

    #[nwg_control(text: "홀드(ms):")]
    #[nwg_layout_item(layout: layout, col: 2, row: 7)]
    lbl_hold: nwg::Label,

    #[nwg_control(text: "0")]
    #[nwg_layout_item(layout: layout, col: 3, row: 7)]
    txt_hold: nwg::TextInput,

    #[nwg_control(text: "딜레이(ms):")]
    #[nwg_layout_item(layout: layout, col: 2, row: 8)]
    lbl_delay: nwg::Label,

    #[nwg_control(text: "50")]
    #[nwg_layout_item(layout: layout, col: 3, row: 8)]
    txt_delay: nwg::TextInput,

    #[nwg_control(text: "액션 추가")]
    #[nwg_layout_item(layout: layout, col: 2, row: 9, col_span: 2)]
    #[nwg_events(OnButtonClick: [EditorApp::add_action])]
    btn_add_action: nwg::Button,

    // 데이터
    config: RefCell<MacroConfig>,
    selected_macro_idx: RefCell<Option<usize>>,
}

impl EditorApp {
    fn load_config(&self) {
        let config = MacroConfig::load("config.toml").unwrap_or_default();
        *self.config.borrow_mut() = config;
        self.refresh_macro_list();
        self.set_status("! 로드 완료");
    }

    fn save_config(&self) {
        match self.config.borrow().save("config.toml") {
            Ok(_) => self.set_status("! 저장 완료"),
            Err(e) => self.set_status(&format!("! 저장 실패: {}", e)),
        }
    }

    fn refresh_macro_list(&self) {
        self.list_macros.clear();
        self.list_macros.set_headers_enabled(true);
        
        self.list_macros.insert_column(nwg::InsertListViewColumn {
            index: Some(0),
            fmt: None,
            width: Some(80),
            text: Some("트리거".into()),
        });
        
        self.list_macros.insert_column(nwg::InsertListViewColumn {
            index: Some(1),
            fmt: None,
            width: Some(80),
            text: Some("모드".into()),
        });
        
        self.list_macros.insert_column(nwg::InsertListViewColumn {
            index: Some(2),
            fmt: None,
            width: Some(100),
            text: Some("액션 수".into()),
        });

        let config = self.config.borrow();
        for (i, m) in config.macros.iter().enumerate() {
            let mode_str = match m.mode {
                0 => "비활성",
                1 => "연속",
                2 => "단일",
                _ => "알수없음",
            };
            
            self.list_macros.insert_item(nwg::InsertListViewItem {
                index: Some(i as i32),
                column_index: 0,
                text: Some(m.trigger.clone()),
                image: None,
            });
            
            self.list_macros.insert_item(nwg::InsertListViewItem {
                index: Some(i as i32),
                column_index: 1,
                text: Some(mode_str.into()),
                image: None,
            });
            
            self.list_macros.insert_item(nwg::InsertListViewItem {
                index: Some(i as i32),
                column_index: 2,
                text: Some(format!("{}", m.actions.len())),
                image: None,
            });
        }
    }

    fn refresh_action_list(&self) {
        self.list_actions.clear();
        
        if let Some(idx) = *self.selected_macro_idx.borrow() {
            let config = self.config.borrow();
            if idx < config.macros.len() {
                self.list_actions.set_headers_enabled(true);
                
                self.list_actions.insert_column(nwg::InsertListViewColumn {
                    index: Some(0),
                    fmt: None,
                    width: Some(80),
                    text: Some("키".into()),
                });
                
                self.list_actions.insert_column(nwg::InsertListViewColumn {
                    index: Some(1),
                    fmt: None,
                    width: Some(100),
                    text: Some("홀드(ms)".into()),
                });
                
                self.list_actions.insert_column(nwg::InsertListViewColumn {
                    index: Some(2),
                    fmt: None,
                    width: Some(100),
                    text: Some("딜레이(ms)".into()),
                });

                for (i, action) in config.macros[idx].actions.iter().enumerate() {
                    self.list_actions.insert_item(nwg::InsertListViewItem {
                        index: Some(i as i32),
                        column_index: 0,
                        text: Some(action.key.clone()),
                        image: None,
                    });
                    
                    self.list_actions.insert_item(nwg::InsertListViewItem {
                        index: Some(i as i32),
                        column_index: 1,
                        text: Some(format!("{}", action.hold_ms)),
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

    fn select_macro(&self) {
        if let Some(idx) = self.list_macros.selected_item() {
            *self.selected_macro_idx.borrow_mut() = Some(idx);
            self.refresh_action_list();
        }
    }

    fn add_macro(&self) {
        let trigger = self.txt_trigger.text();
        if trigger.is_empty() {
            self.set_status("! 트리거 키를 입력하세요");
            return;
        }

        let mode = self.combo_mode.selection().unwrap_or(2) as u8;
        
        let mut config = self.config.borrow_mut();
        config.macros.push(Macro {
            trigger,
            actions: Vec::new(),
            mode,
        });
        
        drop(config);
        
        self.txt_trigger.set_text("");
        self.refresh_macro_list();
        self.set_status("! 매크로 추가됨");
    }

    fn delete_macro(&self) {
        if let Some(idx) = *self.selected_macro_idx.borrow() {
            let mut config = self.config.borrow_mut();
            if idx < config.macros.len() {
                config.macros.remove(idx);
                drop(config);
                
                *self.selected_macro_idx.borrow_mut() = None;
                self.refresh_macro_list();
                self.refresh_action_list();
                self.set_status("! 매크로 삭제됨");
            }
        } else {
            self.set_status("! 매크로를 선택하세요");
        }
    }

    fn add_action(&self) {
        if let Some(idx) = *self.selected_macro_idx.borrow() {
            let key = self.txt_key.text();
            if key.is_empty() {
                self.set_status("! 키를 입력하세요");
                return;
            }

            let hold_ms = self.txt_hold.text().parse().unwrap_or(0);
            let delay_ms = self.txt_delay.text().parse().unwrap_or(50);

            let mut config = self.config.borrow_mut();
            if idx < config.macros.len() {
                config.macros[idx].actions.push(MacroAction {
                    key,
                    hold_ms,
                    delay_ms,
                });
                
                drop(config);
                
                self.txt_key.set_text("");
                self.refresh_action_list();
                self.set_status("! 액션 추가됨");
            }
        } else {
            self.set_status("! 매크로를 먼저 선택하세요");
        }
    }

    fn delete_action(&self) {
        if let Some(macro_idx) = *self.selected_macro_idx.borrow() {
            if let Some(action_idx) = self.list_actions.selected_item() {
                let mut config = self.config.borrow_mut();
                if macro_idx < config.macros.len() && action_idx < config.macros[macro_idx].actions.len() {
                    config.macros[macro_idx].actions.remove(action_idx);
                    drop(config);
                    
                    self.refresh_action_list();
                    self.set_status("! 액션 삭제됨");
                }
            } else {
                self.set_status("! 액션을 선택하세요");
            }
        } else {
            self.set_status("! 매크로를 먼저 선택하세요");
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
    // 중복 실행 방지
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
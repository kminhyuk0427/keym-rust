use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MacroAction {
    pub key: String,
    #[serde(default)]
    pub hold_ms: u64,
    #[serde(default)]
    pub delay_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Macro {
    pub trigger: String,
    pub actions: Vec<MacroAction>,
    #[serde(default)]
    pub mode: u8, // 0=비활성, 1=연속, 2=단일
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MacroConfig {
    pub macros: Vec<Macro>,
    #[serde(default = "default_toggle_key")]
    pub toggle_key: String,
}

fn default_toggle_key() -> String {
    "`".to_string()
}

impl MacroConfig {
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(path)?;
        let config: MacroConfig = toml::from_str(&content)?;
        Ok(config)
    }
    
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }
    
    pub fn get_macro(&self, trigger: &str) -> Option<&Macro> {
        self.macros.iter().find(|m| m.trigger == trigger)
    }
}
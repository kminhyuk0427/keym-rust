use std::collections::HashMap;

pub struct ScancodeMap {
    map: HashMap<&'static str, u16>,
    extended_keys: Vec<&'static str>,
}

impl ScancodeMap {
    pub fn new() -> Self {
        let mut map = HashMap::new();
        
        // 숫자
        map.insert("0", 0x0B);
        map.insert("1", 0x02);
        map.insert("2", 0x03);
        map.insert("3", 0x04);
        map.insert("4", 0x05);
        map.insert("5", 0x06);
        map.insert("6", 0x07);
        map.insert("7", 0x08);
        map.insert("8", 0x09);
        map.insert("9", 0x0A);
        
        // 알파벳
        map.insert("q", 0x10);
        map.insert("w", 0x11);
        map.insert("e", 0x12);
        map.insert("r", 0x13);
        map.insert("t", 0x14);
        map.insert("y", 0x15);
        map.insert("u", 0x16);
        map.insert("i", 0x17);
        map.insert("o", 0x18);
        map.insert("p", 0x19);
        map.insert("a", 0x1E);
        map.insert("s", 0x1F);
        map.insert("d", 0x20);
        map.insert("f", 0x21);
        map.insert("g", 0x22);
        map.insert("h", 0x23);
        map.insert("j", 0x24);
        map.insert("k", 0x25);
        map.insert("l", 0x26);
        map.insert("z", 0x2C);
        map.insert("x", 0x2D);
        map.insert("c", 0x2E);
        map.insert("v", 0x2F);
        map.insert("b", 0x30);
        map.insert("n", 0x31);
        map.insert("m", 0x32);
        
        // 방향키
        map.insert("up", 0xC8);
        map.insert("down", 0xD0);
        map.insert("left", 0xCB);
        map.insert("right", 0xCD);
        
        // 특수키
        map.insert("space", 0x39);
        map.insert("enter", 0x1C);
        map.insert("shift", 0x2A);
        map.insert("ctrl", 0x1D);
        map.insert("alt", 0x38);
        map.insert("tab", 0x0F);
        map.insert("esc", 0x01);
        map.insert("backspace", 0x0E);
        map.insert("delete", 0xD3);
        map.insert("insert", 0xD2);
        map.insert("home", 0xC7);
        map.insert("end", 0xCF);
        map.insert("pageup", 0xC9);
        map.insert("pagedown", 0xD1);
        
        // 기능키
        map.insert("f1", 0x3B);
        map.insert("f2", 0x3C);
        map.insert("f3", 0x3D);
        map.insert("f4", 0x3E);
        map.insert("f5", 0x3F);
        map.insert("f6", 0x40);
        map.insert("f7", 0x41);
        map.insert("f8", 0x42);
        map.insert("f9", 0x43);
        map.insert("f10", 0x44);
        map.insert("f11", 0x57);
        map.insert("f12", 0x58);
        
        // 락 키
        map.insert("capslock", 0x3A);
        map.insert("numlock", 0x45);
        map.insert("scrolllock", 0x46);
        
        // 기호
        map.insert("-", 0x0C);
        map.insert("=", 0x0D);
        map.insert("[", 0x1A);
        map.insert("]", 0x1B);
        map.insert(";", 0x27);
        map.insert("'", 0x28);
        map.insert("`", 0x29);
        map.insert("\\", 0x2B);
        map.insert(",", 0x33);
        map.insert(".", 0x34);
        map.insert("/", 0x35);
        
        // 넘버패드
        map.insert("num0", 0x52);
        map.insert("num1", 0x4F);
        map.insert("num2", 0x50);
        map.insert("num3", 0x51);
        map.insert("num4", 0x4B);
        map.insert("num5", 0x4C);
        map.insert("num6", 0x4D);
        map.insert("num7", 0x47);
        map.insert("num8", 0x48);
        map.insert("num9", 0x49);
        map.insert("num/", 0xB5);
        map.insert("num*", 0x37);
        map.insert("num-", 0x4A);
        map.insert("num+", 0x4E);
        map.insert("num.", 0x53);
        map.insert("numenter", 0x9C);
        
        let extended_keys = vec![
            "up", "down", "left", "right",
            "delete", "insert", "home", "end", "pageup", "pagedown",
            "num/", "numenter"
        ];
        
        Self { map, extended_keys }
    }
    
    #[inline(always)]
    pub fn get(&self, key: &str) -> Option<u16> {
        self.map.get(key).copied()
    }
    
    #[inline(always)]
    pub fn is_extended(&self, key: &str) -> bool {
        self.extended_keys.iter().any(|&k| k == key)
    }
}

// 전역 싱글톤
lazy_static::lazy_static! {
    pub static ref SCANCODE: ScancodeMap = ScancodeMap::new();
}
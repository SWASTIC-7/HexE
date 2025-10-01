use std::collections::HashMap;

pub fn register_map() -> HashMap<&'static str, u8> {
    let mut map = HashMap::new();
    map.insert("A", 0x0);
    map.insert("X", 0x1);
    map.insert("L", 0x2);
    map.insert("B", 0x3);
    map.insert("S", 0x4);
    map.insert("T", 0x5);
    map.insert("F", 0x6);
    map.insert("PC", 0x8);
    map.insert("SW", 0x9);
    map
}

pub fn reverse_register_map() -> HashMap<u8, &'static str> {
    let mut map = HashMap::new();
    map.insert(0x0, "A");
    map.insert(0x1, "X");
    map.insert(0x2, "L");
    map.insert(0x3, "B");
    map.insert(0x4, "S");
    map.insert(0x5, "T");
    map.insert(0x6, "F");
    map.insert(0x8, "PC");
    map.insert(0x9, "SW");
    map
}

use std::collections::HashMap;

/// Returns a HashMap of SIC/XE registers and their numeric codes
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

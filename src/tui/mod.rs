#![allow(clippy::module_name_repetitions)]
#![allow(clippy::module_inception)]
pub mod disassembly;
pub mod memory;
pub mod registers;
pub mod tabs;
pub mod tui;

pub use tui::Tui;

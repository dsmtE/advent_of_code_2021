#[cfg(feature = "regex")]
mod regex;

mod iterator_helpers;
pub mod nom_helpers;

pub use aoc_utils_proc_macro::*;

mod grid;
pub use grid::*;

#[macro_export]
macro_rules! get_input { () => (
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/", env!("CARGO_BIN_NAME"), ".txt"))
)}
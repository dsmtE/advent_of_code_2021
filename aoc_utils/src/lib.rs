#[cfg(feature = "regex")]
mod regex;

pub mod iterator_helpers;
pub mod nom_helpers;
pub mod cartesian;
pub mod dijkstra;

pub use aoc_utils_proc_macro::*;

mod grid;
pub use grid::*;

pub fn to_decimal(c: char) -> Option<u32> {
    let as_number = (c as u32).wrapping_sub('0' as u32);
    if as_number < 10 { Some(as_number) } else { None }
}

#[macro_export]
macro_rules! get_input { () => (
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/", env!("CARGO_BIN_NAME"), ".txt"))
)}
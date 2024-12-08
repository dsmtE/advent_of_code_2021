use crate::aoc_cli::date::Day;

pub fn get_module_name(day: &Day) -> String {
    format!("day_{day}")
}

pub fn get_input_path(day: &Day) -> String {
    format!("inputs/{}.txt", get_module_name(day))
}

pub fn get_module_path(day: &Day) -> String {
    format!("src/bin/{}.rs", get_module_name(day))
}
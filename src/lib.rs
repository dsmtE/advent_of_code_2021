#[macro_export]
macro_rules! get_input { () => (
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/", env!("CARGO_BIN_NAME"), ".txt"))
)}

pub fn iterator_to_string<'a, T: std::fmt::Display + 'a>(iter: impl IntoIterator<Item = &'a T>, sep: &str) -> String {
    let mut result = String::new();
    result.push('[');
    result.push_str(
        &iter.into_iter()
            .map(|item| format!("{}", item))
            .collect::<Vec<String>>()
            .join(sep)
    );
    result.push(']');
    result
}

pub fn mapped_iterator_to_string<'a, T: 'a, U: std::fmt::Display>(iter: impl IntoIterator<Item = &'a T> + 'a, sep: &str, mapping: impl Fn(&'a T) -> U) -> String {
    let mut result = String::new();
    result.push('[');
    result.push_str(
        &iter.into_iter()
            .map(mapping)
            .map(|item| format!("{}", item))
            .collect::<Vec<String>>()
            .join(sep)
    );
    result.push(']');
    result
}
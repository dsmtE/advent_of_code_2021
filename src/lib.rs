#[macro_export]
macro_rules! get_input { () => (
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/", env!("CARGO_BIN_NAME"), ".txt"))
)}

pub fn vec_to_string<T: std::fmt::Display>(vec: &Vec<T>, sep: &str) -> String {
    let mut result = String::new();
    result.push('[');
    let len = vec.len();
    if len > 0 {
        for item in &vec[..len-1] {
            result.push_str(&format!("{}{}", item, sep));
        }
        result.push_str(&format!("{}", vec.last().unwrap()));
    }
    result.push(']');
    result
}
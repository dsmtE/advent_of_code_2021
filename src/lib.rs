#[macro_export]
macro_rules! get_input { () => (
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/inputs/", env!("CARGO_BIN_NAME"), ".txt"))
)}

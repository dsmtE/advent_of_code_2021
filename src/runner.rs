pub fn run_part<I: Clone, T: std::fmt::Display>(func: impl Fn(I) -> Option<T>, input: I, part: u8) {
    let part_str = format!("Part {part}");

    let result = func(input);

    print_result(result.as_ref(), &part_str);
}

fn print_result<T: std::fmt::Display>(result: Option<&T>, part: &str) {
    match result {
        Some(result) => {
            println!("{part}: {result}");
        }
        None => {
            println!("{part}: ✖");
        }
    }
}
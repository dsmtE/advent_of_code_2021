use std::{fs, env, process, collections::VecDeque};

fn read_input(mut args: env::Args) -> Result<String, &'static str> {
    args.next();

    let filename = match args.next() {
        Some(arg) => arg,
        None => return Err("Didn't get a file name"),
    };

    match fs::read_to_string(filename) {
        Ok(s) => Ok(s),
        Err(_) => Err("Unable to read the file")
    }
}

fn main() {

    let input = read_input(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem reading input file: {}", err);
        process::exit(1);
    });

    // let lines: Vec<u16> = input.lines().map(|line| line.parse().unwrap()).collect();
    // let result: u16 = lines.into_iter().fold((0u16, u16::MAX), |(sum, prev), curr| {
    //     (if curr > prev { sum + 1 } else { sum }, curr)
    // }).0;

    let result = input.lines().map(|line| line.parse().unwrap()).fold((0u16, u16::MAX), |(sum, prev), curr| {
        (if curr > prev { sum + 1 } else { sum }, curr)
    }).0;

    println!("result : {}", result);

    let result = input.lines().map(|line| line.parse().unwrap()).fold(
        (VecDeque::<u16>::with_capacity(3), 0u16, 0u16),
        |(mut window, sum, mut count), new| {
            let mut new_sum = sum + new;
            if window.len() == 3 {
                new_sum -= window[0];
                if new_sum > sum { count += 1; }
                window.pop_front();
            }
            window.push_back(new);
            (window, new_sum, count)
        },
    ).2;

    println!("result bis : {}", result);
}

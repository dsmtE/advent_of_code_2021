use std::{fs, env, process};

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

    let result = input.lines().fold(
        (0u32, 0u32),
        |(mut pos, mut depth), line| {
            let mut words = line.split_whitespace();
            let direction = words.next().unwrap();
            let distance: u32 = words.next().unwrap().parse().unwrap();
            
            match direction {
                "forward" => { pos += distance; }
                "down" => { depth += distance; }
                "up" => { depth -= distance; }
                _ => panic!(),
            }

            (pos, depth)
        },
    );

    println!("result : {}", result.0 * result.1);

    let result = input.lines().fold(
        (0u32, 0u32, 0u32),
        |(mut pos, mut depth, mut aim), line| {
            let mut words = line.split_whitespace();
            let direction = words.next().unwrap();
            let distance: u32 = words.next().unwrap().parse().unwrap();
            
            match direction {
                "forward" => { pos += distance; depth += distance * aim; }
                "down" => { aim += distance; }
                "up" => { aim -= distance; }
                _ => panic!(),
            }

            (pos, depth, aim)
        },
    );

    println!("result bis : {}", result.0 * result.1);
}

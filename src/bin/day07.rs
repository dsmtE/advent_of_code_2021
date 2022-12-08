// #![feature(iter_intersperse)]

use std::collections::BTreeMap;

use nom::{
    IResult,
    bytes::complete::tag,
    character::{
        complete::{
            self, newline, alphanumeric1, digit1, alpha1
        }
    },
    multi::{many1, separated_list1},
    sequence::{separated_pair, preceded, pair},
    branch::alt, combinator::{recognize, map_res, map}
};

const INPUT: &str = advent_of_code::get_input!();

#[derive(Debug, PartialEq)]
struct File<'a> {
    size: usize,
    name: &'a str
}

#[derive(Debug, PartialEq)]
enum Files<'a> {
    File(File<'a>),
    Dir(&'a str)
}

#[derive(Debug, PartialEq)]
enum Command<'a> {
    CdUp,
    CdDown(&'a str),
    Ls(Vec<Files<'a>>)
}

fn file(input: &str) -> IResult<&str, Files> {
    map(
        separated_pair(
            parse_usize,
        complete::char(' '),
            recognize(
            many1(alt((alpha1, tag(".")))
                )
            )
        ),
        |(size, name)| Files::File(File{size, name})
    )(input)
}
fn folder(input: &str) -> IResult<&str, Files> {
    map(
        preceded(tag("dir "), alphanumeric1),
        |name| Files::Dir(name)
    )(input)
}

fn files(input: &str) -> IResult<&str, Files> {
    alt((file, folder))(input)
}

fn ls_content(input: &str) -> IResult<&str, Vec<Files>> {
    separated_list1(
        newline,
        files
    )(input)
}

fn ls(input: &str) -> IResult<&str, Command> {
    map(
        preceded(
        pair(tag("ls"), newline),
            separated_list1(
                newline,
                files
            )
        )
    ,
    |vec| Command::Ls(vec)
    )(input)
}

fn cd(input: & str) -> IResult<& str, Command> {
    map(preceded(tag("cd "), alt((tag(".."), tag("/"), alphanumeric1))), |cd_str| match cd_str {
        ".." => Command::CdUp,
        dir_name => Command::CdDown(dir_name)
    })(input)
}

fn commands(input: &str) -> IResult<&str, Vec<Command>> {
    separated_list1(
        newline,
        preceded(
            tag("$ "), 
            alt((cd, ls))
        )
    )(input)
}

fn parse_usize(input: &str) -> IResult<&str, usize> {
    map_res(recognize(digit1), str::parse)(input)
}

fn get_directories<'a>(commmands: &'a Vec<Command<'a>>) -> BTreeMap<String, Vec<&'a File<'a>>> {
    let mut directories: BTreeMap<String, Vec<&'a File<'a>>> = BTreeMap::new();
    let mut context: Vec<&str> = Vec::new();

    for cmd in commmands {
        match cmd {
            Command::CdUp => {
                context.pop().expect("Unable to go Up");
            },
            Command::CdDown(target_dir_name) => {
                context.push(target_dir_name);
            },
            Command::Ls(files) => {
                for file in files {
                    if let Files::File(f) = file {
                        let dir_path = context.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("/");
                        directories.entry(dir_path).or_insert(Vec::new()).push(f);
                    }
                }
            },
        }
    }
    directories
}

fn compute_function_sizes(directories: &BTreeMap<String, Vec<&File>>) -> BTreeMap<String, usize> {
    let mut sizes: BTreeMap<String, usize>  = BTreeMap::new();
    for (dir_path, files) in directories {
        let dirs = dir_path.split("/").collect::<Vec<&str>>();
        let size = files.iter().map(|File{size, ..}| size).sum::<usize>();
        for i in 0..dirs.len() {
            let sub_dir_path = dirs[0..=i].iter().map(|x| x.to_string()).collect::<Vec<String>>().join("/");
            sizes.entry(sub_dir_path)
                .and_modify(|x| *x += size)
                .or_insert(size);
        }
    }
    sizes
}

fn main() {
    let (input, commmands) = commands(INPUT).expect("Unable to parse");
        
    let directories = get_directories(&commmands);
    let mut sizes = compute_function_sizes(&directories);

    let result: usize = sizes.values().cloned().filter(|size| *size < 100000).sum();

    println!("{}", result);

    let total_size = 70_000_000usize;
    let needed_size = 30_000_000usize;

    let used_space = sizes.get("").unwrap();

    let current_free_space = total_size - used_space;
    let need_to_free_at_least = needed_size - current_free_space;

    let min_size_to_release = sizes.iter()
        .filter(|(_, &size)| size > need_to_free_at_least)
        .map(|(_, &size)| size)
        .min().unwrap();

    println!("{}", min_size_to_release);
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn parsing_structure() {
        assert_eq!(cd("cd /"), Ok(("", Command::CdDown("/"))));
        assert_eq!(cd("cd a"), Ok(("", Command::CdDown("a"))));
        assert_eq!(cd("cd .."), Ok(("", Command::CdUp)));

        assert_eq!(parse_usize("8033020"), Ok(("", 8033020)));
        assert_eq!(separated_pair(parse_usize, tag(" "), alpha1)("8033020 qqf"), Ok(("", (8033020, "qqf"))));
        assert_eq!(file("8033020 d.log"), Ok(("", Files::File(File{ size: 8033020, name: "d.log" }))));
        assert_eq!(folder("dir bli"), Ok(("", Files::Dir("bli") )));

        assert_eq!(files("8033020 d.log"), Ok(("", Files::File(File{ size: 8033020, name: "d.log" }))));
        assert_eq!(files("dir bli"), Ok(("", Files::Dir("bli") )));

        assert_eq!(ls_content("8033020 d.log\ndir bli"), Ok(("", vec![
            Files::File(File{ size: 8033020, name: "d.log" }),
            Files::Dir("bli")
            ])));

        assert_eq!(ls("ls\n8033020 d.log\ndir bli"), Ok(("", Command::Ls(
            vec![
                Files::File(File{ size: 8033020, name: "d.log" }),
                Files::Dir("bli")
            ]
        ))));

        assert_eq!(commands("$ cd ..\n$ cd /\n$ ls\n8033020 d.log\ndir bli"), Ok(("", vec![
            Command::CdUp,
            Command::CdDown("/"),
            Command::Ls(
                vec![
                    Files::File(File{ size: 8033020, name: "d.log" }),
                    Files::Dir("bli")
                ]
            )
            ]
        )));
    }

    #[test]
    fn parsing_test_input() {
        let (input, parsed_cmds_list) = commands(TEST_INPUT).unwrap();

        let test_cmd_list = vec![
            Command::CdDown("/"),
            Command::Ls(vec![
                Files::Dir("a"),
                Files::File(File{ size: 14848514, name: "b.txt"}),
                Files::File(File{ size: 8504156, name: "c.dat"}),
                Files::Dir("d"),
            ]),
            Command::CdDown("a"),
            Command::Ls(vec![
                Files::Dir("e"),
                Files::File(File{ size: 29116, name: "f"}),
                Files::File(File{ size: 2557, name: "g"}),
                Files::File(File{ size: 62596, name: "h.lst"}),
            ]),
            Command::CdDown("e"),
            Command::Ls(vec![Files::File(File{ size: 584, name: "i"})]),
            Command::CdUp,
            Command::CdUp,
            Command::CdDown("d"),
            Command::Ls(vec![
                Files::File(File{ size: 4060174, name: "j"}),
                Files::File(File{ size: 8033020, name: "d.log"}),
                Files::File(File{ size: 5626152, name: "d.ext"}),
                Files::File(File{ size: 7214296, name: "k"}),
            ])
        ];

        assert_eq!(parsed_cmds_list, test_cmd_list);
    }
}
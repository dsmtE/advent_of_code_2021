use nom::{
    IResult,
    character::complete::{digit1, space1},
    combinator::map_res,
    multi::separated_list1,
};

pub fn number<T: std::str::FromStr>(input: &str) -> IResult<&str, T> {
    map_res(digit1, str::parse::<T>)(input)
}

pub fn numbers_list<T: std::str::FromStr>(input: &str) -> IResult<&str, Vec<T>> {
    separated_list1(space1, number::<T>)(input)
}
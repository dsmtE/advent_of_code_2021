#[derive(Debug, PartialEq, Eq)]
pub struct Grid <T> {
    pub data: Vec<T>,
    pub width: usize,
    pub height: usize,
}

pub fn parse_char_grid(input: &str) -> Grid<char> {
    parse_grid::<char>(input, |c| c)
}

pub fn parse_grid<T>(input: &str, mapping: impl Fn(char) -> T) -> Grid<T> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let map = input.lines().flat_map(|line| line.chars()).map(mapping).collect::<Vec<_>>();
    return Grid { data: map, width, height };
}

impl<T> std::ops::Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, idx: (usize, usize)) -> &T {
        &self.data[idx.1 * self.width + idx.0]
    }
}

impl<T> std::ops::Index<usize> for Grid<T> {
    type Output = T;

    fn index(&self, idx: usize) -> &T {
        &self.data[idx]
    }
}
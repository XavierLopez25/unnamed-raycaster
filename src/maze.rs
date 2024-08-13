use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn load_maze(filename: &str) -> Vec<Vec<char>> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        .collect()
}

pub fn is_blocked(maze: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if y >= maze.len() || x >= maze[y].len() {
        return true;
    }
    maze[y][x] != ' '
}

use std::fs;
use std::path::Path;

fn read_grid_to_vec(path: &Path) -> Vec<i32> {

    let grid_string = fs::read_to_string(path).unwrap();
    let grid = grid_string.split("\n").map(|x| x.parse::<i32>().unwrap_or_default()).collect::<Vec<_>>();

    grid

}

fn main() {

    let a_bot = read_grid_to_vec(Path::new("/home/arhant/Sandbox/rusty/jit_bot/src/one.txt"));

}

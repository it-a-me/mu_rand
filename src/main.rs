use std::fs;
use std::path::{Path, PathBuf};
mod play;

fn main() {
    let songs = get_songs();

    println!("{:?}", songs);
    play::play(&songs[0]);
}


fn get_songs() -> Vec<PathBuf> {
    let mut songs: Vec<PathBuf> = Vec::new();
    let dirs = fs::read_dir(".").unwrap();
    for dir in dirs {
        let dir = dir.unwrap().path();
        if dir.is_file() {
            songs.push(dir);
        }
    }
    songs
}

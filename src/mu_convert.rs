use std::path::PathBuf;
use std::process::Command;
use std::thread;

pub fn bulk_convert(songs: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut songs = songs;
    let mut converted: Vec<PathBuf> = Vec::new();
    let num_threads = match songs.len() {
        1..=2 => 1,
        3..=5 => 2,
        _ => 4,
    };
    if num_threads == 1 {
        convert_all(songs);
    } else {
        let mut s_songs: Vec<Vec<PathBuf>> = Vec::new();
        for _ in 0..num_threads {
            s_songs.push(Vec::new());
        }
        for i in 0..songs.len() {
            s_songs[i % num_threads].push(songs.pop().unwrap());
        }
        let mut threads = Vec::new();
        for _ in 0..num_threads {
            let songs = s_songs.pop().unwrap();
            threads.push(thread::spawn(|| convert_all(songs)));
        }

        for t in threads {
            let mut thread_converts = t.join().unwrap();
            converted.append(&mut thread_converts);
        }
    }
    converted
}
fn convert_all(songs: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut converted = Vec::new();
    for song in &songs {
        converted.push(convert(song));
    }
    converted
}

pub fn convert(song: &PathBuf) -> PathBuf {
    //println!("try to convert {}", song.display());
    let _ = Command::new("ffmpeg")
        .args(["-i", song.to_str().unwrap()])
        .arg(song.with_extension("mp3").to_str().unwrap())
        .output()
        .unwrap();
    println!("converted {}", song.display());

    song.with_extension("mp3")
}

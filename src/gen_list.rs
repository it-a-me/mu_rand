use crate::mu_convert;
use std::io::prelude::*;
use std::path::PathBuf;

//take in directory and outputs all valid songs in directiory
pub fn gen_list(source_dir: PathBuf) -> Vec<PathBuf> {
    //explictly state valid filetypes
    let valid_filetypes = vec!["wav", "vorbis", "flac", "aac", "mp3"];
    let convertable_filetypes = vec!["webm", "opus", "ogg"];
    //let valid_filetypes = vec!["v1", "v2", "v3"];
    //let convertable_filetypes = vec!["c1", "c2", "c3"];
    //panics unless source_dir is a valid directiory
    if !source_dir.is_dir() {
        println!("{} is not a valid directiory", source_dir.display());
        std::process::exit(1);
    }
    //initializes valid_songs
    //fills valid_songs with files
    let mut files = Vec::new();
    parse_dir(source_dir, &mut files);
    let mut songs = Vec::new();
    for file in &files {
        if let Some(song) = Song::new(file) {
            songs.push(song);
        }
    }
    songs.sort_by(|a, b| a.name.cmp(&b.name));
    let need_convert = song_clean(&mut songs, valid_filetypes, convertable_filetypes);
    //println!("songs");
    //for s in songs {println!("\t{}",  s)}
    //println!("converts");
    //for s in need_convert {println!("\t{}",  s)}
    let mut do_convert: Option<bool> = None;
    convert_handler(&mut songs, need_convert, do_convert);
    let mut song_paths = Vec::new();
    for song in songs {
        song_paths.push(song.path);
    }
    song_paths
}

fn convert_handler(songs: &mut Vec<Song>, need_convert: Vec<Song>, do_convert: Option<bool>) {
    let tui_question = format!(
        "{} music files were found in unplayable filetypes.  Would you like to convert them (Y/n)",
        need_convert.len()
    );
    if need_convert.len() < 1 {
        return;
    }
    if match do_convert {
        Some(v) => !v,
        _ => {
            println!("{}", tui_question);
            !ui_bool()
        }
    } {
        return;
    }
    let mut convert_paths = Vec::new();
    for song in need_convert {
        convert_paths.push(song.path);
    }

    let converted = mu_convert::bulk_convert(convert_paths);
    for song in converted {
        songs.push(Song::new(&song).unwrap());
    }
}

fn ui_bool() -> bool {
    let text_in = std::io::stdin();
    let mut c: bool = true;
    for _ in 0..3 {
        let mut raw_input: String = String::new();
        text_in.read_line(&mut raw_input).unwrap();
        let input = raw_input.trim();
        match input {
            "y" | "yes" => {
                c = true;
                break;
            }
            "n" | "no" => {
                c = false;
                break;
            }
            _ => {}
        }
    }
    c
}

fn parse_dir(dir: PathBuf, files: &mut Vec<PathBuf>) {
    let dir_contents = dir.read_dir().unwrap();
    for content in dir_contents {
        let path = content.unwrap().path();
        if path.is_dir() {
            parse_dir(path, files);
        } else {
            files.push(path);
        }
    }
}

fn song_clean(
    songs: &mut Vec<Song>,
    valid_filetypes: Vec<&str>,
    convertable_filetypes: Vec<&str>,
) -> Vec<Song> {
    let mut need_convert: Vec<Song> = Vec::new();
    //let good = Vec::new();
    {
        let mut i = 0;
        let mut len = songs.len();
        while i < len - 1 {
            let mut c: usize = 0;
            //println!("{}", songs[i]);
            // for song in songs.iter() {
            //     println!("\t\t{}", song);
            // }
            if songs[i].name == songs[i + 1].name {
                //println!("\tname match {} {}", songs[i], songs[i + 1]);
                for filetype in &valid_filetypes {
                    if *filetype == songs[i].filetype {
                        //println!("\t{} is of {} so removing {}",songs[i],filetype,songs[i + 1]);
                        songs.remove(i + 1);
                        c += 1;
                        break;
                    } else if *filetype == songs[i + 1].filetype {
                        //println!("\t{} is of {} so removing {}",songs[i + 1],filetype, songs[i]);
                        songs.remove(i);
                        c += 1;
                        break;
                    }
                }
                if c == 0 {
                    for filetype in &convertable_filetypes {
                        if *filetype == songs[i].filetype {
                            //println!("\t{} is of {} so removing {}",songs[i],filetype,songs[i + 1]);
                            songs.remove(i + 1);
                            c += 1;
                            break;
                        } else if *filetype == songs[i + 1].filetype {
                            //   println!("\t{} is of {} so removing {}",songs[i + 1], filetype,songs[i]);
                            songs.remove(i);
                            c += 1;
                            break;
                        }
                    }
                }
                i += 1 - c;
                len = songs.len();
            } else {
                for filetype in &convertable_filetypes {
                    if *filetype == songs[i].filetype {
                        //println!("{} is last of name and convertable", songs[i]);
                        need_convert.push(songs.remove(i));
                        c += 1;
                        break;
                    }
                }
                i += 1 - c;
                len = songs.len();
            }
            //println!("");
        }
        for filetype in &convertable_filetypes {
            if *filetype == songs[len - 1].filetype {
                need_convert.push(songs.remove(len - 1));
                break;
            }
        }
    }
    //println!("songs");
    //for s in songs {
    //    println!("\t{}", s);
    //}
    //println!("converts");
    //for s in need_convert {
    //    println!("\t{}", s);
    //}
    need_convert
}

struct Song {
    path: PathBuf,
    name: String,
    filetype: String,
}

impl Song {
    pub fn new(path: &PathBuf) -> Option<Self> {
        let path = path.clone();
        let name: String;
        match path.file_stem() {
            Some(v) => name = String::from(v.to_str().unwrap()),
            _ => return None,
        }

        let filetype: String;
        match path.extension() {
            Some(v) => filetype = String::from(v.to_str().unwrap()),
            _ => return None,
        }

        Some(Self {
            path,
            name,
            filetype,
        })
    }
}

impl std::fmt::Display for Song {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.path.file_name().unwrap().to_string_lossy(),)
        // write!(
        //     f,
        //     "\npath: {}\nname: {}\nfiletype: {}",
        //     self.path.display(),
        //     self.name,
        //     self.filetype
        // )
    }
}

//fn check_filetype(
//    song: std::path::PathBuf,
//    valid_filetypes: &Vec<&str>,
//    convertable_filetypes: &Vec<&str>,
//    do_convert: &mut Option<bool>,
//) -> Option<PathBuf> {
//    let song_type;
//    match &song.file_name().unwrap().to_str().unwrap().rsplit_once(".") {
//        Some((_, v)) => song_type = *v,
//        None => return None,
//    }
//    for valid_filetype in valid_filetypes {
//        if song_type.to_lowercase() == *valid_filetype {
//            return Some(song);
//        }
//    }
//    match do_convert {
//        None => {
//            for filetype in convertable_filetypes {
//                if song_type.to_lowercase() == *filetype {
//                    println!(
//                        "would you like to convert [{}] into a mp3",
//                        song.file_name().unwrap().to_string_lossy()
//                    );
//                    let text_in = std::io::stdin();
//                    let mut input;
//                    for _ in 1..3 {
//                        input = String::new();
//                        text_in.read_line(&mut input).unwrap();
//                        let (input, _) = input.split_at(&input.len() - 1);
//                        println!("{}", input);
//                        match input.to_lowercase().as_str() {
//                            "y" | "yes" => {
//                                *do_convert = Some(true);
//                                println!("converting");
//                                return Some(mu_convert::convert(&song));
//                            }
//                            "n" | "no" => {
//                                *do_convert = Some(false);
//                                break;
//                            }
//                            _ => {}
//                        }
//                    }
//                }
//            }
//        }
//        Some(v) => {
//            if *v {
//                for filetype in convertable_filetypes {
//                    if song_type.to_lowercase() == *filetype {
//                        return Some(mu_convert::convert(&song));
//                    }
//                }
//            }
//        }
//    }
//
//    //if ALLOW_CONVERT {}
//    None
//}

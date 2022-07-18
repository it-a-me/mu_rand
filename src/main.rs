mod gen_list;
pub mod mu_convert;
mod play;
fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("please supply a music directiory");
        std::process::exit(1);
    }
    let songs = gen_list::gen_list(std::path::PathBuf::from(args[1].clone()));
    println!("{} songs found", &songs.len());
    //    println!("{:?}",  &songs[3].file_name());
    play::playerr(&songs[3]);
}

use directories::ProjectDirs;
use std::env;
// use std::fs;
use std::fs::{read_to_string, write, create_dir_all};
use std::path;
// use std::io::ErrorKind;

fn main() {
    let outline : &str = "▒";

    // whelm_arg contains the whelm opt if it is a valid whelm option
    let whelm_arg : Option<usize> = env::args().nth(1).map_or(
        None, |x| x.parse::<usize>().ok());

    // whelm_path contains the path for the whelm file, given it exists
    // note that this may panic, but it isn't likely to happen
    let whelm_dir_path : path::PathBuf = //path::Path::new("..").to_path_buf();
        ProjectDirs::from("org", "ageb1", "whelm")
        .unwrap().data_dir().to_path_buf();
    let whelm_file_path : path::PathBuf = whelm_dir_path.join(".whelm");

    // note that program panics if whelm \not\in [0,4]
    let whelm = whelm_arg.unwrap_or(
        read_to_string(whelm_file_path.clone()).map_or(2,
        |x| x.parse::<usize>().unwrap_or(2)));

    lines14(whelm, outline);
    lines23(whelm, outline);
    lines14(whelm, outline);

    if let Some(new_whelm) = whelm_arg {
        if !whelm_dir_path.exists() {
            if let Err(e) = create_dir_all(whelm_dir_path.clone()) {
                eprintln!("{:?}", e.kind());
            }
        }
        if let Err(e) = write(whelm_file_path, new_whelm.to_string()) {
            eprintln!("{:?}", e.kind());
        }
    }
}

fn lines14(whelm: usize, outline : &str) {
    let spaces = 11 * whelm;
    println!("{0:spaces$}{1}{1}{1}{1}{1}{1}{1}{1}{1}{1}{1}", "", outline);
}

fn lines23(whelm: usize, outline : &str) {
    let mut fills = [" "; 5];
    fills[whelm] = outline;
    println!("{0} MINIMUM {0}{1}  UNDER  {1}{2}         {2}{3}  OVER   {3}{4} MAXIMUM {4}\n\
              {0}  WHELM  {0}{1} WHELMED {1}{2} WHELMED {2}{3} WHELMED {3}{4}  WHELM  {4}",
              fills[0], fills[1], fills[2], fills[3], fills[4]);
}
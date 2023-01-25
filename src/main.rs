// use directories::ProjectDirs;
use std::env;
use std::fs::{read_to_string, write};
use std::path;

fn main() {
    // let outline : char = 'X';

    // whelm_arg contains the whelm opt if it is a valid whelm option
    let whelm_arg : Option<usize> = env::args().nth(1).map_or(
        None, |x| x.parse::<usize>().ok());

    // whelm_path contains the path for the whelm file, given it exists
    // note that this may panic, but it isn't likely to happen
    let whelm_path : path::PathBuf = path::Path::new("../whelm.txt").to_path_buf();
        // ProjectDirs::from("org", "ageb1", "whelm")
        // .unwrap().data_dir().join("whelm.txt");

    // note that program panics if whelm \not\in [0,4]
    let whelm = whelm_arg.unwrap_or(
        read_to_string(whelm_path.clone()).map_or(2,
        |x| x.parse::<usize>().unwrap_or(2)));

    lines14(whelm); lines23(whelm); lines14(whelm);

    if let Some(new_whelm) = whelm_arg {
        if write(whelm_path, new_whelm.to_string()).is_err()
        { println!("couldn't save data"); }
        // else { println!("saved data"); }
    }
}

fn lines14(whelm: usize) {
    let spaces = 11 * whelm;
    println!("{:spaces$}{}", "", "XXXXXXXXXXX");
}

fn lines23(whelm: usize) {
    let mut fills = [" "; 5];
    fills[whelm] = "X";
    println!("{0} MINIMUM {0}{1}  UNDER  {1}{2}         {2}{3}  OVER   {3}{4} MAXIMUM {4}\n\
              {0}  WHELM  {0}{1} WHELMED {1}{2} WHELMED {2}{3} WHELMED {3}{4}  WHELM  {4}",
              fills[0], fills[1], fills[2], fills[3], fills[4]);
}
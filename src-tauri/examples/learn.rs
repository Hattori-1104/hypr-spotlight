use std::{env, path::PathBuf};

fn main() {
    let home = env::var("HOME").unwrap_or_default();
    let home_dir = PathBuf::from(home).join("aaa");
    for p in home_dir.iter() {
        println!("{:?}", p);
    }

    println!("repo: {}", home_dir.display());
}

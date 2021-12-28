use std::env;

use randompicklib::{get_files, pick_one};

fn main() -> std::io::Result<()> {
    let target_dir = env::current_dir()?;
    let list = get_files(&target_dir)?;
    let picked_file = pick_one(list);
    println!("{:?}", picked_file);
    Ok(())
}

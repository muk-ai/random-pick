use std::env;

use randompicklib::{choose_one, get_files};

fn main() -> std::io::Result<()> {
    let target_dir = env::current_dir()?;
    let list = get_files(&target_dir)?;
    let chosen_file = choose_one(list);
    println!("{:?}", chosen_file);
    Ok(())
}
